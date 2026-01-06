//! Manages all instances of one single animated object
//!

use std::thread::LocalKey;

use cgmath::{EuclideanSpace, InnerSpace, Matrix4};
use wgpu_renderer::wgpu_renderer::WgpuRendererInterface;

use crate::animated_object::animated_model::animation::Animation;
use crate::animated_object::animated_model::skeleton::Skeleton;
use crate::animated_object::animated_object_data::AnimationData;
use crate::animated_object::gltf_importer::GltfImporter;
use crate::animation_shader::{self, AnimationShaderDraw};

struct AnimationObjectInstance {
    current_animation_index: usize,
    current_animation: Animation,

    instance: animation_shader::Instance,
    transformations: animation_shader::AnimationUniform,

    requires_update: bool,

    is_active: bool,
}

struct AnimationObjectInstanceDevice {
    instance_buffer: animation_shader::InstanceBuffer<animation_shader::Instance>,
    transformations_buffer: animation_shader::AnimationUniformBuffer,
}

pub struct AnimatedObjectStorage {
    // host data
    skeleton: Skeleton,
    animations: Vec<AnimationData>,

    // host isntance data
    instance_data: Vec<AnimationObjectInstance>,

    // device data
    mesh: animation_shader::Mesh,

    // device instance data
    instance_data_device: Vec<AnimationObjectInstanceDevice>,

    max_instances: usize,
}

impl AnimatedObjectStorage {
    pub fn create_from_glb(
        wgpu_renderer: &mut dyn wgpu_renderer::wgpu_renderer::WgpuRendererInterface,
        animation_bind_group_layout: &animation_shader::AnimationBindGroupLayout,
        glb_bin: &[u8],
        max_instances: usize,
    ) -> Self {
        // imported data
        let animation_object_data = GltfImporter::create(glb_bin);
        let mesh_data = animation_object_data.mesh;
        let skeleton_data = animation_object_data.skeleton;
        let animations_data = animation_object_data.animations;

        // host data
        let skeleton = Skeleton::new(&skeleton_data);
        let animations = animations_data;
        println!("skeleton {:?}", skeleton);
        println!("animations {:?}", animations);

        // host instace data
        let mut instance_data: Vec<AnimationObjectInstance> = Vec::new();
        for _i in 0..max_instances {
            let current_animation_index = 1;
            let current_animation = Animation::new(&animations[current_animation_index]);

            let position = cgmath::Vector3::new(0.0, 20.0, 5.0);
            let model = cgmath::Matrix4::from_translation(position);
            let instance = animation_shader::Instance {
                model: model.into(),
                color: [0.5, 0.5, 0.8, 1.0],
            };
            let transformations = animation_shader::AnimationUniform::zero();
            let requires_update = false;
            let is_active = false;

            instance_data.push(AnimationObjectInstance {
                current_animation_index,
                current_animation,
                instance,
                transformations,
                requires_update,
                is_active,
            });
        }

        // device data
        let mesh = animation_shader::Mesh::from_animation_data(wgpu_renderer, &mesh_data);

        // device instance data
        let mut instance_data_device: Vec<AnimationObjectInstanceDevice> = Vec::new();
        for _i in 0..max_instances {
            let instance_buffer = animation_shader::InstanceBuffer::new(
                wgpu_renderer.device(),
                &[animation_shader::Instance::new()],
            );
            let transformations_buffer = animation_shader::AnimationUniformBuffer::new(
                wgpu_renderer.device(),
                animation_bind_group_layout,
            );

            instance_data_device.push(AnimationObjectInstanceDevice {
                instance_buffer,
                transformations_buffer,
            });
        }

        Self {
            skeleton,
            animations,
            instance_data,
            mesh,
            instance_data_device,
            max_instances,
        }
    }

