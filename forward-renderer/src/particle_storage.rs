//! Manages the data on the gpu of the particles
//!

use wgpu_renderer::wgpu_renderer::WgpuRendererInterface;

use crate::{geometry, particle_shader};

pub struct ParticleStorage {
    mesh: particle_shader::Mesh,
    instance: particle_shader::Instance,
}

impl ParticleStorage {
    pub fn new(renderer: &mut dyn WgpuRendererInterface) -> Self {
        let nr_particles = 1;

        let quad = geometry::Quad::new(1.0);
        let instance = particle_shader::Instance{
            position: [0.0, 7.0, 1.0],
            color: [1.0, 1.0, 1.0],
            time: 0.0,
        };
        let mesh =
            particle_shader::Mesh::new(renderer.device(), &quad.vertices, &quad.indices, &[instance]);

        Self {
            mesh,
            instance,
        }
    }
}

impl particle_shader::ParticleShaderDraw for ParticleStorage {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        // mesh data
        let mesh = &self.mesh;

        mesh.draw(render_pass);
    }
}
