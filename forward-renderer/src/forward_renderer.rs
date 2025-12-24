

//! Renders everything
//!


// use crate::animated_object_storage::AnimatedObjectStorage;
// use crate::deferred_color_shader::entity_buffer::MousePosition;
// use crate::deferred_color_shader::{self, DeferredShaderDraw, EntityBuffer, GBuffer};
// use crate::deferred_light_shader::DeferredLightShaderDraw;
// use crate::fxaa_shader::FxaaShaderDraw;
// use crate::performance_monitor::PerformanceMonitor;
// use crate::point_light_storage::PointLightStorage;
// use crate::terrain_storage::TerrainStorage;
// use camera_controller::CameraController;
use wgpu_renderer::performance_monitor::watch;
use wgpu_renderer::vertex_color_shader;
use wgpu_renderer::vertex_texture_shader::{self, VertexTextureShaderDraw};
use wgpu_renderer::wgpu_renderer::camera::{Camera, Projection};
use wgpu_renderer::wgpu_renderer::WgpuRendererInterface;
use winit::event::{ElementState, MouseScrollDelta};

use crate::DrawGui;

// use crate::{
//     deferred_animation_shader, deferred_heightmap_shader, deferred_light_shader,
//     deferred_light_sphere_shader, fxaa_shader,
// };

pub struct RendererSettings {
    pub wait_for_render_loop_to_finish: bool,
    pub enable_vertical_sync: bool,
    pub enable_fxaa: bool,
    pub window_resolution: (u32, u32),
}

pub struct ForwardRenderer {
    settings: RendererSettings,

    pipeline_color: vertex_color_shader::Pipeline,
    pipeline_lines: vertex_color_shader::Pipeline,

    pub texture_bind_group_layout: vertex_texture_shader::TextureBindGroupLayout,
    pipeline_texture_gui: vertex_texture_shader::Pipeline,

    // g_buffer_bind_group_layout: deferred_light_shader::GBufferBindGroupLayout,
    // g_buffer: deferred_color_shader::GBuffer,
    // entity_buffer: deferred_color_shader::EntityBuffer,
    // pipeline_deferred_color: deferred_color_shader::Pipeline,

    // pipeline_deferred_light: deferred_light_shader::Pipeline,
    // pipeline_deferred_light_ambient: deferred_light_shader::Pipeline,
    // pipeline_deferred_light_sphere: deferred_light_sphere_shader::Pipeline,

    // pub animation_bind_group_layout: deferred_animation_shader::AnimationBindGroupLayout,
    // pipeline_deferred_animated: deferred_animation_shader::Pipeline,

    // pub heightmap_bind_group_layout: deferred_heightmap_shader::HeightmapBindGroupLayout,
    // pipeline_deferred_heightmap: deferred_heightmap_shader::Pipeline,

    // post_processing_bind_group_layout: fxaa_shader::PostProcessingTextureBindGroupLayout,
    // post_processing_texture: fxaa_shader::PostProcessingTexture,
    // pipeline_fxaa: fxaa_shader::Pipeline,

    // camera
    pub camera: Camera,
    // camera_controller: CameraController,
    pub projection: Projection,

    camera_uniform: vertex_color_shader::CameraUniform,
    camera_uniform_buffer: vertex_color_shader::CameraUniformBuffer,

    camera_uniform_orthographic: vertex_color_shader::CameraUniform,
    camera_uniform_orthographic_buffer: vertex_color_shader::CameraUniformBuffer,
}

