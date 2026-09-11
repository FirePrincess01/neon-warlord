//! Simulates an inverted pendulum

mod graph_lines;
mod neural_network_drawer;
mod pendulum;
mod verlet_physics_drawer;

use std::collections::VecDeque;

use forward_renderer::{height_map::HeightMapInterface, to_rgb};
use wgpu_renderer::performance_monitor::{Fps, watch::Watch};

use crate::{
    pendulum_simulation::{
        graph_lines::{GraphLines, GraphLinesDrawer}, neural_network_drawer::NeuralNetworkDrawer, pendulum::{Pendulum, PendulumAction, PendulumState}, verlet_physics_drawer::VerletPhysicsDrawer,
    }, physics_simulation_v3_drawer::DrawerObjects, reinforcement_learning::{dqn::{self, Dqn, ReplayKey}, neural_network_simd::NeuralNetworkSimd}, triple_buffer, worker_thread,
};

pub const WATCH_POINTS_SIZE: usize = 10;
type Vec3 = cgmath::Vector3<f32>;

const INPUTS: usize = 4;
const OUTPUTS: usize = 3;
const NR_LAYERS: usize = 5;
const RESIDUAL: bool = true;

pub struct PendulumSimulation {
    // Physics
    ticks: u64,

    // model: Box<NeuralNetworkSimd<INPUTS, OUTPUTS, NR_LAYERS, RESIDUAL>>,
    model_drawer: NeuralNetworkDrawer<INPUTS, OUTPUTS, NR_LAYERS, RESIDUAL>,

    dqn: Dqn<INPUTS, OUTPUTS>,

    graph_loss: GraphLines<1>,
    graph_actions: GraphLines<3>,
    graph_angle: GraphLines<1>,
    graph_angle_vel: GraphLines<1>,
    graph_cart: GraphLines<1>,
    graph_cart_vel: GraphLines<1>,

    graph_drawer_loss: GraphLinesDrawer<1>,
    graph_drawer_actions: GraphLinesDrawer<3>,
    graph_drawer_angle: GraphLinesDrawer<1>,
    graph_drawer_angle_vel: GraphLinesDrawer<1>,
    graph_drawer_cart: GraphLinesDrawer<1>,
    graph_drawer_cart_vel: GraphLinesDrawer<1>,

    pendulum: Pendulum,
    verlet_physics_drawer: VerletPhysicsDrawer,

    // Debug
    ups: Fps,
    last_render_time: instant::Instant,
    watch_ups: Watch<WATCH_POINTS_SIZE>,
}

