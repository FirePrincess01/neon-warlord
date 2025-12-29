//! Library to instantiate a forward rendering pipeline

mod animated_object;
pub mod animation_shader;
mod draw_gui;
mod forward_renderer;
pub mod geometry;
pub mod lod_heightmap_shader;
mod performance_monitor;
mod terrain_storage;
pub use animated_object::animated_object_storage::AnimatedObjectStorage;
pub use draw_gui::DrawGui;
pub use forward_renderer::ForwardRenderer;
pub use forward_renderer::RendererSettings;
pub use performance_monitor::PerformanceMonitor;
pub use terrain_storage::HeightMap;
pub use terrain_storage::TerrainSettings;
pub use terrain_storage::TerrainStorage;
pub use terrain_storage::terrain_texture_details::TerrainTextureDetails;
