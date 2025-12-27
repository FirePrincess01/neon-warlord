//! Info for terrain data generation

#[derive(Clone, Debug)]
pub struct TerrainTextureDetails {
    pub pos_0: cgmath::Vector2<isize>, // texture world position at index (0/0)
    pub pos_1: cgmath::Vector2<isize>, // texture position at index (1/1)
    pub point_distance: usize,         // distance between pos_1.x - pos_0.x

    pub size_0: usize, // nr points between (0/0) and (N/N)
    pub size_1: usize, // nr points between (1/1) and ((N-1)/(N-1)), (size_0 - 2)

    pub nr_tiles: usize, // size_0 - 3

    pub depth: usize,      // Depth of the Node in the quad tree
    pub node_index: usize, // Index of the Node in the quad tree
}

pub fn depth_to_distance(depth: usize, max_depth: usize) -> usize {
    let exponent = max_depth - 1 - depth;

    2usize.pow(exponent as u32)
}

#[test]
fn test_depth_to_distance() {
    let max_depth = 5;
    assert_eq!(depth_to_distance(0, max_depth), 16);
    assert_eq!(depth_to_distance(1, max_depth), 8);
    assert_eq!(depth_to_distance(2, max_depth), 4);
    assert_eq!(depth_to_distance(3, max_depth), 2);
    assert_eq!(depth_to_distance(4, max_depth), 1);
}
