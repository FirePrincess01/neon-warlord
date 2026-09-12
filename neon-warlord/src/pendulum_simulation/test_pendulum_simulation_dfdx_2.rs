use dfdx::optim::Adam;
use dfdx::prelude::*;
use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;

type QNetwork = (
    Linear<3, 64>,
    ReLU,
    Linear<64, 64>,
    ReLU,
    Linear<64, 3>,
);

struct Transition {
    state: [f32; 3],
    action: usize,
    reward: f32,
    next_state: [f32; 3],
    done: bool,
}

#[derive(Clone)]
struct PendulumEnv {
    theta: f32,
    theta_dot: f32,
}

impl PendulumEnv {
    fn new() -> Self {
        Self {
            theta: fastrand::f32() * 6.28 - 3.14, 
            theta_dot: 0.0,
        }
    }

    fn get_state(&self) -> [f32; 3] {
        [self.theta.cos(), self.theta.sin(), self.theta_dot]
    }

    // fn step(&mut self, action_idx: usize) -> (f32, bool) {
    //     let u = match action_idx {
    //         0 => -2.0,
    //         1 => 0.0,
    //         _ => 2.0,
    //     };

    //     let g = 9.81;
    //     let m = 1.0;
    //     let l = 1.0;
    //     let dt = 0.05;

    //     let theta_acc = (3.0 * g / (2.0 * l)) * self.theta.sin() + (3.0 / (m * l * l)) * u;
    //     self.theta_dot += theta_acc * dt;
    //     self.theta = (self.theta + self.theta_dot * dt + std::f32::consts::PI)
    //         .rem_euclid(2.0 * std::f32::consts::PI) - std::f32::consts::PI;

    //     let reward = -(self.theta.powi(2) + 0.2 * self.theta_dot.powi(2) + 0.001 * u.powi(2));
        
    //     (reward, false)
    // }

    fn step(&mut self, action_idx: usize) -> (f32, bool) {
        // 1. Übersetzung des diskreten Aktions-Indexes in ein kontinuierliches Drehmoment (Torque)
        let torque = match action_idx {
            0 => -2.0, // Maximaler Impuls nach links
            1 =>  0.0, // Kein Drehmoment (Gleitphase)
            _ =>  2.0, // Maximaler Impuls nach rechts
        };

        // 2. Physikalische Systemkonstanten (Pendel-Spezifikationen)
        let gravity = 9.81;       // Erdbeschleunigung in m/s²
        let mass = 1.0;          // Masse des Pendelkörpers in kg
        let length = 1.0;        // Länge der Pendelstange in Metern
        let time_step = 0.05;    // Delta t (Diskretes Zeitintervall pro Schritt) in Sekunden

        // 3. Berechnung der Winkelbeschleunigung (Angular Acceleration)
        // Formel basiert auf der Drehmomentbilanz eines starren Rotors (Inverted Pendulum Physik)
        let gravity_torque = (3.0 * gravity / (2.0 * length)) * self.theta.sin();
        let control_torque = (3.0 / (mass * length * length)) * torque;
        let theta_acceleration = gravity_torque + control_torque;

        // 4. Update der Systemzustände mittels Semi-implizitem Euler-Verfahren
        // Neue Winkelgeschwindigkeit berechnen
        self.theta_dot += theta_acceleration * time_step;
        
        // Neuen Winkel berechnen und direkt im Intervall [-PI, PI] normalisieren
        let raw_next_theta = self.theta + self.theta_dot * time_step;
        let pi = std::f32::consts::PI;
        self.theta = (raw_next_theta + pi).rem_euclid(2.0 * pi) - pi;

        // 5. Kosten- und Belohnungsfunktion (Reward Shaping)
        // Bestraft Abweichung vom Zenit, hohe Rotationsgeschwindigkeit und exzessiven Energieeinsatz
        let angle_penalty = self.theta.powi(2);
        let velocity_penalty = 0.2 * self.theta_dot.powi(2);
        let control_penalty = 0.001 * torque.powi(2);
        
        let reward = -(angle_penalty + velocity_penalty + control_penalty);
        
        // Das Pendel hat kein natürliches "Done"-Kriterium, da es unendlich weiterbalancieren soll
        let is_terminal = false; 
        
        (reward, is_terminal)
    }

