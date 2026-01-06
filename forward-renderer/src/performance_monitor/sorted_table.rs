//! The % CPU usage table of the performance monitor
//!

use cgmath::Zero;
use wgpu_renderer::{
    performance_monitor::watch::{self},
    shape::{self, MeshDataInterface},
    vertex_color_shader,
    vertex_texture_shader::TextureBindGroupLayout,
    wgpu_renderer::WgpuRendererInterface,
};

use super::sliding_average::SlidingAverage;

pub struct SortedTable<const SIZE: usize> {
    pub mesh_colors: Vec<vertex_color_shader::Mesh>,
    pub mesh_percent: Vec<wgpu_renderer::label::LabelMesh>,
    pub mesh_names: Vec<wgpu_renderer::label::LabelMesh>,

    names: Vec<&'static str>,
    names_new: Vec<&'static str>,
    averages: Vec<SlidingAverage>,

    label_percents: Vec<wgpu_renderer::label::Label>,
    label_names: Vec<wgpu_renderer::label::Label>,

    size: usize,

    _update_count: usize,
    update_index: usize,
}

impl<const SIZE: usize> SortedTable<SIZE> {
    pub fn new(
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        texture_bind_group_layout: &TextureBindGroupLayout,
        font: &rusttype::Font<'static>,
        average_len: usize,
        color_gradient: &[cgmath::Vector3<f32>],
        scale: f32,
        position: cgmath::Vector3<f32>,
    ) -> Self {
        let size = SIZE;

        let mut mesh_colors: Vec<vertex_color_shader::Mesh> = Vec::with_capacity(size);

        let mut averages: Vec<SlidingAverage> = Vec::with_capacity(size);
        let mut label_percents: Vec<wgpu_renderer::label::Label> = Vec::with_capacity(size);
        let mut mesh_percent: Vec<wgpu_renderer::label::LabelMesh> = Vec::with_capacity(size);

        let mut names: Vec<&'static str> = Vec::with_capacity(size);
        let mut label_names: Vec<wgpu_renderer::label::Label> = Vec::with_capacity(size);
        let mut mesh_names: Vec<wgpu_renderer::label::LabelMesh> = Vec::with_capacity(size);

        const MAX_LINES: usize = 10;
        let max_width = 20.0 * scale;

        // mesh_colors
        #[allow(clippy::needless_range_loop)]
        for i in 0..size {
            let square = shape::Square::new(scale - 5.0);
            let mesh = vertex_color_shader::Mesh::from_shape(
                wgpu_renderer.device(),
                square.triangles(),
                &color_gradient[i],
                &[vertex_color_shader::Instance {
                    position: cgmath::Vector3::new(
                        position.x + max_width * (i / MAX_LINES) as f32,
                        position.y + 2.5 + (i % MAX_LINES) as f32 * scale,
                        position.z,
                    ),
                    rotation: cgmath::Quaternion::zero(),
                }],
            );

            mesh_colors.push(mesh);
        }

        // mesh_percent
        let mut label_percent_with = 0;
        for i in 0..size {
            let label = wgpu_renderer::label::Label::new(font, scale, "00.00 ms");
            label_percent_with = label.width();
            let mesh = wgpu_renderer::label::LabelMesh::new(
                wgpu_renderer,
                label.get_image(),
                texture_bind_group_layout,
                &vertex_color_shader::Instance {
                    position: cgmath::Vector3::new(
                        position.x + scale + max_width * (i / MAX_LINES) as f32,
                        position.y + (i % MAX_LINES) as f32 * scale,
                        0.0,
                    ),
                    rotation: cgmath::Quaternion::zero(),
                },
            );

            averages.push(SlidingAverage::new(average_len));
            label_percents.push(label);
            mesh_percent.push(mesh);
        }

        // mesh_names
        for i in 0..size {
            let text = "....................";
            let label = wgpu_renderer::label::Label::new(font, scale, "....................");
            let mesh = wgpu_renderer::label::LabelMesh::new(
                wgpu_renderer,
                label.get_image(),
                texture_bind_group_layout,
                &vertex_color_shader::Instance {
                    position: cgmath::Vector3::new(
                        position.x
                            + scale
                            + label_percent_with as f32
                            + max_width * (i / MAX_LINES) as f32,
                        position.y + (i % MAX_LINES) as f32 * scale,
                        0.0,
                    ),
                    rotation: cgmath::Quaternion::zero(),
                },
            );

            names.push(text);
            label_names.push(label);
            mesh_names.push(mesh);
        }

        Self {
            mesh_colors,
            mesh_percent,
            mesh_names,
            names_new: names.clone(),
            names,
            averages,
            label_percents,
            label_names,
            size,
            _update_count: 0,
            update_index: 0,
        }
    }

    pub fn update_device(
        &mut self,
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        font: &rusttype::Font<'static>,
    ) {
        // if self.update_count % 1 == 0 {
        let i = self.update_index % self.size;
        self.update_index += 1;

        // mesh_percent
        // let average = 100.0 * self.averages[i].average() as f64 / 16666.0;
        // let average_str = format!("{:>4.1} %", average);

        let average = self.averages[i].average() as f64 / 1000.0;
        let average_str = format!("{:>4.2} ms", average);

        self.label_percents[i].update(font, average_str.as_str()); // this is an expensive operation
        self.mesh_percent[i]
            .update_texture(wgpu_renderer.queue(), self.label_percents[i].get_image());

        // mesh_names
        if self.names_new[i] != self.names[i] {
            self.names[i] = self.names_new[i];
            self.label_names[i].update(font, self.names[i]);
            self.mesh_names[i]
                .update_texture(wgpu_renderer.queue(), self.label_names[i].get_image());
        }
        // }
        // self.update_count += 1;
    }

    pub fn update_from_viewer_data(&mut self, data: &watch::WatchViewerData<SIZE>) {
        let _last_update_time = data.last_update_time;
        let _update_time = data.update_time;
        let watch_points = &data.watch_points;

        for (i, watch_point) in watch_points.iter().enumerate() {
            let duration = (watch_point.stop - watch_point.start).as_micros();

            self.names_new[i] = watch_point.name;
            self.averages[i].push(duration as u32);
        }
    }
}
