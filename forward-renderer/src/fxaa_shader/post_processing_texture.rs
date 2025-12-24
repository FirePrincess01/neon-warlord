// A texture which can be used as output render target

use wgpu_renderer::wgpu_renderer::WgpuRendererInterface;

use super::PostProcessingTextureBindGroupLayout;

pub struct PostProcessingTexture {
    pub _texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub bind_group: wgpu::BindGroup,
}

impl PostProcessingTexture {
    pub fn new(
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        bind_group_layout: &PostProcessingTextureBindGroupLayout,
        surface_width: u32,
        surface_height: u32,
        surface_format: wgpu::TextureFormat,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: surface_width,
            height: surface_height,
            depth_or_array_layers: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some("Post Processing Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: surface_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: Default::default(),
        };
        let texture = wgpu_renderer.device().create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group: wgpu::BindGroup =
            wgpu_renderer
                .device()
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &bind_group_layout.bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view),
                    }],
                    label: Some("g_buffer_bind_group"),
                });

        Self {
            _texture: texture,
            view,
            bind_group,
        }
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(1, &self.bind_group, &[]);
    }
}
