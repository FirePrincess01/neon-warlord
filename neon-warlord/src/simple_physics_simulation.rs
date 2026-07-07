//! Draws some objects using verlet physics

use forward_renderer::geometry;
use wgpu_renderer::{vertex_color_shader::{self, VertexColorShaderDraw}, wgpu_renderer::WgpuRendererInterface};
use cgmath::{Rotation3, Vector2};

use crate::verlet_physics::{self, VerletObject};

pub struct SimplePhysicsSimulation {
    verlet_objects: [VerletObject; 1],
    solver: verlet_physics::solver::Solver,

    mesh: vertex_color_shader::Mesh,
    instances: [vertex_color_shader::Instance; 1] 
}

impl SimplePhysicsSimulation {
    pub fn new(
        wgpu_renderer: &mut dyn WgpuRendererInterface
    ) -> Self {
        let verlet_objects = [
            VerletObject::new(Vector2::new(0.0, 10.0)),
            // VerletObject::new(Vector2::new(0.0, 11.0)),
            // VerletObject::new(Vector2::new(0.0, 12.0)),
            // VerletObject::new(Vector2::new(0.0, 13.0)),

            // VerletObject::new(Vector2::new(0.0, 14.0)),
            // VerletObject::new(Vector2::new(0.0, 15.0)),
            // VerletObject::new(Vector2::new(0.0, 16.0)),
            // VerletObject::new(Vector2::new(0.0, 17.0)),
            ];

        let circle = geometry::Circle::new_color_fade(0.5, 32, [0.0, 0.4, 0.4], [0.4, 0.0, 0.4]);

        let instance = vertex_color_shader::Instance {
                position: cgmath::Vector3::new(0.0, 10.0, 2.0),
                rotation: cgmath::Quaternion::from_angle_x(cgmath::Deg(90.0)),
            };

        let instances = [
            instance, 
            // instance,
            // instance,
            // instance,
            // instance,
            // instance,
            // instance,
            // instance,
            ];

        let mesh = vertex_color_shader::Mesh::new(
            wgpu_renderer.device(),
            &circle.vertices,
            &circle.colors,
            &circle.indices,
            &instances,
        );

        let solver = verlet_physics::solver::Solver::new();

        Self { 
            verlet_objects,
            solver, 
            mesh, 
            instances 
        }
    }
    
    pub fn update(&mut self, wgpu_renderer: &mut dyn WgpuRendererInterface) {
        let dt = 1.0 / 60.0;

        self.solver.update(&mut self.verlet_objects, dt);

        for i in 0..self.verlet_objects.len() {
            let instance = &mut self.instances[i];
            let verlet_object = &mut self.verlet_objects[i];

            instance.position.x = verlet_object.position().x;
            instance.position.z = verlet_object.position().y;
        }

        self.mesh.update_instance_buffer(wgpu_renderer.queue(), &self.instances);
    }

}


impl VertexColorShaderDraw for SimplePhysicsSimulation {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.mesh.draw(render_pass);
    }
}