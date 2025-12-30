//! Interface to draw objects of this shader
//!

pub trait ParticleShaderDraw {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}
