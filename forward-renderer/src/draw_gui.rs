//! Combines some rendering interfaces using an orthographic porjection

use wgpu_renderer::{
    vertex_color_shader::{
        VertexColorShaderDraw, vertex_color_shader_draw::VertexColorShaderDrawLines,
    },
    vertex_texture_shader::VertexTextureShaderDraw,
};

pub trait DrawGui:
    VertexColorShaderDraw + VertexColorShaderDrawLines + VertexTextureShaderDraw
{
}
