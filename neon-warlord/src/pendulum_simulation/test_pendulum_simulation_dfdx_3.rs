use dfdx::optim::Adam;
use dfdx::prelude::*;
use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;

// ============================================================
// Q-NETWORK
// State:
// [cart_position, cart_velocity, cos(theta), sin(theta), theta_dot]
// Actions:
// 0 = push left
// 1 = no force
// 2 = push right
// ============================================================

type QNetwork = (
    Linear<5, 128>,
    ReLU,
    Linear<128, 128>,
    ReLU,
    Linear<128, 3>,
);

struct Transition {
    state: [f32; 5],
    action: usize,
    reward: f32,
    next_state: [f32; 5],
    done: bool,
}

// ============================================================
// CART POLE ENVIRONMENT
// ============================================================

#[derive(Clone)]
struct CartPoleEnv {
    // Cart
    x: f32,
    x_dot: f32,

    // Pole angle and angular velocity
    theta: f32,
    theta_dot: f32,
}

impl CartPoleEnv {
    fn new() -> Self {
        Self {
            // Small random initial position
            x: (fastrand::f32() * 2.0 - 1.0) * 0.5,

            // Small initial velocity
            x_dot: 0.0,

            // Start close to upright
            theta: (fastrand::f32() * 2.0 - 1.0) * 0.15,

            theta_dot: 0.0,
        }
    }

    // --------------------------------------------------------
    // State representation
    // --------------------------------------------------------

    fn get_state(&self) -> [f32; 5] {
        [
            self.x,
            self.x_dot,
            self.theta.cos(),
            self.theta.sin(),
            self.theta_dot,
        ]
    }

    // --------------------------------------------------------
    // Physics
    // --------------------------------------------------------

    fn step(&mut self, action_idx: usize) -> (f32, bool) {
        // Force applied to cart
        let force = match action_idx {
            0 => -10.0,
            1 => 0.0,
            _ => 10.0,
        };

        // Physical constants
        let gravity = 9.81;
        let mass_cart = 1.0;
        let mass_pole = 0.1;
        let total_mass = mass_cart + mass_pole;

        // Half pole length
        let half_length = 0.5;

        // Pole moment of inertia
        let pole_mass_length = mass_pole * half_length;

        // Simulation timestep
        let dt = 0.02;

        // ----------------------------------------------------
        // Standard cart-pole equations of motion
        // ----------------------------------------------------

        let sin_theta = self.theta.sin();
        let cos_theta = self.theta.cos();

        let temp =
            (force + pole_mass_length * self.theta_dot.powi(2) * sin_theta)
            / total_mass;

        let theta_acceleration =
            (gravity * sin_theta - cos_theta * temp)
            /
            (
                half_length
                    * (
                        4.0 / 3.0
                        - mass_pole * cos_theta.powi(2) / total_mass
                    )
            );

        let x_acceleration =
            temp
            - pole_mass_length
                * theta_acceleration
                * cos_theta
                / total_mass;

        // ----------------------------------------------------
        // Semi-implicit Euler integration
        // ----------------------------------------------------

        self.x_dot += x_acceleration * dt;
        self.theta_dot += theta_acceleration * dt;

        self.x += self.x_dot * dt;
        self.theta += self.theta_dot * dt;

        // Normalize angle to [-PI, PI]
        let pi = std::f32::consts::PI;

        self.theta =
            (self.theta + pi)
                .rem_euclid(2.0 * pi)
                - pi;

        // ----------------------------------------------------
        // Terminal conditions
        // ----------------------------------------------------

        let cart_out_of_bounds = self.x.abs() > 2.4;

        // 12 degrees from upright
        let pole_fallen =
            self.theta.abs() > 12.0_f32.to_radians();

        let is_terminal =
            cart_out_of_bounds || pole_fallen;

        // ----------------------------------------------------
        // Reward
        // ----------------------------------------------------

        // Main objective:
        // 1. Keep pole upright
        // 2. Keep cart near center
        // 3. Keep velocities small
        // 4. Don't use excessive force
        //
        // Maximum reward is approximately +1.

        // let angle_error = self.theta;

        // let reward =
        //     1.0
        //     - 2.0 * angle_error.powi(2)
        //     - 0.05 * self.x.powi(2)
        //     - 0.01 * self.x_dot.powi(2)
        //     - 0.01 * self.theta_dot.powi(2)
        //     - 0.0005 * force.powi(2);

        // let reward = if is_terminal {
        //     -10.0
        // } else {
        //     reward
        // };

        // ========================================================
        // Reward shaping
        // ========================================================


        let angle_reward =
            1.0 - 5.0 * self.theta.powi(2);

        let position_penalty =
            0.15 * self.x.powi(2);

        let velocity_penalty =
            0.08 * self.x_dot.powi(2);

        let angular_velocity_penalty =
            0.02 * self.theta_dot.powi(2);

        let force_penalty =
            0.001 * force.powi(2);

        let reward =
            1.0
            + angle_reward
            - position_penalty
            - velocity_penalty
            - angular_velocity_penalty
            - force_penalty;

        let reward = if is_terminal {
            -10.0
        } else {
            reward
        };

        (reward, is_terminal)
    }

