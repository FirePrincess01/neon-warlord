//! Object to traverse the lod quad tree and draw every leaf
//!

use super::{
    lod_quad_tree::QuadTreeInterface,
    terrain_texture_details::{TerrainTextureDetails, depth_to_distance},
};

pub struct QuadTreeDraw<T>
where
    T: FnMut(usize),
{
    max_depth: usize,
    nr_tiles: usize,

    draw_function: T,

    pub requests: Vec<TerrainTextureDetails>,
}

impl<T> QuadTreeDraw<T>
where
    T: FnMut(usize),
{
    pub fn new(max_depth: usize, nr_tiles: usize, draw_function: T) -> Self {
        Self {
            max_depth,
            nr_tiles,
            draw_function,
            requests: Vec::new(),
        }
    }
}

impl<T> QuadTreeInterface for QuadTreeDraw<T>
where
    T: FnMut(usize),
{
    fn request_data(&mut self, node: usize, square: super::lod_quad_tree::Square, depth: usize) {
        let point_distance = depth_to_distance(depth, self.max_depth);
        let pos_1 = square.pos_0;
        let pos_0 = pos_1 - cgmath::vec2(point_distance as isize, point_distance as isize);

        let nr_tiles = self.nr_tiles;
        let size_0 = nr_tiles + 3;
        let size_1 = nr_tiles + 1;

        let details = TerrainTextureDetails {
            pos_0,
            pos_1,
            point_distance,
            size_0,
            size_1,
            nr_tiles,
            depth,
            node_index: node,
        };

        self.requests.push(details);
    }

    fn do_work(&mut self, index: usize, _square: super::lod_quad_tree::Square, _depth: usize) {
        (self.draw_function)(index)
    }
}
