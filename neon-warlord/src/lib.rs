//! Creates the Neon-Warlord application

mod ant_generator;
mod ant_storage;
mod camera_controller;
mod debug_overlay;
mod heightmap_generator;
mod settings;
mod sun_storage;
use forward_renderer::{
    AnimatedObjectStorage, ForwardRenderer, PerformanceMonitor, TerrainStorage,
    particle_storage::ParticleStorage,
};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use wgpu_renderer::{
    default_application::{DefaultApplication, DefaultApplicationInterface},
    performance_monitor::{Fps, watch::Watch},
    wgpu_renderer::WgpuRendererInterface,
};
use winit::event::{ElementState, WindowEvent};

use crate::{
    ant_generator::AntGenerator, ant_storage::AntStorage, camera_controller::CameraController,
    debug_overlay::DebugOverlay, heightmap_generator::HeightMapGenerator, sun_storage::SunStorage,
};

const WATCH_POINTS_SIZE: usize = 7;
const DEBUG_OVERLAY_SIZE: usize = 10;

struct ObjectSettings {
    pub max_nr_ants: usize,
}

struct CameraSettings {
    speed: f32,
    sensitivity: f32,
    sensitivity_scroll: f32,
}

struct NeonWarlord {
    _settings: settings::Settings,

    // Render engine
    size: winit::dpi::PhysicalSize<u32>,
    scale_factor: f32,

    renderer: ForwardRenderer,
    font: rusttype::Font<'static>,

    camera_controller: CameraController,

    // Debug Utilities
    fps: Fps,
    watch_fps: Watch<WATCH_POINTS_SIZE>,
    performance_monitor_fps: PerformanceMonitor<WATCH_POINTS_SIZE>,
    performance_monitor_ups: PerformanceMonitor<WATCH_POINTS_SIZE>,
    debug_overlay: DebugOverlay<DEBUG_OVERLAY_SIZE>,
    mouse_pos_y: u32,
    mouse_pos_x: u32,

    // debug
    device_id: String,
    phase: String,
    location: String,
    force: String,
    id: String,

    // Terrain
    terrain: TerrainStorage,
    terrain_generator: HeightMapGenerator,

    // Ants
    ants: AntStorage,
    _ant_generator: AntGenerator,

    // Sun
    sun: SunStorage,

    // Particles
    particles: ParticleStorage,
}

