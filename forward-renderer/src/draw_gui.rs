//! Combines some rendering interfaces using an orthographic projection

use wgpu_renderer::vertex_color_shader::VertexColorShaderDraw;
use wgpu_renderer::vertex_color_shader::vertex_color_shader_draw::VertexColorShaderDrawLines;
use wgpu_renderer::vertex_texture_shader::VertexTextureShaderDraw;

pub trait DrawGui:
    VertexColorShaderDraw + VertexColorShaderDrawLines + VertexTextureShaderDraw
{
}
