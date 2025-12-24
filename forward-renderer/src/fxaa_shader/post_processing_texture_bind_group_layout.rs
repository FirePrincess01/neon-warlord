pub struct PostProcessingTextureBindGroupLayout {
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl PostProcessingTextureBindGroupLayout {
    pub fn new(device: &wgpu::Device) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                // positions
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                },
                count: None,
            }],
            label: Some("Post Processing Bind Group Layout"),
        });

        Self { bind_group_layout }
    }
}