impl NeonWarlord {
    pub fn new(
        renderer_interface: &mut dyn WgpuRendererInterface,
        size: winit::dpi::PhysicalSize<u32>,
        scale_factor: f32,
    ) -> Self {
        let settings = settings::Settings::new();

        let renderer = ForwardRenderer::new(renderer_interface, settings.get_renderer_settings());

        // font
        let font = wgpu_renderer::freefont::create_font_free_mono();

        // Camera
        let camera_controller = CameraController::new(
            settings.get_camera_settings().speed,
            settings.get_camera_settings().sensitivity,
            settings.get_camera_settings().sensitivity_scroll,
        );

        // world
        // let mut world = ecs2::World::new();

        // let blue_token = world.resources.blues2.create(0.0, 1.0, 1.0);

        // world
        //     .base_factory
        //     .add_blue(blue_token, &mut world.resources);
        // world
        //     .base_factory
        //     .produce(&mut world.resources, &mut world.agents);

        // world mesh
        // let _world_mesh = world_mesh::WorldMesh::new(renderer_interface, &world);

        // performance monitor
        let watch_fps = Watch::new();
        let performance_monitor_fps = PerformanceMonitor::new(
            renderer_interface,
            &renderer.texture_bind_group_layout,
            &font,
            colorous::RAINBOW,
            true,
            "60 fps / 16.66 ms",
            scale_factor,
        );
        let performance_monitor_ups = PerformanceMonitor::new(
            renderer_interface,
            &renderer.texture_bind_group_layout,
            &font,
            colorous::PLASMA,
            false,
            "60 ups / 16.66 ms",
            scale_factor,
        );
        let fps = Fps::new();
        let debug_overlay = DebugOverlay::new(
            renderer_interface,
            &renderer.texture_bind_group_layout,
            &font,
            size.height,
            size.width,
            scale_factor,
        );

        // Mouse position
        let _mouse_pos_y = 0;
        let _mouse_pos_x = 0;

        // Debug Overlay
        // let debug_overlay = DebugOverlay::new(
        //     renderer_interface,
        //     &renderer.texture_bind_group_layout,
        //     &font,
        //     cgmath::Vector3 {
        //         x: 20.0,
        //         y: 120.0,
        //         z: 0.0,
        //     },
        // );

        // create ant
        // let glb_bin = include_bytes!("../res/wiggle_tower2.glb");
        let glb_bin = include_bytes!("../res/ant_0_8.glb");
        let animated_object_storage_ant = AnimatedObjectStorage::create_from_glb(
            renderer_interface,
            &renderer.animation_bind_group_layout,
            glb_bin,
            settings.get_object_settings().max_nr_ants,
        );

        // println!("{:?}", animated_object_storage_ant);

        // let point_light_storage_ant = PointLightStorage::new(
        //     renderer_interface,
        //     settings.max_nr_ants,
        //     settings.dbg_point_lights,
        // );

        let mut ants = AntStorage::new(
            // point_light_storage_ant,
            animated_object_storage_ant,
            settings.get_object_settings().max_nr_ants,
        );

        let ant_generator = AntGenerator::new(settings.get_object_settings().max_nr_ants);
        for elem in &ant_generator.ants {
            ants.set_ant(elem);
        }

        // ant_storage.set_ant(&Ant{
        //     id: todo!(),
        //     pos: todo!(),
        //     rot_z: todo!(),
        //     light_strength: todo!(),
        //     light_color: todo!(),
        // });

        // create game server
        // let game_logic =
        //     market_economy_simulation_server::GameLogicServer::new(settings.get_server_settings());

        // create ant
        // let ant = ant::Ant::new(renderer_interface);

        // let ambient_light_quad_vertices = geometry::Quad::new(2.0);
        // let ambient_light_quad_instance = deferred_light_shader::Instance {
        //     position: [-1.0, -1.0, 0.1],
        //     light_color: [0.4, 0.4, 0.4],
        //     radius: 0.0,
        //     linear: 0.0,
        //     quadratic: 0.0,
        // };
        // let ambient_light_quad = deferred_light_shader::Mesh::new(
        //     renderer_interface.device(),
        //     &ambient_light_quad_vertices.vertices,
        //     &ambient_light_quad_vertices.indices,
        //     &[ambient_light_quad_instance],
        // );

        // // point light storage
        // let point_light_storage = point_light_storage::PointLightStorage::new(
        //     renderer_interface,
        //     settings.max_point_light_instances,
        //     settings.dbg_point_lights,
        // );

        // terrain
        let terrain = TerrainStorage::new(
            settings.get_terrain_settings(),
            renderer_interface,
            &renderer.texture_bind_group_layout,
            include_bytes!("../res/tile.png"),
        );
        let terrain_generator = heightmap_generator::HeightMapGenerator::new();

        // selector
        // let selector = Selector::new();

        // sun
        let sun = SunStorage::new(renderer_interface);

        // Particles
        let particles = ParticleStorage::new(renderer_interface);

        Self {
            _settings: settings,
            size,
            scale_factor,
            renderer,
            font,
            watch_fps,
            performance_monitor_fps,
            performance_monitor_ups,
            mouse_pos_y: 0,
            mouse_pos_x: 0,
            fps,
            debug_overlay,
            terrain,
            terrain_generator,
            ants,
            _ant_generator: ant_generator,
            camera_controller,
            device_id: String::new(),
            phase: String::new(),
            location: String::new(),
            force: String::new(),
            id: String::new(),
            sun,
            particles,
            // settings,

            // size,
            // scale_factor,

            // renderer,

            // _world: world,
            // // world_mesh,
            // ant_storage,

            // watch_fps,
            // performance_monitor_fps,
            // performance_monitor_ups,

            // mouse_pos_y,
            // mouse_pos_x,
            // entity_index: 0,
            // font,

            // debug_overlay,

            // game_logic,

            // ant,

            // ambient_light_quad,
            // // point_light_storage,
            // terrain_storage,

            // selector,
        }
    }
}

