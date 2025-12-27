//! Contains a buffer for the AnimationUniform struct
//!

use super::animation_bind_group_layout;
use super::animation_uniform;
use wgpu::util::DeviceExt;

pub struct AnimationUniformBuffer {
    animation_buffer: wgpu::Buffer,
    animation_bind_group: wgpu::BindGroup,
}

impl AnimationUniformBuffer {
    pub fn new(
        device: &wgpu::Device,
        animation_bind_group_layout: &animation_bind_group_layout::AnimationBindGroupLayout,
    ) -> Self {
        let animation_uniform = animation_uniform::AnimationUniform::zero();

        let animation_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Animation Buffer"),
            contents: bytemuck::cast_slice(&[animation_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let animation_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: animation_bind_group_layout.get(),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: animation_buffer.as_entire_binding(),
            }],
            label: Some("animation_bind_group"),
        });

        Self {
            animation_buffer,
            animation_bind_group,
        }
    }

    pub fn update(
        &mut self,
        queue: &wgpu::Queue,
        animation_uniform: &animation_uniform::AnimationUniform,
    ) {
        queue.write_buffer(
            &self.animation_buffer,
            0,
            bytemuck::cast_slice(&[*animation_uniform]),
        );
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(1, &self.animation_bind_group, &[]);
    }
}
