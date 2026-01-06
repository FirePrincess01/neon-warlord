//! Contains the device buffers to render an object with this shader
//!

use super::Instance;
use super::Vertex;
use super::animation_uniform_buffer::AnimationUniformBuffer;

use super::IndexBuffer;

use super::InstanceBuffer;
use super::VertexBuffer;

/// A general purpose shader using vertices, colors and an instance matrix
pub struct Mesh {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
    _nr_vertices: u32,
}

#[allow(dead_code)]
impl Mesh {
    pub fn new(
        wgpu_renderer: &mut dyn wgpu_renderer::wgpu_renderer::WgpuRendererInterface,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> Self {
        let vertex_buffer = VertexBuffer::new(wgpu_renderer.device(), vertices);
        let index_buffer = IndexBuffer::new(wgpu_renderer.device(), indices);
        let nr_vertices = vertices.len() as u32;

        Self {
            vertex_buffer,
            index_buffer,
            _nr_vertices: nr_vertices,
        }
    }

    pub fn from_animation_data(
        wgpu_renderer: &mut dyn wgpu_renderer::wgpu_renderer::WgpuRendererInterface,
        mesh_data: &crate::animated_object::animated_object_data::MeshData,
    ) -> Self {
        let positions = &mesh_data.positions;
        let normals = &mesh_data.normals;
        let joints = &mesh_data.joints;
        let weights = &mesh_data.weights;
        let indices_u16 = &mesh_data.indices;

        let len = positions.len();
        assert_eq!(normals.len(), len);
        assert_eq!(joints.len(), len);
        assert_eq!(weights.len(), len);

        let mut vertices = Vec::new();
        for i in 0..len {
            let vertex = Vertex {
                position: [positions[i][0], positions[i][1], positions[i][2], 1.0],
                normal: [normals[i][0], normals[i][1], normals[i][2], 0.0],
                joint_indices: [
                    joints[i][0] as u32,
                    joints[i][1] as u32,
                    joints[i][2] as u32,
                    joints[i][3] as u32,
                ],
                joint_weights: weights[i],
            };

            vertices.push(vertex);
        }

        let mut indices = Vec::new();
        for elem in indices_u16 {
            indices.push(*elem as u32);
        }

        Self::new(
            wgpu_renderer,
            &vertices,
            &indices,
        )
    }

    pub fn update_vertex_buffer(&mut self, queue: &wgpu::Queue, vertices: &[Vertex]) {
        self.vertex_buffer.update(queue, vertices);
    }


    pub fn draw<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        animation_buffer: &'a AnimationUniformBuffer,
        instance_buffer: &'a InstanceBuffer<Instance>,
    ) {
        self.vertex_buffer.bind(render_pass);
        self.index_buffer.bind(render_pass);
        animation_buffer.bind(render_pass);
        instance_buffer.bind(render_pass);

        render_pass.draw_indexed(0..self.index_buffer.size(), 0, 0..instance_buffer.size());
    }
}

impl std::fmt::Debug for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Vertices: {}", self._nr_vertices)?;
        writeln!(f, "Indices: {}", self.index_buffer.size())
    }
}