#[allow(unused)]
fn apply_scale_factor(
    position: winit::dpi::PhysicalPosition<f64>,
    scale_factor: f32,
) -> winit::dpi::PhysicalPosition<f64> {
    cfg_if::cfg_if! {
        // apply scale factor for the web
        if #[cfg(target_arch = "wasm32")] {
            let mut res = position;
            res.x = res.x / scale_factor as f64;
            res.y = res.y / scale_factor as f64;
            res
        }
        else {
            position
        }
    }
}

impl DefaultApplicationInterface for NeonWarlord {
    fn create(
        renderer_interface: &mut dyn wgpu_renderer::wgpu_renderer::WgpuRendererInterface,
        size: winit::dpi::PhysicalSize<u32>,
        scale_factor: f32,
    ) -> Self {
        Self::new(renderer_interface, size, scale_factor)
    }

    fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    fn resize(
        &mut self,
        renderer_interface: &mut dyn wgpu_renderer::wgpu_renderer::WgpuRendererInterface,
        new_size: winit::dpi::PhysicalSize<u32>,
    ) {
        println!("resize: {:?}", new_size);

        self.size = new_size;
        self.renderer.resize(renderer_interface, new_size);
        self.debug_overlay.resize(
            renderer_interface,
            &self.renderer.texture_bind_group_layout,
            &self.font,
            new_size.height,
            new_size.width,
        );
    }

    fn update_scale_factor(
        &mut self,
        renderer_interface: &mut dyn wgpu_renderer::wgpu_renderer::WgpuRendererInterface,
        scale_factor: f32,
    ) {
        println!("new scale factor {}", scale_factor);

        // let scale_factor = 2.0;
        self.scale_factor = scale_factor;
        self.performance_monitor_fps.rescale(
            renderer_interface,
            &self.renderer.texture_bind_group_layout,
            &self.font,
            scale_factor,
        );
        self.performance_monitor_ups.rescale(
            renderer_interface,
            &self.renderer.texture_bind_group_layout,
            &self.font,
            scale_factor,
        );

        self.debug_overlay.rescale(
            renderer_interface,
            &self.renderer.texture_bind_group_layout,
            &self.font,
            scale_factor,
        );
    }

