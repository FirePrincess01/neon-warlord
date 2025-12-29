//! Application settings

use forward_renderer::{RendererSettings, TerrainSettings};

use crate::{CameraSettings, ObjectSettings};

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
            nr_tiles: 32,
            max_depth: 4,
        }
    }

    pub fn get_object_settings(&self) -> ObjectSettings {
        ObjectSettings { max_nr_ants: 9 }
    }

    pub(crate) fn get_camera_settings(&self) -> CameraSettings {
        CameraSettings {
            speed: 20.0,
            sensitivity: 1.0,
            sensitivity_scroll: 1.0,
        }
    }
}
