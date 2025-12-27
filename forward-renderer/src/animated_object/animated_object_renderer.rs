// pub struct AnimatedObjectRendererResult {
//     pub _index: usize,
// }

#[allow(dead_code)]
pub trait AnimatedObjectRenderer {
    // fn set_visible(&mut self, index: usize, is_visible: bool);
    // fn create_from_collada(&mut self, source: &str) -> AnimatedObjectRendererResult;
    fn _set_object_position(&mut self, index: usize, x: f32, y: f32, z: f32);
}
