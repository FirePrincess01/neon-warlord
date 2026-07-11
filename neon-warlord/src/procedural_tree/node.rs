//! A node from a tree

use crate::procedural_tree::Vec3;

pub struct Node {
    
    pub children_base_index: usize,
    pub nr_children: usize,
    pub position: Vec3,
}

impl Node {
    pub fn new(position: Vec3) -> Self {
        let children_base_index = 0;
        let nr_children = 0;
    
        Self{
            children_base_index,
            nr_children,
            position,
        }
    }
}