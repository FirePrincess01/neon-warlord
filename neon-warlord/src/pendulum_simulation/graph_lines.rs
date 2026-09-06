//! Draws a lines graph

use std::collections::VecDeque;

use forward_renderer::{particle_shader_two_point, to_rgb};

use crate::pendulum_simulation::Vec3;

pub struct GraphLines {
    pub x: VecDeque<f32>,
    pub y: VecDeque<f32>,
}

impl GraphLines {
    pub fn y_push_pop(&mut self, val: f32) {
        self.y.pop_front();
        self.y.push_back(val);
    }
}

pub struct GraphLinesDrawer {
    size: f32,
    color: Vec3,
    position: Vec3,

    x_lim_start: f32,
    x_lim_end: f32,
    y_lim_start: f32,
    y_lim_end: f32,

    // Grid settings
    grid_color: Vec3,
    grid_spacing: f32,
    grid_extent: f32,
}

impl GraphLinesDrawer {
    pub fn new(size: f32, position: Vec3) -> Self {
        let color = to_rgb("#c300d9");
        let grid_color = to_rgb("#333333");

        let x_lim_start = 0.0;
        let x_lim_end = 100.0;
        let y_lim_start = -1.0;
        let y_lim_end = 1.0;

        Self {
            size,
            color: color.into(),
            position,

            x_lim_start,
            x_lim_end,
            y_lim_start,
            y_lim_end,

            grid_color: grid_color.into(),
            grid_spacing: 2.0,
            grid_extent: 10.0,
        }
    }

    pub fn color(mut self, to_rgb: [f32; 3]) -> GraphLinesDrawer {
        self.color = to_rgb.into();
        self
    }

    pub fn y_lim(mut self, y_lim: f32) -> GraphLinesDrawer {
        self.y_lim_start = -y_lim;
        self.y_lim_end = y_lim;
        self
    }

    pub fn update(
        &mut self,
        graph: &GraphLines,
        edges: &mut Vec<particle_shader_two_point::Instance>,
    ) {
        self.draw_grid(edges);
        self.draw_graph(graph, edges);
    }

    fn draw_grid(&self, edges: &mut Vec<particle_shader_two_point::Instance>) {
        let size = self.size * 0.02;

        let extent = self.grid_extent;
        let spacing = self.grid_spacing;

        let mut value = -extent;

        while value <= extent {
            // Vertical grid line (X axis)
            let p0 = self.position + Vec3::new(value, 0.0, -extent) * self.size;

            let p1 = self.position + Vec3::new(value, 0.0, extent) * self.size;

            edges.push(particle_shader_two_point::Instance {
                position_0: p0.into(),
                position_1: p1.into(),
                color: self.grid_color.into(),
                time: 1.0,
                size,
            });

            // Horizontal grid line (Y axis)
            let p0 = self.position + Vec3::new(-extent, 0.0, value) * self.size;

            let p1 = self.position + Vec3::new(extent, 0.0, value) * self.size;

            edges.push(particle_shader_two_point::Instance {
                position_0: p0.into(),
                position_1: p1.into(),
                color: self.grid_color.into(),
                time: 1.0,
                size,
            });

            value += spacing;
        }
    }

    fn draw_graph(&self, graph: &GraphLines, edges: &mut Vec<particle_shader_two_point::Instance>) {
        let size = self.size * 0.05;

        let count = graph.x.len().min(graph.y.len());

        if count < 2 {
            return;
        }

        let x_range = self.x_lim_end - self.x_lim_start;
        let y_range = self.y_lim_end - self.y_lim_start;

        for i in 0..count - 1 {
            let x0 = (graph.x[i] - self.x_lim_start) / x_range;
            let y0 = (graph.y[i] - self.y_lim_start) / y_range;

            let x1 = (graph.x[i + 1] - self.x_lim_start) / x_range;
            let y1 = (graph.y[i + 1] - self.y_lim_start) / y_range;

            let x0 = x0 * self.grid_extent;
            let x1 = x1 * self.grid_extent;

            // Map [0, 1] -> [-grid_extent, grid_extent]
            let x0 = x0 * 2.0 * self.grid_extent - self.grid_extent;
            let y0 = y0 * 2.0 * self.grid_extent - self.grid_extent;

            let x1 = x1 * 2.0 * self.grid_extent - self.grid_extent;
            let y1 = y1 * 2.0 * self.grid_extent - self.grid_extent;

            let p0 = self.position + Vec3::new(x0, 0.0, y0) * self.size;
            let p1 = self.position + Vec3::new(x1, 0.0, y1) * self.size;

            edges.push(particle_shader_two_point::Instance {
                position_0: p0.into(),
                position_1: p1.into(),
                color: self.color.into(),
                time: 1.0,
                size,
            });
        }
    }
}
