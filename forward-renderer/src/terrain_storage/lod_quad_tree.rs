//! A tree with 4 sub elements
//!

use cgmath::InnerSpace;

use super::terrain_texture_details::depth_to_distance;

pub trait QuadTreeInterface {
    fn request_data(&mut self, node: usize, square: Square, depth: usize);
    fn do_work(&mut self, index: usize, square: Square, depth: usize);
}

pub struct LodQuadTree {
    nodes: Vec<QuadNode>,
    max_depth: usize,

    root: usize,
    square: Square,
}

impl LodQuadTree {
    pub fn new(max_depth: usize, nr_tiles: usize) -> Self {
        let nodes = Vec::new();
        let root = 0;

        let distance = depth_to_distance(0, max_depth);
        let a = nr_tiles * distance;

        let square = Square {
            pos_0: cgmath::Vector2 {
                x: 0 - a as isize / 2,
                y: 0 - a as isize / 2,
            },
            a,
        };

        Self {
            nodes,
            max_depth,
            root,
            square,
        }
    }

    pub fn traverse_leaves(
        &mut self,
        view_position: &Vec3,
        data_interface: &mut impl QuadTreeInterface,
    ) {
        if self.nodes.is_empty() {
            self.nodes.push(QuadNode::new());
            data_interface.request_data(0, self.square.clone(), 0);
            return;
        }

        self.traverse(
            self.root,
            self.square.clone(),
            view_position,
            0,
            data_interface,
        );
    }

    pub fn set_data_index(&mut self, node_index: usize, data_index: usize) {
        self.nodes[node_index].data = Some(data_index);
    }

    fn traverse(
        &mut self,
        node: usize,
        square: Square,
        view_position: &Vec3,
        depth: usize,
        data_interface: &mut impl QuadTreeInterface,
    ) {
        let distance = square.middle_vec3() - view_position;
        let distance_squared = distance.dot(distance);
        let a_squared = square.a * square.a;

        let is_in_range = distance_squared < a_squared as isize * 8;

        if depth < self.max_depth - 1 && is_in_range {
            if self.nodes[node].first_child.is_none() {
                // create children
                let first_child = self.nodes.len();

                self.nodes.push(QuadNode::new());
                data_interface.request_data(first_child, square.quadrant_sw(), depth + 1);

                self.nodes.push(QuadNode::new());
                data_interface.request_data(first_child + 1, square.quadrant_se(), depth + 1);

                self.nodes.push(QuadNode::new());
                data_interface.request_data(first_child + 2, square.quadrant_ne(), depth + 1);

                self.nodes.push(QuadNode::new());
                data_interface.request_data(first_child + 3, square.quadrant_nw(), depth + 1);

                self.nodes[node].first_child = Some(first_child);
            }

            // check children
            let first_child_index = self.nodes[node].first_child.unwrap();

            let data_available = self.nodes[first_child_index].data.is_some()
                && self.nodes[first_child_index + 1].data.is_some()
                && self.nodes[first_child_index + 2].data.is_some()
                && self.nodes[first_child_index + 3].data.is_some();

            if data_available {
                // traverse children
                self.traverse(
                    first_child_index,
                    square.quadrant_sw(),
                    view_position,
                    depth + 1,
                    data_interface,
                );
                self.traverse(
                    first_child_index + 1,
                    square.quadrant_se(),
                    view_position,
                    depth + 1,
                    data_interface,
                );
                self.traverse(
                    first_child_index + 2,
                    square.quadrant_ne(),
                    view_position,
                    depth + 1,
                    data_interface,
                );
                self.traverse(
                    first_child_index + 3,
                    square.quadrant_nw(),
                    view_position,
                    depth + 1,
                    data_interface,
                );
                return;
            }
        }

        // work with data
        if let Some(data_index) = self.nodes[node].data {
            data_interface.do_work(data_index, square, depth)
        }
    }
}

struct QuadNode {
    /// Points to the index of the first child  
    /// first_child+0 = index to 1st child (TL)  
    /// first_child+1 = index to 2nd child (TR)  
    /// first_child+2 = index to 3nd child (BL)  
    /// first_child+3 = index to 4th child (BR)  
    first_child: Option<usize>,

    /// Points to the index of the data
    data: Option<usize>,
}

impl QuadNode {
    fn new() -> Self {
        Self {
            first_child: None,
            data: None,
        }
    }
}

type Vec3 = cgmath::Vector3<isize>;
type Vec2 = cgmath::Vector2<isize>;

#[derive(Clone)]
pub struct Square {
    pub pos_0: cgmath::Vector2<isize>,
    pub a: usize,
}

impl Square {
    pub fn middle_vec3(&self) -> Vec3 {
        let middle = self.middle();

        cgmath::Vector3::new(middle.x, middle.y, 0)
    }

    pub fn middle(&self) -> Vec2 {
        let pos_x = self.pos_0.x + self.a as isize / 2;
        let pos_y = self.pos_0.y + self.a as isize / 2;

        cgmath::Vector2 { x: pos_x, y: pos_y }
    }

    pub fn quadrant_sw(&self) -> Square {
        let pos_x = self.pos_0.x;
        let pos_y = self.pos_0.y;

        Square {
            pos_0: cgmath::Vector2 { x: pos_x, y: pos_y },
            a: self.a / 2,
        }
    }

    pub fn quadrant_se(&self) -> Square {
        let pos_x = self.pos_0.x + self.a as isize / 2;
        let pos_y = self.pos_0.y;

        Square {
            pos_0: cgmath::Vector2 { x: pos_x, y: pos_y },
            a: self.a / 2,
        }
    }

    pub fn quadrant_ne(&self) -> Square {
        let pos_x = self.pos_0.x + self.a as isize / 2;
        let pos_y = self.pos_0.y + self.a as isize / 2;

        Square {
            pos_0: cgmath::Vector2 { x: pos_x, y: pos_y },
            a: self.a / 2,
        }
    }

    pub fn quadrant_nw(&self) -> Square {
        let pos_x = self.pos_0.x;
        let pos_y = self.pos_0.y + self.a as isize / 2;

        Square {
            pos_0: cgmath::Vector2 { x: pos_x, y: pos_y },
            a: self.a / 2,
        }
    }
}
