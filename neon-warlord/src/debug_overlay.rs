//! Prints some debug info to the screen
//!

use cgmath::Zero;
use forward_renderer::DrawGui;
use wgpu_renderer::{
    vertex_color_shader::{
        VertexColorShaderDraw, vertex_color_shader_draw::VertexColorShaderDrawLines,
    },
    vertex_texture_shader::{self, VertexTextureShaderDraw},
    wgpu_renderer::WgpuRendererInterface,
};

struct Entry {
    label: wgpu_renderer::label::Label,
    mesh: wgpu_renderer::label::LabelMesh,
}

pub struct DebugOverlay<const SIZE: usize> {
    entries: [Entry; 10],

    height: u32,
    width: u32,
    scale_factor: f32,
}

impl<const SIZE: usize> DebugOverlay<SIZE> {
    fn create_entries(
        renderer: &mut dyn WgpuRendererInterface,
        texture_bind_group_layout: &vertex_texture_shader::TextureBindGroupLayout,
        font: &rusttype::Font<'static>,
        height: u32,
        _width: u32,
        scale_factor: f32,
    ) -> [Entry; 10] {
        let font_scale = 16.0 * scale_factor;

        let entries: [Entry; 10] = core::array::from_fn(|i| {
            let label = wgpu_renderer::label::Label::new(font, font_scale, "                              ");
            let mesh = wgpu_renderer::label::LabelMesh::new(
                renderer,
                label.get_image(),
                texture_bind_group_layout,
                &vertex_texture_shader::Instance {
                    position: cgmath::Vector3 {
                        x: 15.0,
                        y: height as f32
                            - font_scale
                            - 15.0
                            - i as f32 * (font_scale + 2.0 * scale_factor),
                        z: 0.0,
                    },
                    rotation: cgmath::Quaternion::zero(),
                },
            );

            Entry { label, mesh }
        });

        entries
    }

    pub fn new(
        renderer: &mut dyn WgpuRendererInterface,
        texture_bind_group_layout: &vertex_texture_shader::TextureBindGroupLayout,
        font: &rusttype::Font<'static>,
        height: u32,
        width: u32,
        scale_factor: f32,
    ) -> Self {
        let entries = Self::create_entries(
            renderer,
            texture_bind_group_layout,
            font,
            height,
            width,
            scale_factor,
        );

        Self {
            entries,
            height,
            width,
            scale_factor,
        }
    }

    pub fn update(
        &mut self,
        renderer: &mut dyn WgpuRendererInterface,
        font: &rusttype::Font<'static>,
        index: usize,
        name: &'static str,
        val: f32,
    ) {
        let text = format!("{}: {}", name, val);
        let entry = &mut self.entries[index];
        entry.label.update(font, &text);
        entry
            .mesh
            .update_texture(renderer.queue(), entry.label.get_image());
    }

    pub fn update_str(
        &mut self,
        renderer: &mut dyn WgpuRendererInterface,
        font: &rusttype::Font<'static>,
        index: usize,
        text: &String,
    ) {
        // let text = format!("{}: {}", name, val);
        let entry = &mut self.entries[index];
        entry.label.update(font, &text);
        entry
            .mesh
            .update_texture(renderer.queue(), entry.label.get_image());
    }

    pub fn resize(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        texture_bind_group_layout: &vertex_texture_shader::TextureBindGroupLayout,
        font: &rusttype::Font<'static>,
        height: u32,
        width: u32,
    ) {
        self.height = height;
        self.width = width;

        self.entries = Self::create_entries(
            renderer_interface,
            texture_bind_group_layout,
            font,
            height,
            width,
            self.scale_factor,
        )
    }

    pub fn rescale(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        texture_bind_group_layout: &vertex_texture_shader::TextureBindGroupLayout,
        font: &rusttype::Font<'static>,
        scale_factor: f32,
    ) {
        self.scale_factor = scale_factor;

        self.entries = Self::create_entries(
            renderer_interface,
            texture_bind_group_layout,
            font,
            self.height,
            self.width,
            scale_factor,
        )
    }
}

impl<const SIZE: usize> VertexTextureShaderDraw for DebugOverlay<SIZE> {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        for elem in &self.entries {
            elem.mesh.draw(render_pass);
        }
    }
}

impl<const SIZE: usize> VertexColorShaderDraw for DebugOverlay<SIZE> {
    fn draw<'a>(&'a self, _render_pass: &mut wgpu::RenderPass<'a>) {
        // nothing to do
    }
}

impl<const SIZE: usize> VertexColorShaderDrawLines for DebugOverlay<SIZE> {
    fn draw_lines<'a>(&'a self, _render_pass: &mut wgpu::RenderPass<'a>) {
        // nothing to do
    }
}

impl<const SIZE: usize> DrawGui for DebugOverlay<SIZE> {}
