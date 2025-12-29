//! Creates vertex data to draw a circle
//!

use std::f32::consts::PI;

use wgpu_renderer::vertex_color_shader::Color;
use wgpu_renderer::vertex_color_shader::Vertex;

pub struct Circle {
    pub vertices: Vec<Vertex>,
    pub colors: Vec<Color>,
    pub indices: Vec<u32>,
}

impl Circle {
    pub fn new(r: f32, n: usize) -> Self {
        Self::new_color_fade(r, n, [1.0, 0.0, 1.0], [0.0, 1.0, 1.0])
    }

    pub fn new_color_fade(r: f32, n: usize, color0: [f32; 3], color1: [f32; 3]) -> Self {
        let mut vertices = Vec::<Vertex>::new();
        let mut colors = Vec::<Color>::new();
        let mut indices = Vec::<u32>::new();

        let color0 = cgmath::Vector3::from(color0);
        let color1 = cgmath::Vector3::from(color1);

        // let mut deferred_vertices = Vec::<deferred_color_shader::Vertex>::new();

        let z = 0.00;
        vertices.push(Vertex {
            position: [0.0, 0.0, z],
        }); // center

        colors.push(Color {
            color: (0.4 * color0 + 0.5 * color1).into(),
        }); // center

        let angle = 2.0 * PI / n as f32;
        for i in 0..n + 1 {
            let x = r * f32::cos(angle * i as f32);
            let y = r * f32::sin(angle * i as f32);

            let dist_y = 0.5 * (1.0 + f32::sin(angle * i as f32));

            let color = color0 * dist_y + color1 * (1.0 - dist_y);

            vertices.push(Vertex {
                position: [x, y, z],
            });

            colors.push(Color {
                color: color.into(),
            });
        }

        // for _i in 0..n {
        //     colors.push(color);
        // }

        for i in 1..n {
            indices.push(0);
            indices.push(i as u32);
            indices.push((i + 1) as u32);
        }

        indices.push(0);
        indices.push((n) as u32);
        indices.push(1_u32);

        Self {
            vertices,
            colors,
            indices,
        }
    }
}
