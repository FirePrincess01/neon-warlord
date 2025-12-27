//! Manages all instances of one single animated object
//!

use wgpu_renderer::wgpu_renderer::WgpuRendererInterface;

use crate::animated_object::animated_model::animation::Animation;
use crate::animated_object::animated_model::skeleton::Skeleton;
use crate::animated_object::animated_object_data::AnimationData;
use crate::animated_object::gltf_importer::GltfImporter;
use crate::animation_shader::{self, AnimationShaderDraw};

pub struct AnimatedObjectStorage {
    // host data
    skeleton: Skeleton,
    animation_data: AnimationData,

    instance_host: Vec<AnimatedObjectInstanceHost>,

    // device data
    mesh: animation_shader::Mesh,
    instance_device: Vec<AnimatedObjectInstanceDevice>,

    update_done: usize,

    instances: Vec<animation_shader::Instance>,
    instance_buffer: animation_shader::InstanceBuffer<animation_shader::Instance>,
}

impl AnimatedObjectStorage {
    pub fn create_from_glb(
        wgpu_renderer: &mut dyn wgpu_renderer::wgpu_renderer::WgpuRendererInterface,
        animation_bind_group_layout: &animation_shader::AnimationBindGroupLayout,
        glb_bin: &[u8],
        max_instances: usize,
    ) -> Self {
        let animation_object_data = GltfImporter::create(glb_bin);

        let skeleton = Skeleton::new(&animation_object_data);
        let animation_data = animation_object_data.animations[0].clone();
        // let animation_0 = Animation::new(&animation_data);
        let animation_uniform = animation_shader::AnimationUniform::zero();

        // let instance = deferred_animation_shader::Instance {
        //     position: [0.0, 20.0, 5.0],
        //     color: [0.5, 0.5, 0.8],
        //     entity: [99, 0, 0],
        // };

        let mesh = animation_shader::Mesh::from_animation_data(
            wgpu_renderer,
            // animation_bind_group_layout,
            &animation_object_data,
            // &[instance],
        );

        let mut instance_host = Vec::with_capacity(max_instances);
        let mut instance_device = Vec::with_capacity(max_instances);

        for _i in 0..max_instances {
            instance_host.push(AnimatedObjectInstanceHost {
                animation: Animation::new(&animation_data),
                is_active: false,
                animation_uniform,
                _instance: animation_shader::Instance {
                    position: [0.0, 20.0, 5.0],
                    color: [0.5, 0.5, 0.8],
                    // entity: [i as u32 | ENTITY_ANT_BIT, 0, 0],
                },
            });
        }

        for _i in 0..max_instances {
            instance_device.push(AnimatedObjectInstanceDevice {
                animation_uniform_buffer: animation_shader::AnimationUniformBuffer::new(
                    wgpu_renderer.device(),
                    animation_bind_group_layout,
                ),
                _instance_buffer: animation_shader::InstanceBuffer::new(
                    wgpu_renderer.device(),
                    &[animation_shader::Instance::new()],
                ),
            });
        }

        let mut instances = Vec::with_capacity(max_instances);
        for _i in 0..max_instances {
            instances.push(animation_shader::Instance {
                position: [0.0, 20.0, 5.0],
                color: [0.5, 0.5, 0.8],
                // entity: [i as u32 | ENTITY_ANT_BIT, 0, 0],
            });
        }

        let instance_buffer =
            animation_shader::InstanceBuffer::new(wgpu_renderer.device(), &instances);

        Self {
            skeleton,
            animation_data,
            mesh,
            instance_host,
            instance_device,
            update_done: 0,
            instances,
            instance_buffer,
        }
    }

    /// Updates the animations
    pub fn update_animations(&mut self, dt: &instant::Duration) {
        for elem in &mut self.instance_host {
            if elem.is_active {
                elem.animation.increment_time(dt);
                elem.animation.update_animation_uniform(
                    &self.skeleton,
                    &self.animation_data,
                    &mut elem.animation_uniform,
                );
            }
        }
    }

    /// Copies the data from the host to the device
    pub fn update_device_data(&mut self, renderer: &mut dyn WgpuRendererInterface) {
        let size = self.instance_host.len();
        assert_eq!(size, self.instance_device.len());

        for i in 0..1 {
            // if self.update_done < 1000000 {
            self.update_done += 1;

            // if self.instance_host[i].is_active {
            self.instance_device[i]
                .animation_uniform_buffer
                .update(renderer.queue(), &self.instance_host[i].animation_uniform);

            //     self.instance_device[i]
            //         .instance_buffersa
            //         .update(renderer.queue(), &[self.instance_host[i].instance]);
            // }
            // }
            // }
        }

        self.instance_buffer
            .update(renderer.queue(), &self.instances);
    }

    pub fn max_instances(&self) -> usize {
        self.instance_host.len()
    }

    pub fn set_pos(&mut self, id: usize, pos: cgmath::Vector3<f32>) {
        // self.instance_host[id].instance.position = pos.into();
        self.instances[id].position = pos.into();
    }

    pub fn set_active(&mut self, id: usize) {
        self.instance_host[id].is_active = true;
    }
}

impl std::fmt::Debug for AnimatedObjectStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Mesh:")?;
        writeln!(f, "{:?}", self.mesh)?;

        writeln!(f, "Skeleton:")?;
        writeln!(f, "{:?}", self.skeleton)?;

        writeln!(f, "Animation:")?;
        writeln!(f, "{:?}", self.animation_data)?;

        Ok(())
    }
}

impl AnimationShaderDraw for AnimatedObjectStorage {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        let size = self.instance_host.len();
        assert_eq!(size, self.instance_device.len());

        self.mesh.draw(
            render_pass,
            &self.instance_device[0].animation_uniform_buffer,
            &self.instance_buffer,
        );
    }
}

struct AnimatedObjectInstanceHost {
    pub animation: Animation,
    pub animation_uniform: animation_shader::AnimationUniform,
    pub _instance: animation_shader::Instance,

    pub is_active: bool,
}

struct AnimatedObjectInstanceDevice {
    pub animation_uniform_buffer: animation_shader::AnimationUniformBuffer,
    pub _instance_buffer: animation_shader::InstanceBuffer<animation_shader::Instance>,
}
