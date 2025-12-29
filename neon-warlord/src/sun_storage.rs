//! Holds the sun

use cgmath::Rotation3;
use forward_renderer::geometry;
use wgpu_renderer::vertex_color_shader::{self, VertexColorShaderDraw};

pub struct SunStorage {
    mesh: vertex_color_shader::Mesh,
}

impl SunStorage {
    pub fn new(
        wgpu_renderer: &mut dyn wgpu_renderer::wgpu_renderer::WgpuRendererInterface,
    ) -> Self {
        let circle = geometry::Circle::new_color_fade(200.0, 32, [1.0, 1.0, 0.0], [1.0, 0.0, 0.0]);

        let mesh = vertex_color_shader::Mesh::new(
            wgpu_renderer.device(),
            &circle.vertices,
            &circle.colors,
            &circle.indices,
            &[vertex_color_shader::Instance {
                position: cgmath::Vector3::new(0.0, 1000.0, 140.0),
                rotation: cgmath::Quaternion::from_angle_x(cgmath::Deg(90.0)),
            }],
        );

        Self { mesh }
    }
}

impl VertexColorShaderDraw for SunStorage {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.mesh.draw(render_pass);
    }
}
