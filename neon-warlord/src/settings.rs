//! Application settings

use forward_renderer::{RendererSettings, TerrainSettings};

pub struct Settings {}
impl Settings {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_renderer_settings(&self) -> RendererSettings {
        RendererSettings {
            wait_for_render_loop_to_finish: true,
            enable_vertical_sync: false,
            enable_fxaa: false,
            window_resolution: (1920 / 2, 1080 / 2),
        }
    }

    pub fn get_terrain_settings(&self) -> TerrainSettings {
        TerrainSettings {
            nr_tiles: 16,
            max_depth: 8,
        }
    }
}