impl PendulumSimulation {
    pub fn new() -> Self {
        // agent 0
        let pos = Vec3::new(0.0, 0.0, 2.0);
        let pos_model = pos;

        let pos_graph_loss = pos + Vec3::new(-4.2, 1.0, 1.0);
        let pos_graph_actions = pos + Vec3::new(-2.0, 1.0, 1.0);
        let pos_pendulum = pos + Vec3::new(2.0, -0.5, 1.0);

        let pos_graph_angle = pos + Vec3::new(2.2, 1.0, 0.0);
        let pos_graph_angle_vel = pos + Vec3::new(2.2, 1.0, 2.2);
        let pos_graph_cart = pos + Vec3::new(4.4, 1.0, 0.0);
        let pos_graph_cart_vel = pos + Vec3::new(4.4, 1.0, 2.2);

        let scale = 0.1;

        // let model = Box::new(NeuralNetworkSimd::new());
        let dqn = Dqn::new();
        let model_drawer = NeuralNetworkDrawer::new(&dqn.model, scale, pos_model);

        // Debug
        let ups = Fps::new();
        let watch_ups = Watch::new();

        // Graph
        let graph_x: VecDeque<f32> = (0..100).map(|i| i as f32 * 0.1).collect();
        // let graph_y: VecDeque<f32> = (0..100).map(|i| (i as f32 * 0.1).sin()).collect();
        let graph_y: VecDeque<f32> = (0..100).map(|_i| 0.0).collect();
        let graph_loss = GraphLines {
            x: graph_x.clone(),
            y: [graph_y.clone()],
        };

        let graph_actions = GraphLines {
            x: graph_x.clone(),
            y: [graph_y.clone(), graph_y.clone(), graph_y.clone()],
        };

        let graph_angle = GraphLines {
            x: graph_x.clone(),
            y: [graph_y.clone()],
        };
        let graph_angle_vel = GraphLines {
            x: graph_x.clone(),
            y: [graph_y.clone()],
        };
        let graph_cart = GraphLines {
            x: graph_x.clone(),
            y: [graph_y.clone()],
        };
        let graph_cart_vel = GraphLines {
            x: graph_x.clone(),
            y: [graph_y.clone()],
        };

        let graph_drawer_loss =
            GraphLinesDrawer::new(scale, pos_graph_loss).colors([to_rgb("#12d900").into()]);
        let graph_drawer_actions =
            GraphLinesDrawer::new(scale, pos_graph_actions).colors([to_rgb("#f75ee7").into(), to_rgb("#1d0d1b").into(), to_rgb("#744ff7").into()]);
        let graph_drawer_angle = GraphLinesDrawer::new(scale, pos_graph_angle)
            .colors([to_rgb("#d9ae00").into()])
            .y_lim(std::f32::consts::PI);
        let graph_drawer_angle_vel = GraphLinesDrawer::new(scale, pos_graph_angle_vel)
            .colors([to_rgb("#7700d9").into()])
            .y_lim(std::f32::consts::PI);
        let graph_drawer_cart =
            GraphLinesDrawer::new(scale, pos_graph_cart).colors([to_rgb("#0070d9").into()]);
        let graph_drawer_cart_vel =
            GraphLinesDrawer::new(scale, pos_graph_cart_vel).colors([to_rgb("#0041d9").into()]);

        // Pendulum
        let pendulum = Pendulum::new();
        let verlet_physics_drawer =
            VerletPhysicsDrawer::new(&pendulum.verlet_physics, scale, pos_pendulum);

        // Dqn
        

        Self {
            ticks: 0,

            // model,
            model_drawer,
            graph_loss,
            graph_angle,
            graph_angle_vel,
            graph_cart,
            graph_cart_vel,
            pendulum,
            verlet_physics_drawer,

            ups,
            last_render_time: instant::Instant::now(),
            watch_ups,
            graph_drawer_loss,
            graph_drawer_angle,
            graph_drawer_angle_vel,
            graph_drawer_cart,
            graph_drawer_cart_vel,
            dqn,
            graph_actions,
            graph_drawer_actions,
        }
    }

    pub fn update_physics(&mut self, _height_map: &impl HeightMapInterface) {
        let dt = 1.0 / 60.0;
        self.ticks += 1;

        self.watch_ups.start("Solver");
        let pendulum_state = self.pendulum.state();
        let pendulum_action = self.get_pendulum_action(&pendulum_state);
        let pendulum_state_new = self.pendulum.update(pendulum_action, dt);
        self.set_pendulum_reward(&pendulum_state, pendulum_action, &pendulum_state_new);

       

        self.graph_angle.y_push_pop(0, pendulum_state_new.alpha);
        self.graph_angle_vel
            .y_push_pop(0, pendulum_state_new.angular_velocity);
        self.graph_cart.y_push_pop(0, pendulum_state_new.cart_pos);
        self.graph_cart_vel.y_push_pop(0, pendulum_state_new.cart_velocity);

        self.pendulum.update_verlet_physics(dt);
        self.watch_ups.stop();

        // ups
        let now = instant::Instant::now();
        let dt = now - self.last_render_time;
        self.last_render_time = now;
        self.ups.update(dt);
    }

