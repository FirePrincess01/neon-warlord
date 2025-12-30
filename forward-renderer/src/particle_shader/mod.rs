//! Deferred shader drawing a terrain height map
//!


// mod vertex;
mod instance;
mod mesh;
mod pipeline_particle;
mod particle_shader_draw;

pub use particle_shader_draw::ParticleShaderDraw;
pub use mesh::Mesh;
pub use pipeline_particle::PipelineParticle;
pub use instance::Instance;
// pub use vertex::Vertex;

pub use wgpu_renderer::vertex_color_shader::Vertex;
// pub use wgpu_renderer::vertex_color_shader::Instance;
pub use wgpu_renderer::vertex_color_shader::VertexBuffer;
pub use wgpu_renderer::vertex_color_shader::IndexBuffer;
pub use wgpu_renderer::vertex_color_shader::InstanceBuffer;
pub use wgpu_renderer::vertex_color_shader::CameraBindGroupLayout;


