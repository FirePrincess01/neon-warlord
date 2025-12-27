//! Creates vertex data to draw a circle
//!

use std::f32::consts::PI;

use wgpu_renderer::vertex_color_shader::Color;
use wgpu_renderer::vertex_color_shader::Vertex;

// use crate::deferred_color_shader;

#[allow(dead_code)]
pub struct Circle {
    pub vertices: Vec<Vertex>,
    pub colors: Vec<Color>,
    pub indices: Vec<u16>,
    // pub deferred_vertices: Vec<deferred_color_shader::Vertex>,
}

#[allow(dead_code)]
impl Circle {
    pub fn new(r: f32, n: usize) -> Self {
        let mut vertices = Vec::<Vertex>::new();
        let mut colors = Vec::<Color>::new();
        let mut indices = Vec::<u16>::new();

        // let mut deferred_vertices = Vec::<deferred_color_shader::Vertex>::new();

        let z = 0.01;
        vertices.push(Vertex {
            position: [0.0, 0.0, z],
        }); // center

        let angle = 2.0 * PI / n as f32;
        for i in 0..n + 1 {
            vertices.push(Vertex {
                position: [
                    r * f32::cos(angle * i as f32),
                    r * f32::sin(angle * i as f32),
                    z,
                ],
            });
        }

        let color = Color {
            color: [0.5, 0.5, 0.5],
        };
        colors.push(color); // center
        for _i in 0..n {
            colors.push(color);
        }

        for i in 1..n {
            indices.push(0);
            indices.push(i as u16);
            indices.push((i + 1) as u16);
        }

        indices.push(0);
        indices.push((n) as u16);
        indices.push(1_u16);

        // let normal = [0.0, 0.0, 1.0];
        // for vertex in &vertices {
        // deferred_vertices.push(deferred_color_shader::Vertex {
        //     position: vertex.position,
        //     normal,
        // })

        Self {
            vertices,
            colors,
            indices,
            // deferred_vertices,
        }
    }
}