    /// Updates the animations
    pub fn update_animations(&mut self, dt: &instant::Duration) {
        for elem in &mut self.instance_data {
            if elem.is_active {
                // update time
                elem.current_animation.increment_time(dt);

                // calculate transfomations
                elem.current_animation.update_animation_uniform(
                    &self.skeleton,
                    &self.animations[elem.current_animation_index],
                    &mut elem.transformations,
                );
            }
        }
    }

    /// Copies the data from the host to the device
    pub fn update_device_data(&mut self, renderer: &mut dyn WgpuRendererInterface) {
        for i in 0..self.max_instances {
            // host data
            let instance_data_host = &self.instance_data[i];
            let instance_host = instance_data_host.instance;
            let transformations_host = &instance_data_host.transformations;

            // device data
            let instance_data_device = &mut self.instance_data_device[i];
            let instance_device = &mut instance_data_device.instance_buffer;
            let transformations_device = &mut instance_data_device.transformations_buffer;

            // copy
            instance_device.update(renderer.queue(), &[instance_host]);
            transformations_device.update(renderer.queue(), &transformations_host);
        }
    }

    pub fn max_instances(&self) -> usize {
        self.max_instances
    }

    // The model is assumed to look in x-direction
    // Creates 3 orthogonal vectors to assemble a rotation matrix
    pub fn look_to_rh(pos: cgmath::Point3<f32>, dir: cgmath::Vector3<f32>, up: cgmath::Vector3<f32>) -> Matrix4<f32> {
        let f = dir.normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);
        let s = -s;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            f.x.clone(), f.y.clone(), f.z.clone(), 0.0,
            s.x.clone(), s.y.clone(), s.z.clone(), 0.0,
            u.x.clone(), u.y.clone(), u.z.clone(), 0.0,
            pos.x, pos.y, pos.z, 1.0,
        )
    }

    pub fn set_pos(&mut self, id: usize, pos: &cgmath::Vector3<f32>, look_at: &cgmath::Vector3<f32>) {
        // let look_at = pos + look_at;

        // let model = Matrix4::look_at_rh(
        //     cgmath::Point3 {
        //         x: pos.x,
        //         y: pos.y,
        //         z: pos.z,
        //     },
        //     cgmath::Point3 {
        //         x: look_at.x,
        //         y: look_at.y,
        //         z: look_at.z,
        //     },
        //     cgmath::Vector3::unit_z(),
        // );
        let model = Self::look_to_rh(
                        cgmath::Point3 {
                x: pos.x,
                y: pos.y,
                z: pos.z,
            },
        cgmath::Vector3 {
                x: look_at.x,
                y: look_at.y,
                z: look_at.z,
            },
            cgmath::Vector3::unit_z(),
        );

        // inverts the x axis
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let invert_x = Matrix4::new(
            -1.0, 0.0, 0.0, 0.0,
            0.0, -1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        // inverts the y axis
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let _invert_y = Matrix4::new(
            0.0, -1.0, 0.0, 0.0,
            1.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let model = model * invert_x;

        // let angle: cgmath::Rad<f32> = cgmath::Deg(-90.0).into();
        // let model = cgmath::Matrix4::from_translation(*pos) * cgmath::Matrix4::from_angle_z(angle);

        self.instance_data[id].instance.model = model.into();
    }

    pub fn set_active(&mut self, id: usize) {
        self.instance_data[id].is_active = true;
    }
}

impl std::fmt::Debug for AnimatedObjectStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Mesh:")?;
        writeln!(f, "{:?}", self.mesh)?;

        writeln!(f, "Skeleton:")?;
        writeln!(f, "{:?}", self.skeleton)?;

        writeln!(f, "Animation:")?;
        writeln!(f, "{:?}", self.animations)?;

        Ok(())
    }
}

impl AnimationShaderDraw for AnimatedObjectStorage {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        for i in 0..self.max_instances {
            let mesh = &self.mesh;
            let instance_data_device = &self.instance_data_device[i];

            mesh.draw(
                render_pass,
                &instance_data_device.transformations_buffer,
                &instance_data_device.instance_buffer,
            );
        }
    }
}
