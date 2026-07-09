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
    leaves_mesh: vertex_color_shader::Mesh,
    leaves_instances: Vec<vertex_color_shader::Instance>,

    index: usize,
    size: usize,

    nodes_indices: Vec<usize>,
    leaves_indices: Vec<usize>,
}

impl ProceduralTree {

    pub fn new(
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        verlet_objects: &mut Vec<VerletObject>,
        fixed_links: &mut Vec<verlet_physics::fixed_link::FixedLink>,

    ) -> Self {
        let leaves_color_0 = to_rgb("#4b964f");
        let leaves_color_1 = to_rgb("#0a340c");
        let nodes_color_0 = to_rgb("#836e37");
        let nodes_color_1 = to_rgb("#2b1f03");

        let tree = Tree::new(7, rand::random());
        let nr_nodes = tree.nr_nodes();
        let nr_leaves = tree.nr_leaves();

        let radius = 0.1;
        let nodes_circle = geometry::Circle::new_color_fade(radius, 32, nodes_color_0, nodes_color_1);
        let leaves_circle = geometry::Circle::new_color_fade(radius, 32, leaves_color_0, leaves_color_1);

        let instance = vertex_color_shader::Instance {
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            rotation: cgmath::Quaternion::from_angle_x(cgmath::Deg(90.0)),
        };

        let mut leaves_instances = Vec::with_capacity(nr_leaves);
        for _i in 0..nr_leaves {
            leaves_instances.push(instance);
        }

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

        let leaves_mesh = vertex_color_shader::Mesh::new(
            wgpu_renderer.device(),
            &leaves_circle.vertices,
            &leaves_circle.colors,
            &leaves_circle.indices,
            &leaves_instances,
        );

        // create verlet objects
        let size = tree.size();
        let index = verlet_objects.len();
        for i in 0..size {
            verlet_objects.push(VerletObject::new(Vec3::zero(), radius));
        }

        let mut nodes_indices = Vec::new();
        let mut leaves_indices = Vec::new();
        let mut create_links = CreateLinks {
            verlet_objects,
            fixed_links,
            index,
            nodes_indices: &mut nodes_indices,
            leaves_indices: &mut leaves_indices,
        };
        tree.traverse_tree(&mut create_links);

        Self{
            tree,
            nodes_instances,
            nodes_mesh,
            leaves_instances,
            leaves_mesh,
            index,
            size,

            nodes_indices,
            leaves_indices,
        }
    }

    // fn generate_tree(nr_nodes: usize) -> (Vec<Vec3>) {

    // }


    pub fn update(&mut self, wgpu_renderer: &mut dyn WgpuRendererInterface, verlet_objects: &[VerletObject]) {
        let dt = 1.0 / 60.0;

        // update nodes
        for (i, index) in self.nodes_indices.iter().enumerate() {
            let instance = &mut self.nodes_instances[i];
            let verlet_object = &verlet_objects[*index];

            instance.position.x = verlet_object.position().x;
            instance.position.y = verlet_object.position().y;
            instance.position.z = verlet_object.position().z;
        }

        // update leaves
        for (i, index) in self.leaves_indices.iter().enumerate() {
            let instance = &mut self.leaves_instances[i];
            let verlet_object = &verlet_objects[*index];

            instance.position.x = verlet_object.position().x;
            instance.position.y = verlet_object.position().y;
            instance.position.z = verlet_object.position().z;
        }

        // copy data to device
        self.nodes_mesh
            .update_instance_buffer(wgpu_renderer.queue(), &self.nodes_instances);

        self.leaves_mesh
            .update_instance_buffer(wgpu_renderer.queue(), &self.leaves_instances);
    }
}

impl VertexColorShaderDraw for ProceduralTree {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.nodes_mesh.draw(render_pass);
        self.leaves_mesh.draw(render_pass);
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

    pub nodes_indices: &'a mut Vec<usize>,
    pub leaves_indices: &'a mut Vec<usize>,
}

impl<'a> TreeInterface for CreateLinks<'a> {
    fn create_node(&mut self, 
        node_index: usize, 
        parent_index: Option<usize>,  
        absolute_position: &Vec3, 
        relative_position: &Vec3, 
        is_leave: bool
    ) {
        let node_index = self.index + node_index;
        // println!("node_index {node_index}, is_leave {is_leave}");

        self.verlet_objects[node_index].reset_position(*absolute_position);

        if let Some(parent_index) = parent_index {
            let parent_index = self.index + parent_index;
            self.fixed_links.push(FixedLink::new(parent_index, node_index, *relative_position));

            if is_leave {
                self.leaves_indices.push(node_index);
            } else {
                self.nodes_indices.push(node_index);
            }
        }

    }
}