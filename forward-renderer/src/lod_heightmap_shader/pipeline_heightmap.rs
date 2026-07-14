//! Pipeline drawing a lod terrain height map
//!

use super::CameraBindGroupLayout;
use super::HeightmapBindGroupLayout;
use super::Instance;
use super::LodHeightMapShaderDraw;
use super::TextureBindGroupLayout;
use super::Vertex;
use wgpu_renderer::vertex_color_shader;
use wgpu_renderer::wgpu_renderer::WgpuRendererInterface;
use wgpu_renderer::wgpu_renderer::depth_texture;
use wgpu_renderer::wgpu_renderer::depth_texture::DepthTexture;
use wgpu_renderer::wgpu_renderer::depth_texture_bind_group_layout;
use wgpu_renderer::wgpu_renderer::depth_texture_bind_group_layout::DepthTextureBindGroupLayout;

pub enum LightingModel {
    // no lighting
    None,
    // per vertex lighting
    Gouraud,
}

pub struct Pipeline {
    render_pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub fn new(
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        camera_bind_group_layout: &CameraBindGroupLayout,
        texture_bind_group_layout: &TextureBindGroupLayout,
        heightmap_bind_group_layout: &HeightmapBindGroupLayout,
        shadow_map_bind_group_layout: &DepthTextureBindGroupLayout,
        surface_format: wgpu::TextureFormat,
        lighting: &LightingModel,
    ) -> Self {
        // Shader
        let shader = wgpu_renderer.device().create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader_heightmap.wgsl").into()),
        });

        // Pipeline
        let render_pipeline_layout =
            wgpu_renderer.device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Lod Heigtmap Pipeline Layout"),
                bind_group_layouts: &[
                    Some(camera_bind_group_layout.get()),
                    Some(texture_bind_group_layout.get()),
                    Some(heightmap_bind_group_layout.get()),
                    Some(shadow_map_bind_group_layout.get()),
                ],
                immediate_size: 0,
            });

        let render_pipeline = wgpu_renderer.device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Lod Heigtmap Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: match lighting {
                    LightingModel::None => Some("vs_main"),
                    LightingModel::Gouraud => Some("vs_main_gouraud"),
                },
                buffers: &[Some(Vertex::desc()), Some(Instance::desc())],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
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
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
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
        shadow_map: &'a DepthTexture,
        mesh: &'a mut dyn LodHeightMapShaderDraw,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        camera.bind(render_pass);
        shadow_map.bind(render_pass);
        mesh.draw(render_pass);
    }
}
