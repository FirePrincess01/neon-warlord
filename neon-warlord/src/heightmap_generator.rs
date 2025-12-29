//! Provides a function to generate single tiles of the heightmap

use forward_renderer::{HeightMap, TerrainTextureDetails};
use noise::NoiseFn;

#[allow(unused)]
pub struct HeightMapGenerator {
    perlin: noise::Perlin,
}

impl Default for HeightMapGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unused)]
impl HeightMapGenerator {
    pub fn new() -> Self {
        let perlin: noise::Perlin = noise::Perlin::new(1);

        Self { perlin }
    }

    pub fn generate(&self, details: &TerrainTextureDetails) -> HeightMap {
        let distance = details.point_distance;
        let size_x = details.size_0;
        let size_y = details.size_0;
        let p_x = details.pos_0.x;
        let p_y = details.pos_0.y;

        let size = size_x * size_y;

        let mut heights = Vec::with_capacity(size);
        for y in 0..size_y {
            for x in 0..size_x {
                let mut height = (self.perlin.get([
                    (p_x + x as isize * distance as isize) as f64 / 128.0,
                    (p_y + y as isize * distance as isize) as f64 / 128.0,
                ]) * 20.0)
                    .max(0.0);

                height += (self.perlin.get([
                    (p_x + x as isize * distance as isize) as f64 / 64.0,
                    (p_y + y as isize * distance as isize) as f64 / 64.0,
                ]) * 20.0)
                    .max(0.0);

                height += (self.perlin.get([
                    (p_x + x as isize * distance as isize) as f64 / 32.0,
                    (p_y + y as isize * distance as isize) as f64 / 32.0,
                ]) * 20.0)
                    .max(0.0);

                height += (self.perlin.get([
                    (p_x + x as isize * distance as isize) as f64 / 16.0,
                    (p_y + y as isize * distance as isize) as f64 / 16.0,
                ]) * 8.0)
                    .max(0.0);

                height += (self.perlin.get([
                    (p_x + x as isize * distance as isize) as f64 / 8.0,
                    (p_y + y as isize * distance as isize) as f64 / 8.0,
                ]) * 2.0)
                    .max(0.0);

                // create canyon
                let a = Self::depth_to_distance(7, 8);
                height *= Self::canyon(
                    (p_y + y as isize * distance as isize - a as isize / 2) as f32 / 30.0,
                ) as f64;

                if p_y < -10 {
                    height = 0.0;
                }

                heights.push(height as f32);
                // heights.push(0.0);
            }
        }

        HeightMap {
            heights,
            details: details.clone(),
        }
    }

    fn canyon(x: f32) -> f32 {
        1.0 - 1.0 / (1.0 + x * x * x * x * x * x)
    }

    pub fn depth_to_distance(depth: usize, max_depth: usize) -> usize {
        let exponent = max_depth - 1 - depth;
        2usize.pow(exponent as u32)
    }
}
