//! Creates vertex data to draw a Frame
//!

type Vec3 = cgmath::Vector3<f32>;

use super::mesh::Mesh;
use super::rectangle::Rectangle;

#[allow(unused)]
pub struct Frame {
    mesh: Mesh,
    // pub deferred_vertices: Vec<deferred_color_shader::Vertex>,
    // pub indices: Vec<u16>,
}

#[allow(unused)]
impl Frame {
    pub fn new(scale: f32) -> Self {
        let l: f32 = 1.0 * scale;
        let h: f32 = 0.1 * scale;
        let w: f32 = 0.05 * scale;

        let l2: f32 = l - 2.0 * w;

        // outside
        let r1 = Rectangle::new(
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 0., h),
            Vec3::new(0., l, h),
            Vec3::new(0., l, 0.),
        );

        let r2 = Rectangle::new(
            Vec3::new(0., 0., 0.),
            Vec3::new(l, 0., 0.),
            Vec3::new(l, 0., h),
            Vec3::new(0., 0., h),
        );

        let r3 = Rectangle::new(
            Vec3::new(l, 0., 0.),
            Vec3::new(l, 0., h),
            Vec3::new(l, l, h),
            Vec3::new(l, l, 0.),
        );

        let r4 = Rectangle::new(
            Vec3::new(0., l, h),
            Vec3::new(l, l, h),
            Vec3::new(l, l, 0.),
            Vec3::new(0., l, 0.),
        );

        // inside
        let r5 = Rectangle::new(
            Vec3::new(w, w, h),
            Vec3::new(w, w, 0.),
            Vec3::new(w, w + l2, 0.),
            Vec3::new(w, w + l2, h),
        );

        let r6 = Rectangle::new(
            Vec3::new(w, w, h),
            Vec3::new(w + l2, w, h),
            Vec3::new(w + l2, w, 0.),
            Vec3::new(w, w, 0.),
        );

        let r7 = Rectangle::new(
            Vec3::new(w + l2, w, 0.),
            Vec3::new(w + l2, w, h),
            Vec3::new(w + l2, w + l2, h),
            Vec3::new(w + l2, w + l2, 0.),
        );

        let r8 = Rectangle::new(
            Vec3::new(w, w + l2, 0.),
            Vec3::new(w + l2, w + l2, 0.),
            Vec3::new(w + l2, w + l2, h),
            Vec3::new(w, w + l2, h),
        );

        // top
        let r9 = Rectangle::new(
            Vec3::new(0., w, h),
            Vec3::new(w, w, h),
            Vec3::new(w, w + l2, h),
            Vec3::new(0., w + l2, h),
        );

        let r10 = Rectangle::new(
            Vec3::new(0., 0., h),
            Vec3::new(l, 0., h),
            Vec3::new(l, w, h),
            Vec3::new(0., w, h),
        );

        let r11 = Rectangle::new(
            Vec3::new(w + l2, w, h),
            Vec3::new(l, w, h),
            Vec3::new(l, w + l2, h),
            Vec3::new(w + l2, w + l2, h),
        );

        let r12 = Rectangle::new(
            Vec3::new(0., w + l2, h),
            Vec3::new(l, w + l2, h),
            Vec3::new(l, l, h),
            Vec3::new(0., l, h),
        );

        let mut mesh = Mesh::new();

        mesh.add(&r1);
        mesh.add(&r2);
        mesh.add(&r3);
        mesh.add(&r4);

        mesh.add(&r5);
        mesh.add(&r6);
        mesh.add(&r7);
        mesh.add(&r8);

        mesh.add(&r9);
        mesh.add(&r10);
        mesh.add(&r11);
        mesh.add(&r12);

        // let mut deferred_vertices: Vec<deferred_color_shader::Vertex> = Vec::new();

        // for (vertex, normal) in zip(mesh.vertices, mesh.normals) {
        //     deferred_vertices.push(deferred_color_shader::Vertex {
        //         position: [vertex.x, vertex.y, vertex.z],
        //         normal: [normal.x, normal.y, normal.z],
        //     });
        // }

        // let indices = mesh.indices;

        Self { mesh }
    }
}