impl ForwardRenderer {
    pub fn new(wgpu_renderer: &mut dyn WgpuRendererInterface, settings: RendererSettings) -> Self {
        // enable vsync
        wgpu_renderer.enable_vsync(settings.enable_vertical_sync);
        wgpu_renderer
            .request_window_size(settings.window_resolution.0, settings.window_resolution.1);

        // wgpu renderer
        let surface_width = wgpu_renderer.surface_width();
        let surface_height = wgpu_renderer.surface_height();
        let surface_format: wgpu::TextureFormat = wgpu_renderer.surface_format();

        // pipeline color
        let camera_bind_group_layout =
            vertex_color_shader::CameraBindGroupLayout::new(wgpu_renderer.device());
        let pipeline_color = vertex_color_shader::Pipeline::new(
            wgpu_renderer.device(),
            &camera_bind_group_layout,
            surface_format,
        );

        // pipeline lines
        let pipeline_lines = vertex_color_shader::Pipeline::new_lines(
            wgpu_renderer.device(),
            &camera_bind_group_layout,
            surface_format,
        );

        // pipeline texture gui
        let texture_bind_group_layout =
            vertex_texture_shader::TextureBindGroupLayout::new(wgpu_renderer.device());
        let pipeline_texture_gui = vertex_texture_shader::Pipeline::new_gui(
            wgpu_renderer.device(),
            &camera_bind_group_layout,
            &texture_bind_group_layout,
            surface_format,
        );

        // g_buffer
        // let g_buffer_bind_group_layout =
        //     deferred_light_shader::GBufferBindGroupLayout::new(wgpu_renderer.device());
        // let g_buffer = deferred_color_shader::GBuffer::new(
        //     wgpu_renderer,
        //     &g_buffer_bind_group_layout,
        //     surface_width,
        //     surface_height,
        // );

        // entity_buffer
        // let entity_buffer = deferred_color_shader::EntityBuffer::new(
        //     wgpu_renderer,
        //     surface_width,
        //     surface_height,
        //     settings.enable_memory_mapped_read,
        // );

        // // pipeline deferred color
        // let pipeline_deferred_color = deferred_color_shader::Pipeline::new(
        //     wgpu_renderer.device(),
        //     &camera_bind_group_layout,
        //     surface_format,
        // );

        // // pipeline deferred light
        // let pipeline_deferred_light = deferred_light_shader::Pipeline::new(
        //     wgpu_renderer.device(),
        //     &camera_bind_group_layout,
        //     &g_buffer_bind_group_layout,
        //     surface_format,
        //     false,
        // );

        // let pipeline_deferred_light_ambient = deferred_light_shader::Pipeline::new(
        //     wgpu_renderer.device(),
        //     &camera_bind_group_layout,
        //     &g_buffer_bind_group_layout,
        //     surface_format,
        //     true,
        // );

        // let pipeline_deferred_light_sphere = deferred_light_sphere_shader::Pipeline::new(
        //     wgpu_renderer.device(),
        //     &camera_bind_group_layout,
        //     &g_buffer_bind_group_layout,
        //     surface_format,
        // );

        // let animation_bind_group_layout =
        //     deferred_animation_shader::AnimationBindGroupLayout::new(wgpu_renderer.device());

        // // pipeline deferred animated
        // let pipeline_deferred_animated = deferred_animation_shader::Pipeline::new(
        //     wgpu_renderer.device(),
        //     &camera_bind_group_layout,
        //     &animation_bind_group_layout,
        //     surface_format,
        // );

        // // pipeline deferred heightmap
        // let heightmap_bind_group_layout =
        //     deferred_heightmap_shader::HeightmapBindGroupLayout::new(wgpu_renderer.device());
        // let pipeline_deferred_heightmap = deferred_heightmap_shader::Pipeline::new(
        //     wgpu_renderer.device(),
        //     &camera_bind_group_layout,
        //     &texture_bind_group_layout,
        //     &heightmap_bind_group_layout,
        //     surface_format,
        // );

        // // pipeline fxaa
        // let post_processing_bind_group_layout =
        //     fxaa_shader::PostProcessingTextureBindGroupLayout::new(wgpu_renderer.device());
        // let post_processing_texture = fxaa_shader::PostProcessingTexture::new(
        //     wgpu_renderer,
        //     &post_processing_bind_group_layout,
        //     surface_width,
        //     surface_height,
        //     surface_format,
        // );
        // let pipeline_fxaa = fxaa_shader::Pipeline::new(
        //     wgpu_renderer.device(),
        //     &camera_bind_group_layout,
        //     &post_processing_bind_group_layout,
        //     surface_format,
        // );

        // camera
        let position = cgmath::Point3::new(0.0, 0.0, 0.0);
        let yaw = cgmath::Deg(0.0);
        let pitch = cgmath::Deg(0.0);
        let mut camera = Camera::new(position, yaw, pitch);
        // Self::top_view_point(&mut camera);
        // Self::side_view_point(&mut camera);

        let speed = 40.0;
        let sensitivity = 1.0;
        let sensitivity_scroll = 1.0;
        // let camera_controller = CameraController::new(speed, sensitivity, sensitivity_scroll);

        let width = wgpu_renderer.surface_width();
        let height = wgpu_renderer.surface_height();
        let fovy = cgmath::Deg(45.0);
        let znear = 0.1;
        let zfar = 100.0;
        let projection = Projection::new(width, height, fovy, znear, zfar);

        let camera_uniform = vertex_color_shader::CameraUniform::new();

        let camera_uniform_buffer = vertex_color_shader::CameraUniformBuffer::new(
            wgpu_renderer.device(),
            &camera_bind_group_layout,
        );

        let camera_uniform_orthographic: vertex_color_shader::CameraUniform =
            vertex_color_shader::CameraUniform::new_orthographic(width, height);
        let mut camera_uniform_orthographic_buffer = vertex_color_shader::CameraUniformBuffer::new(
            wgpu_renderer.device(),
            &camera_bind_group_layout,
        );

        camera_uniform_orthographic_buffer
            .update(wgpu_renderer.queue(), camera_uniform_orthographic); // add uniform identity matrix

        Self {
            settings,

            pipeline_color,
            pipeline_lines,

            texture_bind_group_layout,
            pipeline_texture_gui,

            // g_buffer_bind_group_layout,
            // g_buffer,
            // // entity_buffer,

            // pipeline_deferred_color,
            // pipeline_deferred_light,
            // pipeline_deferred_light_ambient,
            // pipeline_deferred_light_sphere,

            // animation_bind_group_layout,
            // pipeline_deferred_animated,

            // heightmap_bind_group_layout,
            // pipeline_deferred_heightmap,

            // post_processing_bind_group_layout,
            // post_processing_texture,
            // pipeline_fxaa,

            camera,
            // camera_controller,
            projection,

            camera_uniform,
            camera_uniform_buffer,

            camera_uniform_orthographic,
            camera_uniform_orthographic_buffer,
        }
    }

