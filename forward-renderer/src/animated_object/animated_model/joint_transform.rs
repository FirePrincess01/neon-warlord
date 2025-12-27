use cgmath::{Matrix, SquareMatrix, Zero};

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct JointTransform {
    translation: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,

    joint_transform: cgmath::Matrix4<f32>,
}

#[allow(unused)]
impl JointTransform {
    // pub fn new(translation: cgmath::Vector3<f32>, rotation: cgmath::Quaternion<f32>) -> Self {
    //     Self {
    //         translation,
    //         rotation,
    //         joint_transform: cgmath::Matrix4::identity(),
    //     }
    // }

    pub fn zero() -> Self {
        let translation: cgmath::Vector3<f32> = cgmath::Vector3::zero();
        let rotation: cgmath::Quaternion<f32> = cgmath::Quaternion::zero();

        Self {
            translation,
            rotation,
            joint_transform: cgmath::Matrix4::identity(),
        }
    }

    pub fn from_mat4(mat: cgmath::Matrix4<f32>) -> Self {
        let c0r0 = mat.row(0)[0];
        let c0r1 = mat.row(1)[0];
        let c0r2 = mat.row(2)[0];
        let _c0r3 = mat.row(3)[0];

        let c1r0 = mat.row(0)[1];
        let c1r1 = mat.row(1)[1];
        let c1r2 = mat.row(2)[1];
        let _c1r3 = mat.row(3)[1];

        let c2r0 = mat.row(0)[2];
        let c2r1 = mat.row(1)[2];
        let c2r2 = mat.row(2)[2];
        let _c2r3 = mat.row(3)[2];

        let c3r0 = mat.row(0)[3];
        let c3r1 = mat.row(1)[3];
        let c3r2 = mat.row(2)[3];
        let _c3r3 = mat.row(3)[3];

        let c0 = cgmath::Vector3::new(c0r0, c0r1, c0r2);
        let c1 = cgmath::Vector3::new(c1r0, c1r1, c1r2);
        let c2 = cgmath::Vector3::new(c2r0, c2r1, c2r2);
        let c3 = cgmath::Vector3::new(c3r0, c3r1, c3r2);

        let mat3 = cgmath::Matrix3::from_cols(c0, c1, c2);

        let translation = c3;
        let rotation: cgmath::Quaternion<f32> = mat3.into();

        Self {
            translation,
            rotation,
            joint_transform: mat,
        }
    }

    pub fn to_mat4(&self) -> cgmath::Matrix4<f32> {
        // let mut res = cgmath::Matrix4::from(self.rotation);
        // let c3: cgmath::Vector4<f32> = cgmath::Vector4::new(
        //     self.translation.x,
        //     self.translation.y,
        //     self.translation.z,
        //     1.0,
        // );
        // res.replace_col(3, c3);

        // res

        self.joint_transform
    }

    // pub fn interpolate(&self, other: &Self, amount: f32) -> JointTransform {
    //     let amount = amount.clamp(0.0, 1.0);

    //     let translation = self.translation.lerp(other.translation, amount);
    //     let rotation = self.rotation.nlerp(other.rotation, amount);

    //     let res = Self::new(translation, rotation);

    //     res
    // }
}
