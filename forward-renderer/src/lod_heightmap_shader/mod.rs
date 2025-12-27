//! Deferred shader drawing a terrain height map
//!

mod lod_heightmap_shader_draw;
mod instance;
mod mesh;
mod pipeline;
mod vertex;

pub use lod_heightmap_shader_draw::LodHeightMapShaderDraw;
pub use mesh::Mesh;
pub use pipeline::Pipeline;

pub use wgpu_renderer::vertex_color_shader::IndexBuffer;

pub use wgpu_renderer::vertex_color_shader::VertexBuffer;
pub use vertex::Vertex;

pub use wgpu_renderer::vertex_color_shader::InstanceBuffer;
pub use instance::Instance;

pub use wgpu_renderer::vertex_texture_shader::Texture;
pub use wgpu_renderer::vertex_texture_shader::TextureBindGroupLayout;

pub use wgpu_renderer::vertex_heightmap_shader::Heightmap;
pub use wgpu_renderer::vertex_heightmap_shader::HeightmapBindGroupLayout;
pub use wgpu_renderer::vertex_heightmap_shader::HeightmapTexture;

// pub use super::deferred_color_shader::EntityBuffer;

// pub use crate::deferred_color_shader::GBuffer;

pub use wgpu_renderer::vertex_color_shader::CameraBindGroupLayout;