    // fn _top_view_point(camera: &mut Camera) {
    //     let position = cgmath::Point3::new(0.0, 0.0, 10.0);
    //     let yaw = cgmath::Deg(-90.0).into();
    //     let pitch = cgmath::Deg(0.0).into();

    //     camera.position = position;
    //     camera.yaw = yaw;
    //     camera.pitch = pitch;
    // }

    // fn side_view_point(camera: &mut Camera) {
    //     let position = cgmath::Point3::new(0.0, 5.0, 12.0);
    //     let yaw = cgmath::Deg(-90.0).into();
    //     let pitch = cgmath::Deg(60.0).into();

    //     camera.position = position;
    //     camera.yaw = yaw;
    //     camera.pitch = pitch;
    // }

    pub fn resize(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        new_size: winit::dpi::PhysicalSize<u32>,
    ) {
        // self.size = new_size;

        self.projection.resize(new_size.width, new_size.height);
        // self.wgpu_renderer.resize(new_size);
        // self.g_buffer = GBuffer::new(
        //     renderer_interface,
        //     &self.g_buffer_bind_group_layout,
        //     new_size.width,
        //     new_size.height,
        // );

        // self.entity_buffer = EntityBuffer::new(
        //     renderer_interface,
        //     new_size.width,
        //     new_size.height,
        //     self.settings.enable_memory_mapped_read,
        // );

        let surface_format = renderer_interface.surface_format();
        // self.post_processing_texture = fxaa_shader::PostProcessingTexture::new(
        //     renderer_interface,
        //     &self.post_processing_bind_group_layout,
        //     new_size.width,
        //     new_size.height,
        //     surface_format,
        // );

        self.camera_uniform_orthographic
            .resize_orthographic(new_size.width, new_size.height);
        self.camera_uniform_orthographic_buffer
            .update(renderer_interface.queue(), self.camera_uniform_orthographic);
    }

    pub fn update(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        dt: instant::Duration,
    ) {
        // camera
        // self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform
            .update_view_proj(&self.camera, &self.projection);
        self.camera_uniform_buffer
            .update(renderer_interface.queue(), self.camera_uniform);
    }

