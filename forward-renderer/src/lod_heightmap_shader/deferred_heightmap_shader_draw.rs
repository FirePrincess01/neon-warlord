//! Interface to draw objects of this shader
//!

pub trait DeferredHeightMapShaderDraw {
    fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>);
}
