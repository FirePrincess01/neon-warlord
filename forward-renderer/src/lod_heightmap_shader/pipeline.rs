//! Pipeline drawing a lod terrain height map
//!

use super::CameraBindGroupLayout;
use super::HeightmapBindGroupLayout;
use super::Instance;
use super::LodHeightMapShaderDraw;
use super::TextureBindGroupLayout;
use super::Vertex;
use wgpu_renderer::vertex_color_shader;
use wgpu_renderer::wgpu_renderer::depth_texture;

pub enum LightingModel {
    // no lighting
    none,
    // per vertex lighting
    gouraud,
}

pub struct Pipeline {
    render_pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub fn new(
        device: &wgpu::Device,
        camera_bind_group_layout: &CameraBindGroupLayout,
        texture_bind_group_layout: &TextureBindGroupLayout,
        heightmap_bind_group_layout: &HeightmapBindGroupLayout,
        surface_format: wgpu::TextureFormat,
        lgithting: &LightingModel
    ) -> Self {
        let lighting = LightingModel::none;
        
        // Shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader_heightmap.wgsl").into()),
        });

        // Pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Lod Heigtmap Pipeline Layout"),
                bind_group_layouts: &[
                    camera_bind_group_layout.get(),
                    texture_bind_group_layout.get(),
                    heightmap_bind_group_layout.get(),
                ],
                immediate_size: 0,
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Lod Heigtmap Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc(), Instance::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: match lighting {
                    LightingModel::none => Some("fs_main") ,
                    LightingModel::gouraud => Some("vs_main_gouraud") ,
                },
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // counter-clockwise direction
                cull_mode: Some(wgpu::Face::Back),
                // cull_mode: None,
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth_texture::DepthTexture::DEPTH_FORMAT,
                depth_write_enabled: true,
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

    pub fn draw<'a>(
        &self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera: &'a vertex_color_shader::CameraUniformBuffer,
        mesh: &'a mut dyn LodHeightMapShaderDraw,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        camera.bind(render_pass);
        mesh.draw(render_pass);
    }
}
