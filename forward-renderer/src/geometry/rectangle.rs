//! Creates vertex data to draw a Rectangle
//!

type Vec3 = cgmath::Vector3<f32>;

use cgmath::InnerSpace;

use super::mesh::MeshInterface;

#[allow(unused)]
pub struct Rectangle {
    vertices: [Vec3; 4],
    normals: [Vec3; 4],
    indices: [u16; 6],
}

#[allow(unused)]
impl Rectangle {
    pub fn new(point_a: Vec3, point_b: Vec3, point_c: Vec3, point_d: Vec3) -> Self {
        let normal1 = (point_a - point_b).cross(point_c - point_b).normalize();
        let normal2 = (point_c - point_d).cross(point_a - point_d).normalize();
        assert_eq!(normal1, normal2);

        let vertices = [point_a, point_b, point_c, point_d];

        let indices = [
            0, 1, 2, // ABC
            2, 3, 0, // CDA
        ];

        let normal = normal1;
        let normals = [normal, normal, normal, normal];

        Self {
            vertices,
            normals,
            indices,
        }
    }
}

impl MeshInterface for Rectangle {
    fn vertices(&self) -> &[Vec3] {
        &self.vertices
    }

    fn normals(&self) -> &[Vec3] {
        &self.normals
    }

    fn indices(&self) -> &[u16] {
        &self.indices
    }
}
