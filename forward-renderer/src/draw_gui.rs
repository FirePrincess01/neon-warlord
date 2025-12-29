//! Combines some rendering interfaces using an orthographic projection

use wgpu_renderer::vertex_texture_shader::VertexTextureShaderDraw;
use wgpu_renderer::vertex_color_shader::vertex_color_shader_draw::VertexColorShaderDrawLines;
use wgpu_renderer::vertex_color_shader::VertexColorShaderDraw;

pub trait DrawGui:
    VertexColorShaderDraw + VertexColorShaderDrawLines + VertexTextureShaderDraw
{
}
