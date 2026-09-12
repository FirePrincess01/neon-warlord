use dfdx::prelude::*;
use std::collections::VecDeque;
use std::f32::consts::PI;

// --- 1. DIE UMWELT (Inverted Pendulum / CartPole Simulation für Swing-Up) ---
struct CartPole {
    x: f32,          // Position des Wagens
    x_dot: f32,      // Geschwindigkeit des Wagens
    theta: f32,      // Winkel des Pendels (0 = vertikal nach oben, PI/-PI = nach unten hängend)
    theta_dot: f32,  // Winkelgeschwindigkeit
    steps: usize,
}

impl CartPole {
    fn new() -> Self {
        let mut env = Self {
            x: 0.0,
            x_dot: 0.0,
            theta: PI, // Startet exakt nach unten hängend (180 Grad)
            theta_dot: 0.0,
            steps: 0,
        };
        // Kleiner zufälliger Schubs mit fastrand, um die Symmetrie zu brechen
        env.theta += fastrand::f32() * 0.1 - 0.05;
        env
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
        let force = if action == 1 { 15.0 } else { -15.0 };
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

        // Winkel fortlaufend auf den Bereich [-PI, PI] normalisieren.
        // Dadurch ist 0.0 perfekt oben, PI und -PI sind unten.
        self.theta = ((self.theta + PI) % (2.0 * PI)) - PI;

        // Für den Swing-Up bricht das Spiel NICHT mehr ab, wenn das Pendel unten ist.
        // Es bricht nur ab, wenn der Wagen aus dem Bildschirm fährt oder 200 Schritte um sind.
        let out_of_bounds = self.x < -2.4 || self.x > 2.4;
        let done = out_of_bounds || self.steps >= 200;

        // REWARD SHAPING FÜR DEN SWING-UP (Überarbeitet):
        let reward = if !out_of_bounds {
            // cos(theta) ist +1.0 wenn perfekt oben, und -1.0 wenn unten.
            // Durch (+1.0) / 2.0 normieren wir den Wert perfekt auf den Bereich [0.0 bis 1.0].
            let target_reward = (self.theta.cos() + 1.0) / 2.0;
            
            // Wir nehmen den Wert hoch 2, damit "fast oben" extrem viel mehr belohnt wird
            // als das bloße Herabhängen im Keller.
            let mut r = target_reward.powi(2);

            // Kleine Strafen für zu wildes Bewegen, damit er oben stabilisiert
            r -= 0.01 * self.theta_dot.powi(2);
            r -= 0.01 * self.x_dot.powi(2);

            r.max(0.0) // Verhindert, dass der Reward negativ wird!
        } else {
            0.0 // Keine harte negative Strafe, einfach Null bei Out-of-Bounds
        };

        (self.state(), reward, done)
    }
}

