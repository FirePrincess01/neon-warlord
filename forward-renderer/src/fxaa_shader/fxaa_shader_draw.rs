//! Interface to draw objects of this shader
//!

pub trait FxaaShaderDraw {
    fn draw_fxaa<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}