    // Hilfsfunktion zur Darstellung des Pendels in der Konsole
    fn render_ascii(&self) -> &'static str {
        let deg = self.theta.to_degrees();
        if deg.abs() < 15.0 {
            "  |  (Perfekt ausbalanciert)"
        } else if deg >= 15.0 && deg < 60.0 {
            "  \\"
        } else if deg <= -15.0 && deg > -60.0 {
            "  /"
        } else if deg >= 60.0 && deg < 120.0 {
            "  __"
        } else if deg <= -60.0 && deg > -120.0 {
            "__"
        } else {
            "  .  (Hängt nach unten)"
        }
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

    let max_steps = 100_000;
    let mut epsilon = 1.0f32;
    let epsilon_decay = 0.0005; // Langsameres Absinken über ca. 2000 Schritte
    let target_update_interval = 400; // Selteneres Updaten schützt vor Oszillationen


    println!("--- TRAINING START ---");
    for step in 0..max_steps {
        let state = env.get_state();
        
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

        if replay_buffer.len() >= batch_size {
            let mut batch = Vec::with_capacity(batch_size);
            for _ in 0..batch_size {
                let idx = fastrand::usize(0..replay_buffer.len());
                batch.push(&replay_buffer[idx]);
            }

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

            let grads = online_net.alloc_grads();
            let predicted_q = online_net.forward_mut(states_tensor.traced(grads));
            
            let loss = mse_loss(predicted_q, targets_tensor);
            let loss_val = loss.array();

            let mut grads = loss.backward();
            opt.update(&mut online_net, &grads).expect("Optimizer update failed");
            online_net.zero_grads(&mut grads);

            if step % 1000 == 0 && replay_buffer.len() >= batch_size {
                println!("Step: {}, Loss: {:.4}, Epsilon: {:.2}", step, loss_val, epsilon);
            }
        }

        if step % target_update_interval == 0 {
            target_net = online_net.clone();
        }

        if epsilon > 0.05 {
            epsilon -= epsilon_decay;
        }
    }

    // ==========================================
    // ============ EVALUIERUNG =================
    // ==========================================
    println!("\n--- EVALUIERUNG START (100 Schritte) ---");
    
    // Wir setzen das Pendel für den Test auf eine schwierige Startposition (z.B. leicht schräg nach oben)
    let mut eval_env = PendulumEnv { theta: 0.3, theta_dot: 0.0 }; 
    let mut balanced_steps = 0;

    for eval_step in 1..=100 {
        let state = eval_env.get_state();
        
        // Reine Exploitation: Bestmögliche Aktion laut trainiertem Netzwerk wählen
        let state_tensor = dev.tensor(state);
        let q_values = online_net.forward(state_tensor);
        let q_arr = q_values.array();
        let action = q_arr.iter().enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();

        // Schritt ausführen
        let _ = eval_env.step(action);

        // Prüfen, ob das Pendel innerhalb von +/- 15 Grad (ca. 0.26 Radian) steht
        let is_balanced = eval_env.theta.abs() < 0.26;
        if is_balanced {
            balanced_steps += 1;
        }

        // Live-Anzeige im Terminal
        println!(
            "Eval-Schritt: {:3} | Winkel: {:6.1}° | Visualisierung: {}", 
            eval_step, 
            eval_env.theta.to_degrees(), 
            eval_env.render_ascii()
        );

        // Kurze Pause, damit man das Pendeln in Echtzeit sieht
        sleep(Duration::from_millis(50));
    }

    println!("\n--- ERGEBNIS ---");
    println!("Das Pendel war in {} von 100 Testschritten perfekt ausbalanciert!", balanced_steps);
}
