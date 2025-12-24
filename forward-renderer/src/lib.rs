
mod draw_gui;
mod forward_renderer;
mod camera_controller;
mod performance_monitor;
pub use forward_renderer::ForwardRenderer;
pub use forward_renderer::RendererSettings;
pub use performance_monitor::PerformanceMonitor;
pub use draw_gui::DrawGui;



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
