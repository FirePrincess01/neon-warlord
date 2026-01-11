//! Manages the data on the gpu of the particles
//!

use wgpu_renderer::{
    shape::{MeshDataInterface, UVSphere},
    wgpu_renderer::WgpuRendererInterface,
};

use crate::{geometry, particle_shader};

pub struct ParticleStorage {
    mesh: particle_shader::Mesh,
    instances: [particle_shader::Instance; 1],
    time: f32,
}

impl ParticleStorage {
    pub fn new(renderer: &mut dyn WgpuRendererInterface) -> Self {
        let nr_particles = 25;

        let quad = geometry::Quad::new(1.0); // 4 positions

        let mut mesh_host = geometry::Mesh::new();
        for _i in 0..nr_particles {
            mesh_host.add(&quad);
        }

        let instances = [particle_shader::Instance {
            position: [0.0, 7.0, 1.0],
            color: [0.01, 0.01, 0.01],
            time: 0.0,
        }];
        let mesh = particle_shader::Mesh::from_geometry(renderer.device(), &mesh_host, &instances);

        Self {
            mesh,
            instances,
            time: 0.0,
        }
    }

    pub fn update(&mut self, renderer: &mut dyn WgpuRendererInterface, dt: instant::Duration) {
        self.time += dt.as_secs_f32() / 2.0;

        self.instances[0].time = self.time;
        // self.instances[1].time = self.time + 10.0;
        // self.instances[2].time = self.time + 20.0;

        // for instance in &mut self.instances {
        //     instance.time = self.time;
        // }

        self.mesh
            .update_instance_buffer(renderer.queue(), &self.instances);
    }
}

impl particle_shader::ParticleShaderDraw for ParticleStorage {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        // mesh data
        let mesh = &self.mesh;

        mesh.draw(render_pass);
    }
}
