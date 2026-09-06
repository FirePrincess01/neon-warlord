//! Draws a lines graph

use forward_renderer::{particle_shader_two_point, to_rgb};

use crate::pendulum_simulation::Vec3;

pub struct GraphLines {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
}

pub struct GraphLinesDrawer {
    size: f32,
    color: Vec3,
    position: Vec3,

    // Grid settings
    grid_color: Vec3,
    grid_spacing: f32,
    grid_extent: f32,
}

impl GraphLinesDrawer {
    pub fn new(size: f32, position: Vec3) -> Self {
        let color = to_rgb("#c300d9");
        let grid_color = to_rgb("#333333");

        Self {
            size,
            color: color.into(),
            position,

            grid_color: grid_color.into(),
            grid_spacing: 2.0,
            grid_extent: 10.0,
        }
    }
    
pub fn update(
        &mut self,
        graph: &GraphLines,
        edges: &mut Vec<particle_shader_two_point::Instance>,
    ) {
        self.draw_grid(edges);
        self.draw_graph(graph, edges);
    }

    fn draw_grid(
        &self,
        edges: &mut Vec<particle_shader_two_point::Instance>,
    ) {
        let size = self.size * 0.02;

        let extent = self.grid_extent;
        let spacing = self.grid_spacing;

        let mut value = -extent;

        while value <= extent {
            // Vertical grid line (X axis)
            let p0 = self.position
                + Vec3::new(value, 0.0, -extent) * self.size;

            let p1 = self.position
                + Vec3::new(value, 0.0, extent) * self.size;

            edges.push(particle_shader_two_point::Instance {
                position_0: p0.into(),
                position_1: p1.into(),
                color: self.grid_color.into(),
                time: 1.0,
                size: size,
            });

            // Horizontal grid line (Y axis)
            let p0 = self.position
                + Vec3::new(-extent, 0.0, value) * self.size;

            let p1 = self.position
                + Vec3::new(extent, 0.0, value) * self.size;

            edges.push(particle_shader_two_point::Instance {
                position_0: p0.into(),
                position_1: p1.into(),
                color: self.grid_color.into(),
                time: 1.0,
                size: size,
            });

            value += spacing;
        }
    }

    fn draw_graph(
        &self,
        graph: &GraphLines,
        edges: &mut Vec<particle_shader_two_point::Instance>,
    ) {
        let size = self.size * 0.02;

        let count = graph.x.len().min(graph.y.len());

        if count < 2 {
            return;
        }

        for i in 0..count - 1 {
            let p0 = self.position
                + Vec3::new(graph.x[i], 0.0, graph.y[i]) * self.size;

            let p1 = self.position
                + Vec3::new(graph.x[i + 1], 0.0, graph.y[i + 1]) * self.size;

            edges.push(particle_shader_two_point::Instance {
                position_0: p0.into(),
                position_1: p1.into(),
                color: self.color.into(),
                time: 1.0,
                size: size,
            });
        }
    }
}