// --- 2. ARCHITEKTUR DES NEURONALEN NETZWERKS (Kapazität auf 128 erweitert) ---
type QNetwork = (
    (Linear<4, 128>, ReLU),
    (Linear<128, 128>, ReLU),
    Linear<128, 2>,
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

    let mut adam = dfdx::optim::Adam::new(&q_net, dfdx::optim::AdamConfig {
        lr: 1e-3,
        betas: [0.9, 0.999],
        eps: 1e-8,
        weight_decay: None,
    });

    let mut epsilon = 1.0f32;
    let epsilon_decay = 0.998; 
    let epsilon_min = 0.02;
    let gamma = 0.99f32;
    let batch_size = 64;
    let mut replay_buffer: VecDeque<Transition> = VecDeque::with_capacity(20000);

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
            if replay_buffer.len() > 20000 {
                replay_buffer.pop_front();
            }

            if replay_buffer.len() >= batch_size {
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

                let states_t = dev.tensor(states_batch);
                let next_states_t = dev.tensor(next_states_batch);

                // --- DOUBLE DQN IMPLEMENTIERUNG ---
                // 1. Das ONLINE-Netzwerk bestimmt die beste Aktion für den Folgezustand
                let next_online_q = q_net.forward(next_states_t.clone());
                let next_online_arr = next_online_q.array();

                // 2. Das TARGET-Netzwerk berechnet den stabilen Q-Wert für ebendiese Aktion
                let next_target_q = target_net.forward(next_states_t);
                let next_target_arr = next_target_q.array();
                
                let mut target_qs = [0.0f32; 64];
                for i in 0..batch_size {
                    // Wähle Aktion mit dem Online-Netz aus
                    let best_action_next = if next_online_arr[i][0] > next_online_arr[i][1] { 0 } else { 1 };
                    // Bewerte sie über das Target-Netz (Verhindert Überschätzungs-Bias)
                    let max_next_q = next_target_arr[i][best_action_next];
                    
                    target_qs[i] = rewards_batch[i] + gamma * max_next_q * (1.0 - dones_batch[i]);
                }

                let gradients = q_net.alloc_grads();
                let pred_q_values = q_net.forward(states_t.trace(gradients));
                let pred_q_arr = pred_q_values.array();

                let mut targets_array = pred_q_arr;
                for i in 0..batch_size {
                    targets_array[i][actions_batch[i]] = target_qs[i];
                }
                let targets_t = dev.tensor(targets_array);

                let loss = mse_loss(pred_q_values, targets_t);
                let grads = loss.backward();
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

        if episode % 20 == 0 {
            println!("Episode: {:4}, Accum. Reward: {:7.1}, Epsilon: {:.3}", episode, total_reward, epsilon);
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

            let state_tensor = dev.tensor(state);
            let q_values = q_net.forward(state_tensor);
            let q_arr = q_values.array();
            
            let action = if q_arr[0] > q_arr[1] { 0 } else { 1 };

            let (_, reward, done) = env.step(action);
            total_reward += reward;

            if done {
                break;
            }
        }

        println!("Test Episode {:2}: Accum. Reward = {:7.1}", test_ep, total_reward);
        evaluation_scores.push(total_reward);
    }

    let avg_score: f32 = evaluation_scores.iter().sum::<f32>() / test_episodes as f32;
    println!("------------------------------------");
    println!("DURCHSCHNITTLICHER EVALUATIONS-REWARD: {:.1}", avg_score);
    println!("====================================");


    // --- 3. REINE EVALUATIONS-SCHLEIFE MIT ASCII-VISUALISIERUNG ---
    println!("\n=== TRAINING BEENDET ===");
    println!("Starte visualisierte Evaluation für 3 Episoden...");

    for test_ep in 1..=3 {
        let mut env = CartPole::new();
        let mut total_reward = 0.0;

        loop {
            let state = env.state();

            let state_tensor = dev.tensor(state);
            let q_values = q_net.forward(state_tensor);
            let q_arr = q_values.array();
            
            let action = if q_arr[0] > q_arr[1] { 0 } else { 1 };

            let (_, reward, done) = env.step(action);
            total_reward += reward;

            // ASCII-Darstellung berechnen
            // Wagenposition auf einer Linie von 40 Zeichen mappen
            let cart_idx = (((env.x + 2.4) / 4.8) * 38.0).clamp(0.0, 38.0) as usize;
            let mut track = vec!["-"; 39];
            track[cart_idx] = "█"; // Das Auto
            
            // Pendel-Richtung bestimmen anhand des Sinus/Cosinus
            let pole_char = if env.theta.cos() > 0.5 {
                "|" // Oben
            } else if env.theta.sin() > 0.0 {
                "/" // Rechts geneigt
            } else if env.theta.sin() < -0.0 {
                "\\" // Links geneigt
            } else {
                "." // Unten hängend
            };

            // Terminal-Zeile ausgeben und kurz schlafen, um Animation zu erzeugen
            print!("\rTrack: {}  Pendel: {}  Winkel: {:5.2} rad", track.join(""), pole_char, env.theta);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(30)); // 30ms pro Frame

            if done {
                break;
            }
        }
        println!("\nTest Episode {:2}: Accum. Reward = {:7.1}\n", test_ep, total_reward);
    }
}
