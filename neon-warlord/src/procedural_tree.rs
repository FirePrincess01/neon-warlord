//! Generates a tree with nodes an leafs

pub mod node;
pub mod tree;
pub mod leaf;

use crate::{procedural_tree::tree::{Tree, TreeInterface}, verlet_physics::{self, VerletObject, fixed_link::FixedLink}};
use forward_renderer::geometry;
use rand::rng;
use wgpu_renderer::{
    vertex_color_shader::{self, VertexColorShaderDraw},
    wgpu_renderer::WgpuRendererInterface,
};
use cgmath::{Rotation3, Zero};

type Vec3 = cgmath::Vector3<f32>;


pub struct ProceduralTree {
    // verlet_objects: Vec<VerletObject>,
    // links: Vec<verlet_physics::link::Link>,
    // fixed_links: Vec<verlet_physics::fixed_link::FixedLink>,
    // fixed: Vec<verlet_physics::fixed::Fixed>,
    // solver: verlet_physics::solver::Solver,

    tree: Tree,

    nodes_instances: Vec<vertex_color_shader::Instance>,
    nodes_mesh: vertex_color_shader::Mesh,
    // leafs_mesh: vertex_color_shader::Mesh,
    // leafs_instances: Vec<vertex_color_shader::Instance>,

    index: usize,
    size: usize,
}

impl ProceduralTree {

    pub fn new(
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        verlet_objects: &mut Vec<VerletObject>,
        fixed_links: &mut Vec<verlet_physics::fixed_link::FixedLink>,

    ) -> Self {
        let leaves_color_0 = to_rgb("#5ea762");
        let leaves_color_1 = to_rgb("#164918");
        let nodes_color_0 = to_rgb("#a89156");
        let nodes_color_1 = to_rgb("#43320b");

        let tree = Tree::new(5, rand::random());
        let nr_nodes = tree.size();

        let radius = 0.1;
        let leaves_circle = geometry::Circle::new_color_fade(radius, 32, nodes_color_0, nodes_color_1);
        let nodes_circle = geometry::Circle::new_color_fade(radius, 32, nodes_color_0, nodes_color_1);

        let instance = vertex_color_shader::Instance {
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            rotation: cgmath::Quaternion::from_angle_x(cgmath::Deg(90.0)),
        };

        let mut nodes_instances = Vec::with_capacity(nr_nodes);
        for _i in 0..nr_nodes {
            nodes_instances.push(instance);
        }

        let nodes_mesh = vertex_color_shader::Mesh::new(
            wgpu_renderer.device(),
            &nodes_circle.vertices,
            &nodes_circle.colors,
            &nodes_circle.indices,
            &nodes_instances,
        );

        // create verlet objects
        let size = nr_nodes;
        let index = verlet_objects.len();
        for i in 0..size {
            verlet_objects.push(VerletObject::new(Vec3::zero(), radius));
        }

        let mut create_links = CreateLinks {
            verlet_objects,
            fixed_links,
            index,
        };
        tree.traverse_tree(&mut create_links);

        Self{
            tree,
            nodes_instances,
            nodes_mesh,
            index,
            size,
        }
    }

    // fn generate_tree(nr_nodes: usize) -> (Vec<Vec3>) {

    // }


    pub fn update(&mut self, wgpu_renderer: &mut dyn WgpuRendererInterface, verlet_objects: &[VerletObject]) {
        let dt = 1.0 / 60.0;

        for i in 0..self.size{
            let instance = &mut self.nodes_instances[i];
            let verlet_object = &verlet_objects[self.index + i];

            instance.position.x = verlet_object.position().x;
            instance.position.y = verlet_object.position().y;
            instance.position.z = verlet_object.position().z;
        }

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

struct CreateLinks<'a> {
    verlet_objects: &'a mut Vec<VerletObject>,
    fixed_links: &'a mut Vec<verlet_physics::fixed_link::FixedLink>,
    index: usize,
}

impl<'a> TreeInterface for CreateLinks<'a> {
    fn create_node(
        &mut self, 
        node_index: usize, 
        parent_index: Option<usize>, 
        pos: &Vec3, 
        is_leave: bool
    ) {
        let node_index = self.index + node_index;
        println!("node_index {node_index}, is_leave {is_leave}");

        self.verlet_objects[node_index].reset_position(*pos);

        if let Some(parent_index) = parent_index {
            let parent_index = self.index + parent_index;
            self.fixed_links.push(FixedLink::new(parent_index, node_index, *pos));
        }

    }
}