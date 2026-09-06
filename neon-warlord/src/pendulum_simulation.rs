//! Simulates an inverted pendulum

mod graph_lines;
mod neural_network_drawer;
mod pendulum;
mod verlet_physics_drawer;

use std::{collections::VecDeque, time::Duration};

use forward_renderer::{height_map::HeightMapInterface, to_rgb};
use instant::Instant;
use wgpu_renderer::performance_monitor::{Fps, watch::Watch};

use crate::{
    pendulum_simulation::{
        graph_lines::{GraphLines, GraphLinesDrawer}, neural_network_drawer::NeuralNetworkDrawer, pendulum::{Pendulum, PendulumAction}, verlet_physics_drawer::VerletPhysicsDrawer,
    }, physics_simulation_v3_drawer::DrawerObjects, reinforcement_learning::neural_network_simd::NeuralNetworkSimd, triple_buffer, worker_thread,
};

pub const WATCH_POINTS_SIZE: usize = 10;
type Vec3 = cgmath::Vector3<f32>;

const INPUTS: usize = 5;
const OUTPUTS: usize = 2;
const NR_LAYERS: usize = 10;
const RESIDUAL: bool = true;

pub struct PendulumSimulation {
    // Physics
    ticks: u64,

    model: Box<NeuralNetworkSimd<INPUTS, OUTPUTS, NR_LAYERS, RESIDUAL>>,
    model_drawer: NeuralNetworkDrawer<INPUTS, OUTPUTS, NR_LAYERS, RESIDUAL>,
    graph_loss: GraphLines,
    graph_angle: GraphLines,
    graph_angle_vel: GraphLines,
    graph_cart: GraphLines,
    graph_cart_vel: GraphLines,

    graph_drawer_loss: GraphLinesDrawer,
    graph_drawer_angle: GraphLinesDrawer,
    graph_drawer_angle_vel: GraphLinesDrawer,
    graph_drawer_cart: GraphLinesDrawer,
    graph_drawer_cart_vel: GraphLinesDrawer,

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
        let pos_graph_loss = pos + Vec3::new(-2.0, 1.0, 1.0);
        let pos_pendulum = pos + Vec3::new(2.0, -0.5, 1.0);

        let pos_graph_angle =     pos + Vec3::new(2.2, 1.0, 2.2);
        let pos_graph_angle_vel = pos + Vec3::new(2.2, 1.0, 0.0);
        let pos_graph_cart =      pos + Vec3::new(4.4, 1.0, 2.2);
        let pos_graph_cart_vel =  pos + Vec3::new(4.4, 1.0, 0.0);


        let scale = 0.1;

        let model = Box::new(NeuralNetworkSimd::new());
        let model_drawer = NeuralNetworkDrawer::new(&model, scale, pos_model);

        // Debug
        let ups = Fps::new();
        let watch_ups = Watch::new();

        // Graph
        let graph_x: VecDeque<f32> = (0..100).map(|i| i as f32 * 0.1).collect();
        let graph_y: VecDeque<f32> = (0..100).map(|i| (i as f32 * 0.1).sin() * 10.0).collect();
        let graph_loss = GraphLines {x: graph_x.clone(), y: graph_y.clone()};

        let graph_angle = GraphLines {x: graph_x.clone(), y: graph_y.clone()};
        let graph_angle_vel = GraphLines {x: graph_x.clone(), y: graph_y.clone()};
        let graph_cart = GraphLines {x: graph_x.clone(), y: graph_y.clone()};
        let graph_cart_vel = GraphLines {x: graph_x.clone(), y: graph_y.clone()};

        let graph_drawer_loss = GraphLinesDrawer::new(scale, pos_graph_loss).color(to_rgb("#12d900"));
        let graph_drawer_angle = GraphLinesDrawer::new(scale, pos_graph_angle).color(to_rgb("#9c00d9"));
        let graph_drawer_angle_vel = GraphLinesDrawer::new(scale, pos_graph_angle_vel).color(to_rgb("#7700d9"));
        let graph_drawer_cart = GraphLinesDrawer::new(scale, pos_graph_cart).color(to_rgb("#0070d9"));
        let graph_drawer_cart_vel = GraphLinesDrawer::new(scale, pos_graph_cart_vel).color(to_rgb("#003dd9"));

        // Pendulum
        let pendulum = Pendulum::new();
        let verlet_physics_drawer = VerletPhysicsDrawer::new(
            &pendulum.verlet_physics, 
            scale,
            pos_pendulum,
        );


        Self {
            ticks: 0,

            model,
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
        }
    }

    pub fn update_physics(&mut self, _height_map: &impl HeightMapInterface) {
        let dt = 1.0 / 60.0;
        self.ticks += 1;

        self.watch_ups.start("Solver");
        let pendulum_state = self.pendulum.update(PendulumAction::None, dt);

        self.graph_angle.y_push_pop(pendulum_state.alpha);
        self.graph_angle_vel.y_push_pop(pendulum_state.angular_velocity);
        self.graph_cart.y_push_pop(pendulum_state.cart_pos);
        self.graph_cart_vel.y_push_pop(pendulum_state.cart_velocity);

        self.pendulum.update_verlet_physics(dt);
        self.watch_ups.stop();

        // ups
        let now = instant::Instant::now();
        let dt = now - self.last_render_time;
        self.last_render_time = now;
        self.ups.update(dt);
    }

    pub fn update_drawer(&mut self, objects: &mut DrawerObjects) {
        let nodes = &mut objects.genome_nodes;
        let edges = &mut objects.genome_edges;

        self.watch_ups.start("Draw Model");
        self.model_drawer.update(&self.model, nodes, edges);

        self.graph_drawer_loss.update(&self.graph_loss, edges);
        self.graph_drawer_angle.update(&self.graph_angle, edges);
        self.graph_drawer_angle_vel.update(&self.graph_angle_vel, edges);
        self.graph_drawer_cart.update(&self.graph_cart, edges);
        self.graph_drawer_cart_vel.update(&self.graph_cart_vel, edges);

        self.verlet_physics_drawer.update(&self.pendulum.verlet_physics, nodes, edges);

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
