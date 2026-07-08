//! A node from a tree

type Vec3 = cgmath::Vector3<f32>;

struct Node {
    
    children: [Vec3; 8],
    nr_children: usize
}