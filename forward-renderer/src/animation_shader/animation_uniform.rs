//! The AnimationUniform struct used in the shader
//!

use cgmath::prelude::*;

const MAX_JOINTS: usize = 16;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AnimationUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    pub joint_transform: [[[f32; 4]; 4]; MAX_JOINTS],
}

impl AnimationUniform {
    pub fn zero() -> Self {
        let uniform_mat: [[f32; 4]; 4] = cgmath::Matrix4::identity().into();

        let joint_transform: [[[f32; 4]; 4]; MAX_JOINTS] = [uniform_mat; MAX_JOINTS];

        Self { joint_transform }
    }
}