    // pub fn process_keyboard(&mut self, key: winit::keyboard::KeyCode, state: ElementState) -> bool {
    //     self.camera_controller.process_keyboard(key, state)
    // }

    // pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
    //     self.camera_controller.process_scroll(delta);
    // }

    pub fn get_view_position(&self) -> cgmath::Vector3<f32> {
        self.camera.get_view_position()
    }

    pub fn _get_view_direction(&self) -> cgmath::Vector3<f32> {
        self.camera.get_view_direction()
    }

    // #[allow(clippy::too_many_arguments)]
    // fn render_deferred(
    //     &self,
    //     renderer_interface: &mut dyn WgpuRendererInterface,
    //     _view: &wgpu::TextureView,
    //     encoder: &mut wgpu::CommandEncoder,
    //     // meshes: &[&dyn DeferredShaderDraw],
    //     // ant_light_orbs: &dyn DeferredShaderDraw,
    //     // terrain_storage: &mut TerrainStorage,
    //     // animated_object_storage: &AnimatedObjectStorage,
    // ) {
    //     let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //         label: Some("Deferred Render Pass"),
    //         color_attachments: &[
    //             //     Some(wgpu::RenderPassColorAttachment {
    //             //         view,
    //             //         resolve_target: None,
    //             //         ops: wgpu::Operations {
    //             //             load: wgpu::LoadOp::Clear(wgpu::Color {
    //             //                 r: 0.00,
    //             //                 g: 0.00,
    //             //                 b: 0.00,
    //             //                 a: 1.0,
    //             //             }),
    //             //             store: wgpu::StoreOp::default(),
    //             //         },
    //             //     }),
    //             // None,
    //             // None,
    //             Some(wgpu::RenderPassColorAttachment {
    //                 view: &self.g_buffer.position.view,
    //                 resolve_target: None,
    //                 ops: wgpu::Operations {
    //                     load: wgpu::LoadOp::Clear(wgpu::Color {
    //                         r: 0.00,
    //                         g: 0.00,
    //                         b: 0.00,
    //                         a: 1.0,
    //                     }),
    //                     store: wgpu::StoreOp::default(),
    //                 },
    //             }),
    //             Some(wgpu::RenderPassColorAttachment {
    //                 view: &self.g_buffer.normal.view,
    //                 resolve_target: None,
    //                 ops: wgpu::Operations {
    //                     load: wgpu::LoadOp::Clear(wgpu::Color {
    //                         r: 0.00,
    //                         g: 0.00,
    //                         b: 0.00,
    //                         a: 1.0,
    //                     }),
    //                     store: wgpu::StoreOp::default(),
    //                 },
    //             }),
    //             Some(wgpu::RenderPassColorAttachment {
    //                 view: &self.g_buffer.albedo.view,
    //                 resolve_target: None,
    //                 ops: wgpu::Operations {
    //                     load: wgpu::LoadOp::Clear(wgpu::Color {
    //                         r: 0.00,
    //                         g: 0.00,
    //                         b: 0.00,
    //                         a: 1.0,
    //                     }),
    //                     store: wgpu::StoreOp::default(),
    //                 },
    //             }),
    //             // Some(wgpu::RenderPassColorAttachment {
    //             //     view: &self.entity_buffer.view,
    //             //     resolve_target: None,
    //             //     ops: wgpu::Operations {
    //             //         load: wgpu::LoadOp::Clear(wgpu::Color {
    //             //             r: 0.00,
    //             //             g: 0.00,
    //             //             b: 0.00,
    //             //             a: 1.0,
    //             //         }),
    //             //         store: wgpu::StoreOp::default(),
    //             //     },
    //             // }),
    //         ],
    //         depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
    //             view: renderer_interface.get_depth_texture_view(),
    //             depth_ops: Some(wgpu::Operations {
    //                 load: wgpu::LoadOp::Clear(1.0),
    //                 store: wgpu::StoreOp::default(),
    //             }),
    //             stencil_ops: None,
    //         }),
    //         timestamp_writes: None,
    //         occlusion_query_set: None,
    //     });

