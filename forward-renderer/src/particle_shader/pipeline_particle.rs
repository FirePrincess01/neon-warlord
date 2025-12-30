//! A general purpose pipeline using vertices, colors and instances
//!
//! Vertices and Colors are independently updatable
//! The implementation uses wgpu for rendering
//!

use wgpu_renderer::vertex_color_shader::CameraUniformBuffer;
use wgpu_renderer::vertex_color_shader::camera_bind_group_layout;
use wgpu_renderer::wgpu_renderer::depth_texture::DepthTexture;
use crate::particle_shader::ParticleShaderDraw;

use super::Instance;
use super::Vertex;

/// A general purpose shader using vertices, colors and an instance matrix
pub struct PipelineParticle {
    render_pipeline: wgpu::RenderPipeline,
}

impl PipelineParticle {
    pub fn new(
        device: &wgpu::Device,
        camera_bind_group_layout: &camera_bind_group_layout::CameraBindGroupLayout,
        surface_format: wgpu::TextureFormat,
    ) -> Self {
        let topology = wgpu::PrimitiveTopology::TriangleList;

        // Shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader_particle.wgsl").into()),
        });

        // PipelineParticle
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render PipelineParticle Layout"),
                bind_group_layouts: &[camera_bind_group_layout.get()],
                immediate_size: 0,
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render PipelineParticle"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    Vertex::desc(),
                    Instance::desc(),
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology, // wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // counter-clockwise direction
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DepthTexture::DEPTH_FORMAT,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            cache: None,
            multiview_mask: None,
        });

        Self { render_pipeline }
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);
    }

    pub fn draw<'a>(
        &self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera: &'a CameraUniformBuffer,
        mesh: &'a dyn ParticleShaderDraw,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        camera.bind(render_pass);
        mesh.draw(render_pass);
    }
}
