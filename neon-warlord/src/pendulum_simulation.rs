//! Simulates an inverted pendulum

mod graph_lines;
mod neural_network_drawer;

use forward_renderer::height_map::HeightMapInterface;
use wgpu_renderer::performance_monitor::{Fps, watch::Watch};

use crate::{
    pendulum_simulation::{
        graph_lines::{GraphLines, GraphLinesDrawer},
        neural_network_drawer::NeuralNetworkDrawer,
    },
    physics_simulation_v3_drawer::DrawerObjects,
    reinforcement_learning::neural_network_simd::NeuralNetworkSimd,
    triple_buffer, worker_thread,
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
    graph: GraphLines,
    graph_drawer: GraphLinesDrawer,

    // Debug
    ups: Fps,
    last_render_time: instant::Instant,
    watch_ups: Watch<WATCH_POINTS_SIZE>,
}

impl PendulumSimulation {
    pub fn new() -> Self {
        // agent 0
        let pos = Vec3::new(0.0, 0.0, 2.0);
        let scale = 0.1;

        let model = Box::new(NeuralNetworkSimd::new());
        let model_drawer = NeuralNetworkDrawer::new(&model, scale, pos);

        // Debug
        let ups = Fps::new();
        let watch_ups = Watch::new();

        let graph_x: Vec<f32> = (0..100).map(|i| i as f32 * 0.1).collect();
        let graph_y: Vec<f32> = (0..100).map(|i| (i as f32 * 0.1).sin() * 10.0).collect();
        let graph = GraphLines {
            x: graph_x,
            y: graph_y,
        };
        let graph_drawer = GraphLinesDrawer::new(scale, pos + Vec3::new(-2.0, 1.0, 1.0));

        Self {
            ticks: 0,

            model,
            model_drawer,
            graph,
            graph_drawer,

            ups,
            last_render_time: instant::Instant::now(),
            watch_ups,
        }
    }

    pub fn update_physics(&mut self, _height_map: &impl HeightMapInterface) {
        let _dt = 1.0 / 60.0;
        self.ticks += 1;

        // self.watch_ups.stop();

        // self.watch_ups.start("Solver");
        // self.watch_ups.stop();

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

        self.graph_drawer.update(&self.graph, edges);

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