    //     // ants
    //     self.pipeline_deferred_animated.draw(
    //         &mut render_pass,
    //         &self.camera_uniform_buffer,
    //         animated_object_storage,
    //     );

    //     // light orbs (shining through the ants)
    //     self.pipeline_deferred_color.draw(
    //         &mut render_pass,
    //         &self.camera_uniform_buffer,
    //         ant_light_orbs,
    //     );

    //     // terrain
    //     self.pipeline_deferred_heightmap.draw(
    //         &mut render_pass,
    //         &self.camera_uniform_buffer,
    //         terrain_storage,
    //     );

    //     // other
    //     for mesh in meshes {
    //         self.pipeline_deferred_color
    //             .draw(&mut render_pass, &self.camera_uniform_buffer, *mesh);
    //     }
    // }

    // fn render_light(
    //     &self,
    //     renderer_interface: &mut dyn WgpuRendererInterface,
    //     view: &wgpu::TextureView,
    //     encoder: &mut wgpu::CommandEncoder,
    //     meshes: &[&dyn DeferredLightShaderDraw],
    //     ambient_light_quad: &impl DeferredLightShaderDraw,
    //     point_light_storage: &PointLightStorage,
    // ) {
    //     let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //         label: Some("Light Render Pass"),
    //         color_attachments: &[Some(wgpu::RenderPassColorAttachment {
    //             view,
    //             resolve_target: None,
    //             ops: wgpu::Operations {
    //                 // load: wgpu::LoadOp::Load,
    //                 load: wgpu::LoadOp::Clear(wgpu::Color {
    //                     r: 0.00,
    //                     g: 0.00,
    //                     b: 0.00,
    //                     a: 1.0,
    //                 }),
    //                 store: wgpu::StoreOp::default(),
    //             },
    //         })],
    //         depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
    //             view: renderer_interface.get_depth_texture_view(),
    //             depth_ops: Some(wgpu::Operations {
    //                 // load: wgpu::LoadOp::Clear(1.0),
    //                 load: wgpu::LoadOp::Load,
    //                 store: wgpu::StoreOp::default(),
    //             }),
    //             stencil_ops: None,
    //         }),
    //         timestamp_writes: None,
    //         occlusion_query_set: None,
    //     });

    //     // ambient light
    //     self.pipeline_deferred_light_ambient.draw(
    //         &mut render_pass,
    //         &self.camera_uniform_buffer,
    //         &self.g_buffer,
    //         ambient_light_quad,
    //     );

    //     // lights
    //     for mesh in meshes {
    //         self.pipeline_deferred_light.draw(
    //             &mut render_pass,
    //             &self.camera_uniform_buffer,
    //             &self.g_buffer,
    //             *mesh,
    //         );
    //     }

    //     // point lights
    //     self.pipeline_deferred_light.draw(
    //         &mut render_pass,
    //         &self.camera_uniform_buffer,
    //         &self.g_buffer,
    //         point_light_storage,
    //     );

    //     // debug spheres
    //     self.pipeline_deferred_light_sphere.draw(
    //         &mut render_pass,
    //         &self.camera_uniform_buffer,
    //         &self.g_buffer,
    //         point_light_storage,
    //     );
    // }

    // fn render_fxaa(
    //     &self,
    //     view: &wgpu::TextureView,
    //     encoder: &mut wgpu::CommandEncoder,
    //     mesh: &dyn FxaaShaderDraw,
    // ) {
    //     let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //         label: Some("FXAA Render Pass"),
    //         color_attachments: &[Some(wgpu::RenderPassColorAttachment {
    //             view,
    //             resolve_target: None,
    //             ops: wgpu::Operations {
    //                 // load: wgpu::LoadOp::Load,
    //                 load: wgpu::LoadOp::Clear(wgpu::Color {
    //                     r: 0.00,
    //                     g: 0.00,
    //                     b: 0.00,
    //                     a: 1.0,
    //                 }),
    //                 store: wgpu::StoreOp::default(),
    //             },
    //         })],
    //         depth_stencil_attachment: None,
    //         timestamp_writes: None,
    //         occlusion_query_set: None,
    //     });

