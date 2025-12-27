//! Interface to draw objects of this shader
//!

pub trait LodHeightMapShaderDraw {
    fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>);
}
