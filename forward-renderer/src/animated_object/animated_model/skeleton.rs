//! Creates a bone hiarchy

use cgmath::SquareMatrix;

use super::joint::Joint;

/// Creates a bone hiarchy
pub struct Skeleton {
    joints: Vec<Joint>,
}

impl Skeleton {
    pub(crate) fn new(
        skeleton_data: &crate::animated_object::animated_object_data::SkeletonData,
    ) -> Self {
        let joint_names = &skeleton_data.joint_names;
        let joint_children = &skeleton_data.joint_children;
        let joint_translations = &skeleton_data.joint_translations;
        let joint_rotations = &skeleton_data.joint_rotations;
        let inverse_bind_transforms = &skeleton_data.inverse_bind_transforms;

        let nr_joints = joint_names.len();
        assert_eq!(joint_children.len(), nr_joints);
        assert_eq!(joint_translations.len(), nr_joints);
        assert_eq!(joint_rotations.len(), nr_joints);
        assert_eq!(inverse_bind_transforms.len(), nr_joints);

        let mut joints = Vec::new();

        for i in 0..nr_joints {
            let name = joint_names[i].clone();
            let child_names = &joint_children[i];
            let child_indices = skeleton_data.joint_children_indices(i);
            let translation = joint_translations[i];
            let rotation = joint_rotations[i];
            let inverse_bind_transform = &inverse_bind_transforms[i];

            let joint = Joint::new(
                name,
                child_names.clone(),
                child_indices,
                translation,
                rotation,
                *inverse_bind_transform,
            );
            joints.push(joint);
        }

        Self { joints }
    }

    fn calculate_joint_transforms(
        &self,
        local_transforms: &[cgmath::Matrix4<f32>],
        joint_transforms: &mut [cgmath::Matrix4<f32>],
        parent_transform: &cgmath::Matrix4<f32>,
        joint_index: usize,
    ) {
        let joint = &self.joints[joint_index];

        // calculate current transformation
        let current_transform = parent_transform * local_transforms[joint_index];

        // calculate current transformation applicable to a vertex
        let current_joint_transform = current_transform * joint.get_inverse_bind_transform();
        // let current_joint_transform = cgmath::Matrix4::identity();
        joint_transforms[joint_index] = current_joint_transform;

        let children = joint.get_children_indices();
        for child in children {
            self.calculate_joint_transforms(
                local_transforms,
                joint_transforms,
                &current_transform,
                *child,
            )
        }
    }

    pub fn create_key_frame(
        &self,
        sample_poses: &[cgmath::Decomposed<cgmath::Vector3<f32>, cgmath::Quaternion<f32>>],
    ) -> Vec<cgmath::Matrix4<f32>> {
        let size = self.joints.len();
        let mut local_transforms: Vec<cgmath::Matrix4<f32>> =
            vec![cgmath::Matrix4::identity(); size];
        let mut joint_transforms: Vec<cgmath::Matrix4<f32>> =
            vec![cgmath::Matrix4::identity(); size];

        // set local transforms
        #[allow(clippy::needless_range_loop)]
        for i in 0..size {
            local_transforms[i] = self.joints[i].get_transform();
        }

        // apply sample poses
        #[allow(clippy::needless_range_loop)]
        for i in 0..sample_poses.len() {
            local_transforms[i] = cgmath::Matrix4::from(sample_poses[i]);
        }

        // calculate joint transforms
        let root_joint_index = 0;
        let parent_transform = cgmath::Matrix4::identity();
        self.calculate_joint_transforms(
            &local_transforms,
            &mut joint_transforms,
            &parent_transform,
            root_joint_index,
        );

        joint_transforms
    }

    fn print_children(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        index: usize,
        depth: usize,
    ) -> std::fmt::Result {
        let joint = &self.joints[index];
        for _i in 0..depth {
            write!(f, " |")?;
        }

        writeln!(f, "{} ", joint._get_name())?;

        for child_index in joint.get_children_indices() {
            self.print_children(f, *child_index, depth + 1)?;
        }

        Ok(())
    }
}

impl std::fmt::Debug for Skeleton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Nr Joints: {} ", self.joints.len())?;
        self.print_children(f, 0, 0)?;
        Ok(())
    }
}