    // --------------------------------------------------------
    // ASCII visualization
    // --------------------------------------------------------

    fn render_ascii(&self) -> String {
        let width = 61;

        // Map cart position [-2.4, 2.4] to screen
        let normalized =
            ((self.x + 2.4) / 4.8)
                .clamp(0.0, 1.0);

        let cart_pos =
            (normalized * (width - 1) as f32) as usize;

        let mut line =
            vec![' '; width];

        line[cart_pos] = '■';

        let pole_symbol =
            if self.theta.abs() < 5.0_f32.to_radians() {
                '|'
            } else if self.theta > 0.0 {
                '/'
            } else {
                '\\'
            };

        // Display pole approximately above cart
        let pole_pos = cart_pos;

        if pole_pos < width {
            line[pole_pos] = pole_symbol;
        }

        let track =
            "-".repeat(width);

        format!(
            "x={:+.2}m  angle={:+.1}°\n{}\n{}",
            self.x,
            self.theta.to_degrees(),
            String::from_iter(line),
            track
        )
    }
}

// ============================================================
// TRAINING
// ============================================================

#[test]
fn main() {
    let dev = Cpu::default();

    let mut online_net =
        dev.build_module::<QNetwork, f32>();

    let mut target_net =
        online_net.clone();

    let mut opt = Adam::new(
        &online_net,
        AdamConfig {
            lr: 1e-3,
            ..Default::default()
        },
    );

    let mut replay_buffer =
        VecDeque::with_capacity(10_000);

    let mut env = CartPoleEnv::new();

    let gamma = 0.99_f32;
    let batch_size = 64;

    let max_steps = 300_000;

    // --------------------------------------------------------
    // Exploration
    // --------------------------------------------------------

    let mut epsilon = 1.0_f32;

    let epsilon_min = 0.05;
    let epsilon_decay = 0.00001;

    // --------------------------------------------------------
    // Target network
    // --------------------------------------------------------

    let target_update_interval = 1000;

    println!("======================================");
    println!("       CART-POLE DQN TRAINING");
    println!("======================================");

    for step in 0..max_steps {

        // Reset after episode ends
        if step == 0 || env.x.abs() > 2.4
            || env.theta.abs() > 12.0_f32.to_radians()
        {
            env = CartPoleEnv::new();
        }

        let state = env.get_state();

        // ----------------------------------------------------
        // Epsilon-greedy action selection
        // ----------------------------------------------------

        let action =
            if fastrand::f32() < epsilon {

                // Exploration
                fastrand::usize(0..3)

            } else {

                // Exploitation
                let state_tensor =
                    dev.tensor(state);

                let q_values =
                    online_net.forward(state_tensor);

                let q_arr =
                    q_values.array();

                q_arr
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| {
                        a.partial_cmp(b).unwrap()
                    })
                    .map(|(idx, _)| idx)
                    .unwrap()
            };

        // ----------------------------------------------------
        // Environment step
        // ----------------------------------------------------

        let (reward, done) =
            env.step(action);

        let next_state =
            env.get_state();

        // ----------------------------------------------------
        // Store transition
        // ----------------------------------------------------

        replay_buffer.push_back(
            Transition {
                state,
                action,
                reward,
                next_state,
                done,
            }
        );

        if replay_buffer.len() > 10_000 {
            replay_buffer.pop_front();
        }

        // ----------------------------------------------------
        // Train DQN
        // ----------------------------------------------------

        if replay_buffer.len() >= batch_size {

            let mut batch =
                Vec::with_capacity(batch_size);

            for _ in 0..batch_size {

                let idx =
                    fastrand::usize(
                        0..replay_buffer.len()
                    );

                batch.push(
                    &replay_buffer[idx]
                );
            }

            // ------------------------------------------------
            // Build tensors
            // ------------------------------------------------

            let mut states_vec =
                Vec::with_capacity(
                    batch_size * 5
                );

            let mut next_states_vec =
                Vec::with_capacity(
                    batch_size * 5
                );

            for t in &batch {

                states_vec.extend_from_slice(
                    &t.state
                );

                next_states_vec.extend_from_slice(
                    &t.next_state
                );
            }

            let states_tensor =
                dev.tensor_from_vec(
                    states_vec,
                    (Const::<64>, Const::<5>)
                );

            let next_states_tensor =
                dev.tensor_from_vec(
                    next_states_vec,
                    (Const::<64>, Const::<5>)
                );

            // ------------------------------------------------
            // Target Q-values
            // ------------------------------------------------

            let next_q_values =
                target_net
                    .forward(next_states_tensor)
                    .array();

            let mut targets_vec =
                online_net
                    .forward(states_tensor.clone())
                    .array();

            for i in 0..batch_size {

                let max_next_q =
                    next_q_values[i]
                        .iter()
                        .max_by(|a, b| {
                            a.partial_cmp(b)
                                .unwrap()
                        })
                        .unwrap();

                let target =
                    if batch[i].done {

                        batch[i].reward

                    } else {

                        batch[i].reward
                            + gamma * max_next_q
                    };

                targets_vec[i][batch[i].action] =
                    target;
            }

            let targets_tensor =
                dev.tensor(targets_vec);

            // ------------------------------------------------
            // Backpropagation
            // ------------------------------------------------

            let grads =
                online_net.alloc_grads();

            let predicted_q =
                online_net.forward_mut(
                    states_tensor.traced(grads)
                );

            let loss =
                mse_loss(
                    predicted_q,
                    targets_tensor
                );

            let loss_val =
                loss.array();

            let mut grads =
                loss.backward();

            opt.update(
                &mut online_net,
                &grads
            )
            .expect(
                "Optimizer update failed"
            );

            online_net.zero_grads(
                &mut grads
            );

            // ------------------------------------------------
            // Logging
            // ------------------------------------------------

            if step % 1000 == 0 {

                println!(
                    "Step: {:6} | Loss: {:8.4} | Epsilon: {:.3} | x: {:+.2} | angle: {:+.1}°",
                    step,
                    loss_val,
                    epsilon,
                    env.x,
                    env.theta.to_degrees()
                );
            }
        }

        // ----------------------------------------------------
        // Update target network
        // ----------------------------------------------------

        if step % target_update_interval == 0 {

            target_net =
                online_net.clone();
        }

        // ----------------------------------------------------
        // Epsilon decay
        // ----------------------------------------------------

        if epsilon > epsilon_min {

            epsilon -= epsilon_decay;

            if epsilon < epsilon_min {
                epsilon = epsilon_min;
            }
        }
    }

    // ========================================================
    // EVALUATION
    // ========================================================

    println!();
    println!("======================================");
    println!("          CART-POLE EVALUATION");
    println!("======================================");

    // Difficult but recoverable initial state
    let mut eval_env =
        CartPoleEnv {
            x: 0.0,
            x_dot: 0.0,
            theta: 0.15,
            theta_dot: 0.0,
        };

    let mut balanced_steps = 0;
    let eval_steps = 500;

    for eval_step in 1..=eval_steps {

        let state =
            eval_env.get_state();

        // Pure exploitation
        let state_tensor =
            dev.tensor(state);

        let q_values =
            online_net.forward(
                state_tensor
            );

        let q_arr =
            q_values.array();

        let action =
            q_arr
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| {
                    a.partial_cmp(b)
                        .unwrap()
                })
                .map(|(idx, _)| idx)
                .unwrap();

        let (reward, done) =
            eval_env.step(action);

        // ----------------------------------------------------
        // Check balance
        // ----------------------------------------------------

        let balanced =
            eval_env.theta.abs()
                < 10.0_f32.to_radians()
                && eval_env.x.abs() < 2.0;

        if balanced {
            balanced_steps += 1;
        }

        // ----------------------------------------------------
        // Display
        // ----------------------------------------------------

        println!(
            "\nEval step: {:3} | Action: {} | Reward: {:+.3}",
            eval_step,
            action,
            reward
        );

        println!(
            "{}",
            eval_env.render_ascii()
        );

        if done {

            println!(
                "\nPENDEL FALLEN / CART OUT OF BOUNDS!"
            );

            break;
        }

        sleep(
            Duration::from_millis(20)
        );
    }

    println!();
    println!("======================================");
    println!("              RESULT");
    println!("======================================");

    println!(
        "Balanced: {} / {} steps",
        balanced_steps,
        eval_steps
    );
}
