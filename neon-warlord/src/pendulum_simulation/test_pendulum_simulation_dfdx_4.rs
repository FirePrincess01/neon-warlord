use dfdx::prelude::*;
use std::collections::VecDeque;

// --- 1. DIE UMWELT (Inverted Pendulum / CartPole Simulation) ---
struct CartPole {
    x: f32,          // Position des Wagens
    x_dot: f32,      // Geschwindigkeit des Wagens
    theta: f32,      // Winkel des Pendels (0 = vertikal nach oben)
    theta_dot: f32,  // Winkelgeschwindigkeit
    steps: usize,
}

impl CartPole {
    fn new() -> Self {
        Self {
            x: 0.0,
            // Kleine zufällige Initialisierung mit fastrand im Bereich -0.05 bis 0.05
            x_dot: fastrand::f32() * 0.1 - 0.05,
            theta: fastrand::f32() * 0.1 - 0.05,
            theta_dot: fastrand::f32() * 0.1 - 0.05,
            steps: 0,
        }
    }

    fn state(&self) -> [f32; 4] {
        [self.x, self.x_dot, self.theta, self.theta_dot]
    }

    // Führt einen Physik-Zeitschritt aus
    fn step(&mut self, action: usize) -> ([f32; 4], f32, bool) {
        self.steps += 1;

        let gravity = 9.8;
        let masscart = 1.0;
        let masspole = 0.1;
        let total_mass = masscart + masspole;
        let length = 0.5;
        let polemass_length = masspole * length;
        let force = if action == 1 { 10.0 } else { -10.0 };
        let tau = 0.02;

        let cos_theta = self.theta.cos();
        let sin_theta = self.theta.sin();

        let temp = (force + polemass_length * self.theta_dot * self.theta_dot * sin_theta) / total_mass;
        let thetaacc = (gravity * sin_theta - cos_theta * temp) / (length * (4.0 / 3.0 - masspole * cos_theta * cos_theta / total_mass));
        let xacc = temp - polemass_length * thetaacc * cos_theta / total_mass;

        self.x += tau * self.x_dot;
        self.x_dot += tau * xacc;
        self.theta += tau * self.theta_dot;
        self.theta_dot += tau * thetaacc;

        let done = self.x < -2.4 || self.x > 2.4 || self.theta < -0.209 || self.theta > 0.209 || self.steps >= 200;
        let reward = if !done { 1.0 } else { 0.0 };

        (self.state(), reward, done)
    }
}

// --- 2. ARCHITEKTUR DES NEURONALEN NETZWERKS ---
type QNetwork = (
    (Linear<4, 64>, ReLU),
    (Linear<64, 64>, ReLU),
    Linear<64, 2>,
);

struct Transition {
    state: [f32; 4],
    action: usize,
    reward: f32,
    next_state: [f32; 4],
    done: bool,
}

