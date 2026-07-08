//! Generates a tree with nodes an leafs

pub mod node;

use crate::verlet_physics::{self, VerletObject};
use wgpu_renderer::{
    vertex_color_shader::{self, VertexColorShaderDraw},
    wgpu_renderer::WgpuRendererInterface,
};

type Vec3 = cgmath::Vector3<f32>;


pub struct ProceduralTree {
    verlet_objects: Vec<VerletObject>,
    links: Vec<verlet_physics::link::Link>,
    fixed: Vec<verlet_physics::fixed::Fixed>,
    solver: verlet_physics::solver::Solver,

    nodes_mesh: vertex_color_shader::Mesh,
    nodes_instances: Vec<vertex_color_shader::Instance>,
    leafs_mesh: vertex_color_shader::Mesh,
    leafs_instances: Vec<vertex_color_shader::Instance>,
}

impl ProceduralTree {

    pub fn new() -> Self {
        let color_leaves = to_rgb("#4d944f");
        let color_nodes = to_rgb("#a5873c");


    }

    fn generate_tree(nr_nodes: usize) -> (Vec<Vec3>) {

    }


    pub fn update(&mut self, wgpu_renderer: &mut dyn WgpuRendererInterface) {
        let dt = 1.0 / 60.0;

        // physics
        self.solver
            .update(&mut self.verlet_objects, &self.links, &self.fixed, dt);

        // update positions
        for i in 0..self.verlet_objects.len() {
            let instance = &mut self.nodes_instances[i];
            let verlet_object = &mut self.verlet_objects[i];

            instance.position.x = verlet_object.position().x;
            instance.position.z = verlet_object.position().y;
        }

        // update device
        self.nodes_mesh
            .update_instance_buffer(wgpu_renderer.queue(), &self.nodes_instances);
    }
}

impl VertexColorShaderDraw for ProceduralTree {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.nodes_mesh.draw(render_pass);
    }
}

fn to_rgb(hex: &str) -> [f32; 3] {
    let hex = hex.trim_start_matches('#');

    assert!(
        hex.len() == 6,
        "Expected a 6-digit hex color like #RRGGBB"
    );

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap() as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap() as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap() as f32 / 255.0;

    [r, g, b]
}