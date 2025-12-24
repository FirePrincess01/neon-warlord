use forward_renderer::RendererSettings;



pub struct Settings {
    
}
impl Settings {
    pub(crate) fn new() -> Self {
        Self {  }
    }
    
    pub(crate) fn get_renderer_settings(&self) -> RendererSettings {
        RendererSettings {
            wait_for_render_loop_to_finish: true,
            enable_vertical_sync: false,
            enable_fxaa: false,
            window_resolution: (1920 / 2, 1080 / 2),
        }
    }
}