#[test]
fn main() {
    let dev = Cpu::default();

    let mut q_net = dev.build_module::<QNetwork, f32>();
    let mut target_net = q_net.clone();
    // let mut sgd = dfdx::optim::Sgd::new(&q_net, dfdx::optim::SgdConfig {
    //     lr: 1e-3,
    //     momentum: None,
    //     weight_decay: None,
    // });

    let mut adam = dfdx::optim::Adam::new(&q_net, dfdx::optim::AdamConfig {
        lr: 1e-3,          // Lernrate
        betas: [0.9, 0.999],
        eps: 1e-8,
        weight_decay: None,
    });

    let mut epsilon = 1.0f32;
    let epsilon_decay = 0.995;
    let epsilon_min = 0.05;
    let gamma = 0.99f32;
    let batch_size = 64;
    let mut replay_buffer: VecDeque<Transition> = VecDeque::with_capacity(10000);

    for episode in 1..=2000 {
        let mut env = CartPole::new();
        let mut total_reward = 0.0;

        loop {
            let state = env.state();

            // Epsilon-Greedy mit fastrand
            let action = if fastrand::f32() < epsilon {
                fastrand::usize(0..2)
            } else {
                let state_tensor = dev.tensor(state);
                let q_values = q_net.forward(state_tensor);
                let q_arr = q_values.array();
                if q_arr[0] > q_arr[1] { 0 } else { 1 }
            };

            let (next_state, reward, done) = env.step(action);
            total_reward += reward;

            replay_buffer.push_back(Transition { state, action, reward, next_state, done });
            if replay_buffer.len() > 10000 {
                replay_buffer.pop_front();
            }

            if replay_buffer.len() >= batch_size {
                // Initialisiere feste Arrays statt dynamische Vektoren (Größe 64)
                let mut states_batch = [[0.0f32; 4]; 64];
                let mut actions_batch = [0usize; 64];
                let mut rewards_batch = [0.0f32; 64];
                let mut next_states_batch = [[0.0f32; 4]; 64];
                let mut dones_batch = [0.0f32; 64];

                for i in 0..batch_size {
                    let idx = fastrand::usize(0..replay_buffer.len());
                    let t = &replay_buffer[idx];
                    
                    states_batch[i] = t.state;
                    actions_batch[i] = t.action;
                    rewards_batch[i] = t.reward;
                    next_states_batch[i] = t.next_state;
                    dones_batch[i] = if t.done { 1.0f32 } else { 0.0f32 };
                }

                // Jetzt akzeptiert dfdx die Arrays problemlos, da die Dimension (64) bekannt ist
                let states_t = dev.tensor(states_batch);
                let next_states_t = dev.tensor(next_states_batch);

                let next_q_values = target_net.forward(next_states_t);
                let next_q_arr = next_q_values.array(); // Typisiert als [[f32; 2]; 64]
                
                let mut target_qs = [0.0f32; 64];
                for i in 0..batch_size {
                    let max_next_q = next_q_arr[i][0].max(next_q_arr[i][1]);
                    target_qs[i] = rewards_batch[i] + gamma * max_next_q * (1.0 - dones_batch[i]);
                }

                // 1. Allokiere die Gradienten (Ownership liegt hier)
                let gradients = q_net.alloc_grads();

                // 2. Trace OHNE `&mut` aufrufen – es übergibt die Ownership an den Tensor
                let pred_q_values = q_net.forward(states_t.trace(gradients));
                let pred_q_arr = pred_q_values.array();

                let mut targets_array = pred_q_arr;
                for i in 0..batch_size {
                    targets_array[i][actions_batch[i]] = target_qs[i];
                }
                let targets_t = dev.tensor(targets_array);

                // 3. Berechne den Loss. Der Loss-Tensor hält nun die Gradienten.
                let loss = mse_loss(pred_q_values, targets_t);

                // 4. Extrahiere die Gradienten aus dem Loss-Tensor mittels Rückwärtspass (Backpropagation)
                let grads = loss.backward();

                // 5. Übergib die extrahierten Gradienten als Referenz an den Optimizer
                adam.update(&mut q_net, &grads).expect("Fehler beim Optimizer-Update");
            }

            if done {
                break;
            }
        }

        if episode % 10 == 0 {
            target_net = q_net.clone();
        }

        if epsilon > epsilon_min {
            epsilon *= epsilon_decay;
        }

        if episode % 10 == 0 {
            println!("Episode: {:3}, Score: {:5.1}, Epsilon: {:.3}", episode, total_reward, epsilon);
        }
    }


    // --- 3. REINE EVALUATIONS-SCHLEIFE (TESTPHASE) ---
    println!("\n=== TRAINING BEENDET ===");
    println!("Starte Evaluation für 10 Episoden mit Epsilon = 0.0 (Keine Zufallsaktionen)...");

    let test_episodes = 10;
    let mut evaluation_scores = Vec::new();

    for test_ep in 1..=test_episodes {
        let mut env = CartPole::new();
        let mut total_reward = 0.0;

        loop {
            let state = env.state();

            // Epsilon ist 0.0 -> Immer die beste bekannte Aktion wählen
            let state_tensor = dev.tensor(state);
            let q_values = q_net.forward(state_tensor);
            let q_arr = q_values.array();
            
            let action = if q_arr[0] > q_arr[1] { 0 } else { 1 };

            // Schritt in der Umwelt ausführen (Kein Abspeichern im Buffer, kein Training)
            let (_, reward, done) = env.step(action);
            total_reward += reward;

            if done {
                break;
            }
        }

        println!("Test Episode {:2}: Score = {:5.1}", test_ep, total_reward);
        evaluation_scores.push(total_reward);
    }

    // Durchschnittlichen Score berechnen
    let avg_score: f32 = evaluation_scores.iter().sum::<f32>() / test_episodes as f32;
    println!("------------------------------------");
    println!("DURCHSCHNITTLICHER TEST-SCORE: {:.1} / 200.0", avg_score);
    println!("====================================");

}
