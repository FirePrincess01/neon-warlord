//! Deferred shader drawing animated objects
//!

mod animation_shader_draw;
mod mesh;
mod pipeline_animation;

mod animation_bind_group_layout;
mod animation_uniform;
mod animation_uniform_buffer;
mod instance;
mod vertex;

pub use pipeline_animation::LightingModel;
pub use animation_bind_group_layout::AnimationBindGroupLayout;
pub use animation_shader_draw::AnimationShaderDraw;
pub use animation_uniform::AnimationUniform;
pub use animation_uniform_buffer::AnimationUniformBuffer;
pub use mesh::Mesh;
pub use pipeline_animation::Pipeline;
pub use vertex::Vertex;

pub use instance::Instance;
pub use wgpu_renderer::vertex_color_shader::InstanceBuffer;
pub use wgpu_renderer::vertex_color_shader::VertexBuffer;

pub use wgpu_renderer::vertex_color_shader::IndexBuffer;

pub use wgpu_renderer::vertex_color_shader::CameraBindGroupLayout;