    fn update(
        &mut self,
        renderer_interface: &mut dyn wgpu_renderer::wgpu_renderer::WgpuRendererInterface,
        dt: instant::Duration,
    ) {
        // Render engine
        self.camera_controller
            .update_camera(&mut self.renderer.camera, dt);
        self.renderer.update(renderer_interface, dt);

        // Particles
        self.watch_fps.start(5, "Update particles");
        {
            self.particles.update(renderer_interface, dt);
        }
        self.watch_fps.stop(5);

        // Animations
        self.watch_fps.start(4, "Update animations");
        {
            self.ants.animated_object_storage.update_animations(&dt);

            self.ants
                .animated_object_storage
                .update_device_data(renderer_interface);
        }
        self.watch_fps.stop(4);

        // Terrain
        self.watch_fps.start(3, "Update terrain");
        {
            // set terrain view position
            self.terrain
                .set_view_position(&self.renderer.get_view_position());

            // generate map
            let requests = self.terrain.get_requests().clone();
            for request in requests {
                let terrain_part = self.terrain_generator.generate(&request);
                self.terrain.update_height_map(
                    renderer_interface,
                    &self.renderer.heightmap_bind_group_layout,
                    terrain_part,
                );
            }
            self.terrain.clear_requests();
        }
        self.watch_fps.stop(3);

        // Debug utilities
        self.watch_fps.update();
        self.watch_fps.start(0, "Debug utilities");
        {
            self.fps.update(dt);

            self.debug_overlay.update_val(
                renderer_interface,
                &self.font,
                0,
                "fps",
                self.fps.get() as f32,
            );

            self.debug_overlay.update_val(
                renderer_interface,
                &self.font,
                1,
                "x",
                self.mouse_pos_x as f32,
            );

            self.debug_overlay.update_val(
                renderer_interface,
                &self.font,
                2,
                "y",
                self.mouse_pos_y as f32,
            );

            self.debug_overlay
                .update_str(renderer_interface, &self.font, 3, &self.device_id);

            self.debug_overlay
                .update_str(renderer_interface, &self.font, 4, &self.phase);

            self.debug_overlay
                .update_str(renderer_interface, &self.font, 5, &self.location);

            self.debug_overlay
                .update_str(renderer_interface, &self.font, 6, &self.force);

            self.debug_overlay
                .update_str(renderer_interface, &self.font, 7, &self.id);

            self.performance_monitor_fps.update_from_data(
                renderer_interface,
                &self.font,
                &self.watch_fps.get_viewer_data(),
            );
        }
        self.watch_fps.stop(0);
    }

    fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
        self.watch_fps.start(2, "Process user inputs");

        let res = match event {
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key:
                            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F1),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.performance_monitor_ups.show = false;
                self.performance_monitor_fps.show = !self.performance_monitor_fps.show;
                true
            }
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key:
                            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F2),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.performance_monitor_fps.show = false;
                self.performance_monitor_ups.show = !self.performance_monitor_ups.show;
                true
            }
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        // virtual_keycode: Some(key),
                        physical_key: winit::keyboard::PhysicalKey::Code(key),
                        state,
                        ..
                    },
                ..
            } => {
                // self.renderer.process_keyboard(*key, *state),
                self.camera_controller.process_keyboard(key, state)
            }
            WindowEvent::MouseWheel { delta, .. } => {
                // self.renderer.process_scroll(delta);
                self.camera_controller.process_scroll(delta);
                true
            }
            WindowEvent::CursorMoved { position, .. } => {
                let pos = apply_scale_factor(*position, self.scale_factor);

                self.mouse_pos_y = self.size.height.saturating_sub(pos.y as u32);
                self.mouse_pos_x = pos.x as u32;
                true
            }
            winit::event::WindowEvent::Touch(touch) => {
                self.device_id = format!("{:?}", touch.device_id);
                self.phase = format!("{:?}", touch.phase);
                self.location = format!("{:?}", touch.location);
                self.force = format!("{:?}", touch.force);
                self.id = format!("{:?}", touch.id);
                true
            }

            _ => false,
        };
        self.watch_fps.stop(2);

        res
    }

    fn render(
        &mut self,
        renderer_interface: &mut dyn wgpu_renderer::wgpu_renderer::WgpuRendererInterface,
    ) -> Result<(), wgpu::SurfaceError> {
        // render current frame
        let res;
        self.watch_fps.start(1, "Draw");
        {
            res = self.renderer.render(
                renderer_interface,
                &mut self.terrain,
                &[&self.ants.animated_object_storage],
                &[
                    &self.performance_monitor_fps,
                    &self.performance_monitor_ups,
                    &self.debug_overlay,
                ],
                &[&self.sun],
                &[&self.particles],
            )
        }
        self.watch_fps.stop(1);

        res
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    let event_loop = winit::event_loop::EventLoop::with_user_event()
        .build()
        .expect("Creating the event loop failed");
    let mut application: DefaultApplication<NeonWarlord> = DefaultApplication::new(&event_loop);
    event_loop.run_app(&mut application).unwrap();
}
