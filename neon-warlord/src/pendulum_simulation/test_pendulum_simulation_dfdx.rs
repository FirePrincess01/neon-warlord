use dfdx::{optim::Adam, prelude::*};
use std::collections::VecDeque;

// Definition der Netzwerk-Architektur
type QNetwork = (
    Linear<3, 64>,
    ReLU,
    Linear<64, 64>,
    ReLU,
    Linear<64, 3>,
);

// Struktur für den Experience Replay Buffer
struct Transition {
    state: [f32; 3],
    action: usize,
    reward: f32,
    next_state: [f32; 3],
    done: bool,
}

// Minimalistische Pendel-Simulation
struct PendulumEnv {
    theta: f32,
    theta_dot: f32,
}

impl PendulumEnv {
    fn new() -> Self {
        Self {
            // fastrand f32() generiert Zahlen im Bereich [0.0, 1.0)
            theta: fastrand::f32() * 6.28 - 3.14, 
            theta_dot: 0.0,
        }
    }

    fn get_state(&self) -> [f32; 3] {
        [self.theta.cos(), self.theta.sin(), self.theta_dot]
    }

    fn step(&mut self, action_idx: usize) -> (f32, bool) {
        let u = match action_idx {
            0 => -2.0,
            1 => 0.0,
            _ => 2.0,
        };

        let g = 9.81;
        let m = 1.0;
        let l = 1.0;
        let dt = 0.05;

        let theta_acc = (3.0 * g / (2.0 * l)) * self.theta.sin() + (3.0 / (m * l * l)) * u;
        self.theta_dot += theta_acc * dt;
        self.theta = (self.theta + self.theta_dot * dt + std::f32::consts::PI)
            .rem_euclid(2.0 * std::f32::consts::PI) - std::f32::consts::PI;

        let reward = -(self.theta.powi(2) + 0.1 * self.theta_dot.powi(2) + 0.001 * u.powi(2));
        
        (reward, false)
    }
}

#[test]
#[ignore = "too expensive"]
fn main() {
    let dev = Cpu::default();

    let mut online_net = dev.build_module::<QNetwork, f32>();
    let mut target_net = online_net.clone();

    let mut opt = Adam::new(&online_net, AdamConfig {
        lr: 1e-3,
        ..Default::default()
    });

    let mut replay_buffer = VecDeque::with_capacity(10_000);
    let mut env = PendulumEnv::new();
    let mut epsilon = 1.0f32;
    let gamma = 0.99f32;
    let batch_size = 64;

    for step in 0..2000 {
        let state = env.get_state();
        
        // Epsilon-Greedy Action Selection mit fastrand
        let action = if fastrand::f32() < epsilon {
            fastrand::usize(0..3)
        } else {
            let state_tensor = dev.tensor(state);
            let q_values = online_net.forward(state_tensor);
            let q_arr = q_values.array();
            q_arr.iter().enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(idx, _)| idx)
                .unwrap()
        };

        let (reward, done) = env.step(action);
        let next_state = env.get_state();

        replay_buffer.push_back(Transition { state, action, reward, next_state, done });
        if replay_buffer.len() > 10_000 {
            replay_buffer.pop_front();
        }

        // Training startet bei ausreichendem Füllstand
        if replay_buffer.len() >= batch_size {
            // Echtes, zufälliges Sampling mittels fastrand
            let mut batch = Vec::with_capacity(batch_size);
            for _ in 0..batch_size {
                let idx = fastrand::usize(0..replay_buffer.len());
                batch.push(&replay_buffer[idx]);
            }

            // Tensoren vorbereiten
            let mut states_vec = Vec::with_capacity(batch_size * 3);
            let mut next_states_vec = Vec::with_capacity(batch_size * 3);
            for t in &batch {
                states_vec.extend_from_slice(&t.state);
                next_states_vec.extend_from_slice(&t.next_state);
            }

            let states_tensor = dev.tensor_from_vec(states_vec, (Const::<64>, Const::<3>));
            let next_states_tensor = dev.tensor_from_vec(next_states_vec, (Const::<64>, Const::<3>));

            let next_q_values = target_net.forward(next_states_tensor).array();
            let mut targets_vec = online_net.forward(states_tensor.clone()).array();
            
            for i in 0..batch_size {
                let max_next_q = next_q_values[i].iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
                let target = batch[i].reward + gamma * max_next_q * (if batch[i].done { 0.0 } else { 1.0 });
                targets_vec[i][batch[i].action] = target;
            }
            let targets_tensor = dev.tensor(targets_vec);

            // Gradienten-Tracking & Backpropagation
            let grads = online_net.alloc_grads();
            let predicted_q = online_net.forward_mut(states_tensor.traced(grads));
            
            let loss = mse_loss(predicted_q, targets_tensor);
            let loss_val = loss.array();

            let mut grads = loss.backward();
            opt.update(&mut online_net, &grads).expect("Optimizer update failed");
            online_net.zero_grads(&mut grads);

            if step % 100 == 0 {
                println!("Step: {}, Loss: {:.4}, Epsilon: {:.2}", step, loss_val, epsilon);
            }
        }

        if step % 200 == 0 {
            target_net = online_net.clone();
        }

        if epsilon > 0.05 {
            epsilon -= 0.002;
        }
    }
}
