//! Creates vertex data to draw a quad
//!

use crate::geometry::rectangle::Rectangle;

type Vec3 = cgmath::Vector3<f32>;

#[allow(unused)]
pub struct Quad {
    rectangle: Rectangle,
}

impl Quad {
    #[allow(unused)]
    pub fn new(size: f32) -> Self {
        let a = size / 2.0;

        let rectangle = Rectangle::new(
            Vec3::new(-a, -a, 0.0),
            Vec3::new(a, -a, 0.0),
            Vec3::new(a, a, 0.0),
            Vec3::new(-a, a, 0.0),
        );

        Self {
            rectangle,
        }
    }
}

impl super::mesh::MeshInterface for Quad {
    fn vertices(&self) -> &[Vec3] {
        self.rectangle.vertices()
    }

    fn normals(&self) -> &[Vec3] {
        self.rectangle.normals()
    }

    fn indices(&self) -> &[u32] {
        self.rectangle.indices()
    }
}
