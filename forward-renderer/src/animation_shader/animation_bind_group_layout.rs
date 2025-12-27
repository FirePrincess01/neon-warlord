//! A bind group to create a animation uniform buffer for this shader
//!

pub struct AnimationBindGroupLayout {
    animation_bind_group_layout: wgpu::BindGroupLayout,
}

impl AnimationBindGroupLayout {
    pub fn new(device: &wgpu::Device) -> Self {
        // Camera
        let animation_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("animation_bind_group_layout"),
            });

        Self {
            animation_bind_group_layout,
        }
    }

    pub fn get(&self) -> &wgpu::BindGroupLayout {
        &self.animation_bind_group_layout
    }
}
