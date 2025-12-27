mod camera_controller;
mod draw_gui;
mod forward_renderer;
mod lod_heightmap_shader;
mod performance_monitor;
mod terrain_storage;
pub use draw_gui::DrawGui;
pub use forward_renderer::ForwardRenderer;
pub use forward_renderer::RendererSettings;
pub use performance_monitor::PerformanceMonitor;
pub use terrain_storage::HeightMap;
pub use terrain_storage::TerrainSettings;
pub use terrain_storage::TerrainStorage;
pub use terrain_storage::terrain_texture_details::TerrainTextureDetails;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
