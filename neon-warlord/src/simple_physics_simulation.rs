//! Draws some objects using verlet physics

use cgmath::{Rotation3, Vector2};
use forward_renderer::geometry;
use wgpu_renderer::{
    vertex_color_shader::{self, VertexColorShaderDraw},
    wgpu_renderer::WgpuRendererInterface,
};

use crate::verlet_physics::{self, VerletObject};

pub struct SimplePhysicsSimulation {
    verlet_objects: Vec<VerletObject>,
    links: Vec<verlet_physics::link::Link>,
    fixed: Vec<verlet_physics::fixed::Fixed>,
    solver: verlet_physics::solver::Solver,

    mesh: vertex_color_shader::Mesh,
    instances: Vec<vertex_color_shader::Instance>,

    radius: f32,

    ticks: u64,
}

impl SimplePhysicsSimulation {
    pub fn new(wgpu_renderer: &mut dyn WgpuRendererInterface) -> Self {
        let nr_objects = 1000;
        let radius = 0.1;

        let mut verlet_objects = Vec::with_capacity(nr_objects);

        let nr_links = 40;
        let mut links = Vec::with_capacity(nr_links);
        for i in 0..nr_links {
            verlet_objects.push(VerletObject::new(
                Vector2::new(i as f32 * 0.2 - 5.0, 15.0 - i as f32 * 0.2),
                radius,
            ));

            if i < nr_links - 1 {
                links.push(verlet_physics::link::Link::new(i, i + 1, 0.2));
            }
        }

        let fixed = vec![verlet_physics::fixed::Fixed::new(
            0,
            Vector2::new(-5.0, 15.0),
        )];

        let circle = geometry::Circle::new_color_fade(radius, 32, [0.0, 0.4, 0.4], [0.4, 0.0, 0.4]);

        let instance = vertex_color_shader::Instance {
            position: cgmath::Vector3::new(0.0, 10.0, 20.0),
            rotation: cgmath::Quaternion::from_angle_x(cgmath::Deg(90.0)),
        };

        let mut instances = Vec::with_capacity(nr_objects);
        for _i in 0..nr_objects {
            instances.push(instance);
        }

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
            links,
            fixed,
            solver,
            mesh,
            instances,
            radius,
            ticks: 0,
        }
    }

    pub fn update(&mut self, wgpu_renderer: &mut dyn WgpuRendererInterface) {
        let dt = 1.0 / 60.0;
        self.ticks += 1;

        if self.verlet_objects.len() < self.instances.len() && self.ticks.is_multiple_of(8) {
            self.verlet_objects
                .push(VerletObject::new(Vector2::new(0.0, 15.0), self.radius));
        }

        self.solver
            .update(&mut self.verlet_objects, &self.links, &self.fixed, dt);

        for i in 0..self.verlet_objects.len() {
            let instance = &mut self.instances[i];
            let verlet_object = &mut self.verlet_objects[i];

            instance.position.x = verlet_object.position().x;
            instance.position.z = verlet_object.position().y;
        }

        self.mesh
            .update_instance_buffer(wgpu_renderer.queue(), &self.instances);
    }
}

impl VertexColorShaderDraw for SimplePhysicsSimulation {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.mesh.draw(render_pass);
    }
}
