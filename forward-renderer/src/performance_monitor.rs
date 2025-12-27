//! Creates a nice graph with description for user defined watchpoints

mod sliding_average;
mod sorted_table;

use cgmath::Zero;
use wgpu_renderer::{
    performance_monitor::{self, watch},
    vertex_color_shader::{
        self, VertexColorShaderDraw, vertex_color_shader_draw::VertexColorShaderDrawLines,
    },
    vertex_texture_shader::{TextureBindGroupLayout, VertexTextureShaderDraw},
    wgpu_renderer::WgpuRendererInterface,
};

use crate::draw_gui::DrawGui;

struct Data<const SIZE: usize> {
    graph_host: performance_monitor::Graph<SIZE>,
    graph_device: vertex_color_shader::Mesh,

    // label_30fps: wgpu_renderer::label::LabelMesh,
    label_60fps: wgpu_renderer::label::LabelMesh,
    // label_120fps: wgpu_renderer::label::LabelMesh,
    table: sorted_table::SortedTable<SIZE>,
}

pub struct PerformanceMonitor<const SIZE: usize> {
    color_gradient: colorous::Gradient,
    indicator: &'static str,
    scale_factor: f32,

    data: Data<SIZE>,

    pub show: bool,
}

impl<const SIZE: usize> PerformanceMonitor<SIZE> {
    fn create_data(
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        texture_bind_group_layout: &TextureBindGroupLayout,
        font: &rusttype::Font<'static>,
        color_gradient: colorous::Gradient,
        indicator: &'static str,
        scale_factor: f32,
    ) -> Data<SIZE> {
        let graph_host = performance_monitor::Graph::new(color_gradient, scale_factor);

        let graph_device = vertex_color_shader::Mesh::new(
            wgpu_renderer.device(),
            graph_host.vertices.as_slice(),
            graph_host.colors.as_slice(),
            graph_host.indices.as_slice(),
            &[vertex_color_shader::Instance::zero()],
        );

        let font_scale = 14.0 * scale_factor;
        let label_60fps_host = wgpu_renderer::label::Label::new(font, font_scale, indicator);

        let label_60fps = wgpu_renderer::label::LabelMesh::new(
            wgpu_renderer,
            label_60fps_host.get_image(),
            texture_bind_group_layout,
            &vertex_color_shader::Instance {
                position: cgmath::Vector3::new(
                    graph_host.get_width() as f32 - label_60fps_host.width() as f32,
                    graph_host.get_height_60fps(),
                    0.0,
                ),
                rotation: cgmath::Quaternion::zero(),
            },
        );

        // create table
        let table = sorted_table::SortedTable::new(
            wgpu_renderer,
            texture_bind_group_layout,
            font,
            graph_host.get_nr_lines(),
            &graph_host.color_gradient(),
            font_scale,
            cgmath::Vector3 {
                x: graph_host.get_width() as f32 + 5.0,
                y: 10.0,
                z: 0.0,
            },
        );

        Data {
            graph_host,
            graph_device,
            label_60fps,
            table,
        }
    }

    pub fn new(
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        texture_bind_group_layout: &TextureBindGroupLayout,
        font: &rusttype::Font<'static>,
        color_gradient: colorous::Gradient,
        show: bool,
        indicator: &'static str,
        scale_factor: f32,
    ) -> Self {
        let data = Self::create_data(
            wgpu_renderer,
            texture_bind_group_layout,
            font,
            color_gradient,
            indicator,
            scale_factor,
        );

        Self {
            data,
            show,
            color_gradient,
            indicator,
            scale_factor,
        }
    }

    pub fn update_from_data(
        &mut self,
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        font: &rusttype::Font<'static>,
        data: &watch::WatchViewerData<SIZE>,
    ) {
        self.data.graph_host.update_from_viewer_data(data);
        self.data.table.update_from_viewer_data(data);

        if self.show {
            self.data.graph_device.update_vertex_buffer(
                wgpu_renderer.queue(),
                self.data.graph_host.vertices.as_slice(),
            );
            self.data.table.update_device(wgpu_renderer, font);
        }
    }

    pub fn rescale(
        &mut self,
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        texture_bind_group_layout: &TextureBindGroupLayout,
        font: &rusttype::Font<'static>,
        scale_factor: f32,
    ) {
        self.scale_factor = scale_factor;
        self.data = Self::create_data(
            wgpu_renderer,
            texture_bind_group_layout,
            font,
            self.color_gradient,
            self.indicator,
            scale_factor,
        );
    }
}

impl<const SIZE: usize> VertexColorShaderDraw for PerformanceMonitor<SIZE> {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.show {
            for elem in &self.data.table.mesh_colors {
                elem.draw(render_pass);
            }
        }
    }
}

impl<const SIZE: usize> VertexColorShaderDrawLines for PerformanceMonitor<SIZE> {
    fn draw_lines<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.show {
            self.data.graph_device.draw_lines(render_pass);
        }
    }
}

impl<const SIZE: usize> VertexTextureShaderDraw for PerformanceMonitor<SIZE> {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.show {
            self.data.label_60fps.draw(render_pass);

            for elem in &self.data.table.mesh_percent {
                elem.draw(render_pass);
            }

            for elem in &self.data.table.mesh_names {
                elem.draw(render_pass);
            }
        }
    }
}

impl<const SIZE: usize> DrawGui for PerformanceMonitor<SIZE> {}
