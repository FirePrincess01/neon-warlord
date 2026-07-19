//! Draws an object in 3D

use forward_renderer::{geometry, to_rgb};
use wgpu_renderer::{vertex_color_shader::{self, VertexColorShaderDraw}, wgpu_renderer::WgpuRendererInterface};
use crate::verlet_physics::verlet_composition::VerletComposition;
use cgmath::{Rotation3, Zero};

pub struct AgentDrawer {
    nodes_mesh: vertex_color_shader::Mesh,
    nodes_instances: Vec<vertex_color_shader::Instance>,
}

impl AgentDrawer {
    pub fn new(
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        composition: &VerletComposition
    ) -> Self {
        let radius = 0.1;
        let nodes_color_0 = to_rgb("#d8b0e8");
        let nodes_color_1 = to_rgb("#300c36");

        let nodes_circle =
            geometry::Circle::new_color_fade(radius, 32, nodes_color_0, nodes_color_1);

        let instance = vertex_color_shader::Instance {
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            rotation: cgmath::Quaternion::from_angle_x(cgmath::Deg(90.0)),
        };

        let nr_nodes = composition.verlet_objects.len();

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

        Self {
            nodes_mesh,
            nodes_instances,
        }
    }

    pub fn update(
        &mut self,
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        composition: &VerletComposition,
    ) {
        let size = std::cmp::min(self.nodes_instances.len(), composition.verlet_objects.len());
        
        // copy from physics to model
        for i in 0..size {
            let instance = &mut self.nodes_instances[i];
            let verlet_object = &composition.verlet_objects[i];

            instance.position = verlet_object.position();
        }

        // copy from model to device
        self.nodes_mesh
            .update_instance_buffer(wgpu_renderer.queue(), &self.nodes_instances);

    }
}

impl VertexColorShaderDraw for AgentDrawer {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.nodes_mesh.draw(render_pass);
    }
}