    //     self.pipeline_fxaa.draw(
    //         &mut render_pass,
    //         &self.camera_uniform_buffer,
    //         &self.post_processing_texture,
    //         mesh,
    //     );
    // }

    fn render_forward(
        &self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        // textured_meshes: &impl VertexTextureShaderDraw,
        gui_elements: &[&mut dyn DrawGui],
        // performance_monitors: &[&mut PerformanceMonitor<{ super::WATCH_POINTS_SIZE }>],
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Forward Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    // load: wgpu::LoadOp::Load,
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.01,
                        g: 0.01,
                        b: 0.01,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::default(),
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: renderer_interface.get_depth_texture_view(),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    // load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::default(),
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // performance monitor
        for elem in gui_elements {
            self.pipeline_lines.draw_lines(
                &mut render_pass,
                &self.camera_uniform_orthographic_buffer,
                *elem,
            );
        }

        for elem in gui_elements {
            self.pipeline_color.draw(
                &mut render_pass,
                &self.camera_uniform_orthographic_buffer,
                *elem,
            );
        }

        for elem in gui_elements {
            self.pipeline_texture_gui.draw(
                &mut render_pass,
                &self.camera_uniform_orthographic_buffer,
                *elem,
            );
        }

        // textured meshes
        // self.pipeline_texture_gui.draw(
        //     &mut render_pass,
        //     &self.camera_uniform_orthographic_buffer,
        //     textured_meshes,
        // );
    }

    // pub fn read_entity_index(&mut self) -> u32 {
    //     {
    //         // self.entity_buffer.read_pixel()
    //         0
    //     }
    // }

    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        // deferred: & impl DeferredShaderDraw,
        // deferred_light: & impl DeferredLightShaderDraw,
        // deferred_combined: &(impl DeferredShaderDraw + DeferredLightShaderDraw),
        // animated_object_storage: &AnimatedObjectStorage,
        // point_light_storage: &PointLightStorage,
        // terrain_storage: &mut TerrainStorage,

        // ant_light_orbs: &(impl DeferredShaderDraw + DeferredLightShaderDraw),
        // mesh_textured_gui: &impl VertexTextureShaderDraw,
        // ambient_light_quad: &deferred_light_shader::Mesh,

        gui_elements: &[&mut dyn DrawGui],
        // watch_fps: &mut watch::Watch<{ super::WATCH_POINTS_SIZE }>,
        // mouse_position: MousePosition,
    ) -> Result<(), wgpu::SurfaceError> {
        // watch_fps.start(0, "Wait for next frame");
        let output = renderer_interface.get_current_texture()?;
        // watch_fps.stop(0);

        // watch_fps.start(1, "Draw");

        let view: wgpu::TextureView = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder: wgpu::CommandEncoder =
            renderer_interface
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        // draw
        // self.render_deferred(
        //     renderer_interface,
        //     &view,
        //     &mut encoder,
        //     &[],
        //     ant_light_orbs,
        //     terrain_storage,
        //     animated_object_storage,
        // );

        // self.render_light(
        //     renderer_interface,
        //     if self.settings.enable_fxaa {
        //         &self.post_processing_texture.view
        //     } else {
        //         &view
        //     },
        //     &mut encoder,
        //     &[ant_light_orbs],
        //     ambient_light_quad,
        //     point_light_storage,
        // );

        // if self.settings.enable_fxaa {
        //     self.render_fxaa(&view, &mut encoder, ambient_light_quad);
        // }

        self.render_forward(
            renderer_interface,
            &view,
            &mut encoder,
            // mesh_textured_gui,
            gui_elements,
        );

        // copy entity texture
        // self.entity_buffer
        //     .copy_texture_to_buffer(&mut encoder, mouse_position);

        renderer_interface
            .queue()
            .submit(std::iter::once(encoder.finish()));
        output.present();

        // map entity texture to the host
        // self.entity_buffer.map_buffer_async();

        // wait to see how high the gpu load is
        if self.settings.wait_for_render_loop_to_finish {
            renderer_interface.device().poll(wgpu::Maintain::Wait);
        } else {
            renderer_interface.device().poll(wgpu::Maintain::Poll);
        }

        // watch_fps.stop(1);

        Ok(())
    }
}
