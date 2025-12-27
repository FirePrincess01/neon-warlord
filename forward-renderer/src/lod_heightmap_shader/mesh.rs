//! Contains the device buffers to render an object with this shader
//!

use wgpu_renderer::shape;

use super::Vertex;

use super::IndexBuffer;
use super::VertexBuffer;

pub struct Mesh {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
}

#[allow(unused)]
impl Mesh {
    pub fn new(device: &wgpu::Device, vertices: &[Vertex], indices: &[u32]) -> Self {
        let vertex_buffer = VertexBuffer::new(device, vertices);
        let index_buffer = IndexBuffer::new(device, indices);
        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn from_shape(device: &wgpu::Device, shape: &shape::MeshDataTriangles) -> Self {
        let vertices = &shape.positions;
        let normals = &shape.normals;
        let indices = &shape.indices;

        assert_eq!(vertices.len(), normals.len());

        let len = vertices.iter().len();
        let mut mesh_vertices = Vec::with_capacity(len);

        for elem in vertices {
            mesh_vertices.push(Vertex {
                position: (*elem).into(),
            });
        }

        Self::new(device, &mesh_vertices, indices)
    }

    pub fn _update_vertex_buffer(&mut self, queue: &wgpu::Queue, vertices: &[Vertex]) {
        self.vertex_buffer.update(queue, vertices);
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.vertex_buffer.bind(render_pass);
        self.index_buffer.bind(render_pass);
    }

    pub fn draw_indexed<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.draw_indexed(0..self.index_buffer.size(), 0, 0..1);
    }
}