    fn get_pendulum_action(&mut self, pendulum_state: &PendulumState) -> PendulumAction {
        let alpha = pendulum_state.alpha;
        let angular_velocity = pendulum_state.angular_velocity;
        let cart_pos = pendulum_state.cart_pos;
        let cart_velocity = pendulum_state.cart_velocity;

        let inputs = [
            alpha, 
            angular_velocity, 
            cart_pos, 
            cart_velocity, 
        ];

        let action = self.dqn.choose_action(&inputs);
         self.graph_actions.y_push_pop(0, action.1[0]);
         self.graph_actions.y_push_pop(1, action.1[1]);
         self.graph_actions.y_push_pop(2, action.1[2]);

        let pendulum_action: PendulumAction = (action.0 as u8).into();

        pendulum_action
    }

    fn set_pendulum_reward(&mut self, 
        pendulum_state: &PendulumState, 
        pendulum_action: PendulumAction, 
        pendulum_state_next: &PendulumState,
    ) {
        let alpha = pendulum_state.alpha;
        let angular_velocity = pendulum_state.angular_velocity;
        let cart_pos = pendulum_state.cart_pos;
        let cart_velocity = pendulum_state.cart_velocity;
        let action: u8 = pendulum_action.into();
        let action = action as usize;
        // println!("action: {}", action);

        let inputs: [f32; 4] = [
            pendulum_state.alpha, 
            pendulum_state.angular_velocity, 
            pendulum_state.cart_pos, 
            pendulum_state.cart_velocity, 
        ];

        let inputs_next: [f32; 4] = [
            pendulum_state_next.alpha, 
            pendulum_state_next.angular_velocity, 
            pendulum_state_next.cart_pos, 
            pendulum_state_next.cart_velocity, 
        ];

        let replay_key_inputs: [i8; 4] = [
            (alpha / std::f32::consts::PI * 100.0) as i8,
            (angular_velocity * 100.0) as i8,
            (cart_pos * 100.0) as i8,
            (cart_velocity * 100.0) as i8,
        ];

         let replay_key = ReplayKey {
            inputs: replay_key_inputs,
            action,
        };

        let reward = alpha.abs();
        
        if self.ticks > 100 {
            let episode_finished = self.ticks.is_multiple_of(1000);
            
            let finished = episode_finished;

            self.dqn.set_reward(inputs, action, reward, inputs_next, finished, replay_key);

            if episode_finished {
                let loss = self.dqn.learn_replay();
                self.graph_loss.y_push_pop(0, loss);
            }
        }
    }

    pub fn update_drawer(&mut self, objects: &mut DrawerObjects) {
        let nodes = &mut objects.genome_nodes;
        let edges = &mut objects.genome_edges;

        self.watch_ups.start("Draw Model");
        self.model_drawer.update(&self.dqn.model, nodes, edges);

        self.graph_drawer_loss.update(&self.graph_loss, edges);
        self.graph_drawer_actions.update(&self.graph_actions, edges);
        self.graph_drawer_angle.update(&self.graph_angle, edges);
        self.graph_drawer_angle_vel
            .update(&self.graph_angle_vel, edges);
        self.graph_drawer_cart.update(&self.graph_cart, edges);
        self.graph_drawer_cart_vel
            .update(&self.graph_cart_vel, edges);

        self.verlet_physics_drawer
            .update(&self.pendulum.verlet_physics, nodes, edges);

        self.watch_ups.stop();

        objects.ups = self.ups.get();
        self.watch_ups.update();
        objects.watch_ups = self.watch_ups.get_viewer_data();
    }
    

}

pub struct PendulumSimulationThread<T>
where
    T: HeightMapInterface,
{
    pub sim: PendulumSimulation,
    pub producer: triple_buffer::Producer<DrawerObjects>,
    pub height_map: T,
}

impl<T> worker_thread::Update for PendulumSimulationThread<T>
where
    T: HeightMapInterface,
{
    fn update(&mut self) {
        let data = self.producer.buffer();
        data.clear();

        self.sim.update_physics(&self.height_map);
        self.sim.update_drawer(data);

        self.producer.publish();
    }
}
