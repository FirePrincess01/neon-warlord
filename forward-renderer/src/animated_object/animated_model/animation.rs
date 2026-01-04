use crate::{animated_object::animated_object_data::AnimationData, animation_shader};

use super::skeleton::Skeleton;

type Decomposed = cgmath::Decomposed<cgmath::Vector3<f32>, cgmath::Quaternion<f32>>;

pub struct Animation {
    // animation_data: AnimationData,
    max_key_frame_time: f32,

    current_key_frame_time: instant::Duration,
}

impl Animation {
    pub fn zero() -> Self {
        Self {
            max_key_frame_time: 0.0,
            current_key_frame_time: instant::Duration::default(),
        }
    }

    pub fn new(animation_data: &AnimationData) -> Self {
        let mut max_key_frame_time: f32 = 0.0;
        for elem in &animation_data.joint_rotations {
            let max_time = *elem.key_times.last().unwrap();
            max_key_frame_time = max_key_frame_time.max(max_time);
        }

        for elem in &animation_data.joint_translations {
            let max_time = *elem.key_times.last().unwrap();
            max_key_frame_time = max_key_frame_time.max(max_time);
        }

        Self {
            // animation_data,
            max_key_frame_time,

            current_key_frame_time: instant::Duration::ZERO,
        }
    }

    fn get_current_key_frame_time(&self) -> f32 {
        let animation_speed = 1.0;
        self.current_key_frame_time.as_secs_f32() * animation_speed
    }

    pub fn increment_time(&mut self, dt: &instant::Duration) {
        self.current_key_frame_time += *dt;

        if self.get_current_key_frame_time() > self.max_key_frame_time {
            self.current_key_frame_time = instant::Duration::ZERO;
        }
    }

    fn get_sample_poses(&self, animation_data: &AnimationData) -> Vec<Decomposed> {
        let current_time = self.get_current_key_frame_time();
        let joint_translations = &animation_data.joint_translations;
        let joint_rotations = &animation_data.joint_rotations;
        let len = joint_translations.len();
        assert_eq!(joint_rotations.len(), len);

        let mut res: Vec<Decomposed> = Vec::new();

        for i in 0..len {
            let translation = joint_translations[i].get_translation(current_time);
            let rotation = joint_rotations[i].get_rotation(current_time);

            let decomposed = cgmath::Decomposed {
                scale: 1.0,
                rot: *rotation,
                disp: *translation,
            };

            res.push(decomposed);
        }

        res
    }

    pub fn update_animation_uniform(
        &self,
        skeleton: &Skeleton,
        animation_data: &AnimationData,
        animation_uniform: &mut animation_shader::AnimationUniform,
    ) {
        let sample_poses: Vec<Decomposed> = self.get_sample_poses(animation_data);
        let joint_transforms = skeleton.create_key_frame(&sample_poses);

        // println!("animation_uniform.joint_transform.len() {}", animation_uniform.joint_transform.len());
        // println!("joint_transforms.len() {}", joint_transforms.len());
        // println!("joint_transforms {:?}", joint_transforms);
        assert!(animation_uniform.joint_transform.len() >= joint_transforms.len());

        let len = joint_transforms.len();
        #[allow(clippy::needless_range_loop)]
        for i in 0..len {
            animation_uniform.joint_transform[i] = joint_transforms[i].into();
        }
    }
}
