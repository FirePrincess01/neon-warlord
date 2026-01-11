//! Manages the data on the gpu of the particles
//!

use wgpu_renderer::{
    shape::{MeshDataInterface, UVSphere},
    wgpu_renderer::WgpuRendererInterface,
};

use crate::{geometry, particle_shader};

pub struct ParticleStorage {
    mesh: particle_shader::Mesh,
    instances: Vec<particle_shader::Instance>,

    _max_instances: usize,
}

impl ParticleStorage {
    pub fn new(renderer: &mut dyn WgpuRendererInterface, max_instances: usize) -> Self {
        let nr_particles = 100;

        let sphere = UVSphere::new(1.0, 6);
        let sphere_triangles = sphere.triangles(); // 96 positions
        println!("vertices: {}", sphere_triangles.positions.len());

        // let quad = geometry::Quad::new(1.0); // 4 positions

        let mut mesh_host = geometry::Mesh::new();
        for _i in 0..nr_particles {
            // mesh_host.add(&quad);
            mesh_host.add_triangles(sphere_triangles);
        }

        let mut instances = Vec::with_capacity(max_instances);
        for i in 0..max_instances {
            instances.push(particle_shader::Instance {
                position: [4.0 + i as f32 * (4.0), 7.0, 4.0],
                color: [0.01, 0.01, 0.01],
                time: 0.0,
                size: 0.1
            });
        }

        let mesh = particle_shader::Mesh::from_geometry(renderer.device(), &mesh_host, &instances);

        Self {
            mesh,
            instances,
            _max_instances: max_instances,
        }
    }

    pub fn set_position(&mut self, index: usize, pos: cgmath::Vector3<f32>)
    {
        self.instances[index].position = pos.into();
    }

    pub fn set_size(&mut self, index: usize, size: f32)
    {
        self.instances[index].size = size;
    }

    pub fn update(&mut self, renderer: &mut dyn WgpuRendererInterface, dt: instant::Duration) {
        let dt = dt.as_secs_f32() / 2.0;

        for elem in &mut self.instances {
            elem.time = elem.time + dt;
        }

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
