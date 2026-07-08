//! A tree with nodes and leaves

use crate::procedural_tree::node::Node;
use cgmath::{InnerSpace, Vector3};
use rand::{RngExt, SeedableRng, rngs::StdRng};

type Vec3 = cgmath::Vector3<f32>;

pub struct Tree {
    nodes: Vec<Node>,

    root: usize,
    depth: usize,
}

impl Tree {
    pub fn new(depth: usize, seed: u64) -> Self {
        let mut nodes = Vec::new();
        let root = 0;

        let mut rng = StdRng::seed_from_u64(seed);
        

        let root_node = Node::new(Vec3::new(-2.0, 0.0, 3.0));
        nodes.push(root_node);

        Self::add_children(root, &mut nodes, depth-1, depth, &mut rng);
    
        Self {
            nodes,
            root,
            depth,
        }
    }

    fn add_children(node: usize, nodes: &mut Vec<Node>, depth: usize, max_depth: usize, rng: &mut StdRng) {

        let nr_children: usize = if depth == 1 {
            rng.random_range(3..=5)
        }
        else {
            rng.random_range(1..=(max_depth - depth).clamp(1, 3))
        };
        
        let index = nodes.len();
        let node = &mut nodes[node];
        node.children_base_index = index;
        node.nr_children = nr_children;

        for i in 0..nr_children {
            let x = rng.random_range(-1.0..=1.0);
            let y = rng.random_range(-1.0..=1.0);
            let z = rng.random_range(0.5..=1.0);

            let pos = Vec3::new(x, y, z).normalize();

            nodes.push(Node::new(Vec3::new(pos.x, pos.y, pos.z)));
        }

        for i in index..index+nr_children {
            if depth > 0 {
                Self::add_children(i, nodes, depth-1, max_depth, rng);
            }
        }
    }

    pub fn traverse_tree(&self, tree_interface: &mut dyn TreeInterface) {
        let node = &self.nodes[self.root];
        tree_interface.create_node(self.root, None, &node.position, node.nr_children == 0);
        Self::traverse(&self, self.root, tree_interface);
    }

    fn traverse(&self, node_index: usize, tree_interface: &mut dyn TreeInterface) {
        let node = &self.nodes[node_index];
        let pos_0 = &node.position;
        let index = node.children_base_index;
        let nr_children = node.nr_children;

        for i in index..index+nr_children {
            let node = &self.nodes[i];
            let pos_1 = node.position;
            tree_interface.create_node(i, Some(node_index), &pos_1, node.nr_children == 0);
        }

        for i in index..index+nr_children {
            Self::traverse(&self, i, tree_interface);
        }
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

}


pub trait TreeInterface {
    fn create_node(&mut self, node_index: usize, parent_index: Option<usize>,  pos: &Vec3, is_leave: bool);
}