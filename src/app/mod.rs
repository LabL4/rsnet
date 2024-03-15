pub mod camera;
pub mod event_loop;
pub mod utils;
pub mod state;

use camera::CameraController;

use self::utils::create_multisampled_framebuffer;
pub use self::state::State;
use crate::{
    gui::{self, renderer::GuiRenderer},
    renderer::Renderer,
    scene,
    utils::{frame_counter, wgpu::{context::Context, surface::SurfaceWrapper}, FrameCounter},
};

use egui_wgpu::ScreenDescriptor;
use std::{iter, sync::Arc};
use wgpu::{CommandEncoderDescriptor, TextureViewDescriptor};
use winit::window::Window;


pub struct App<'a> {
    pub gui_renderer: Option<GuiRenderer>,
    pub scene_renderer: Option<Renderer<'a>>,

    pub msaa_view: Option<wgpu::TextureView>,
    // pub ms_bundle: Option<wgpu::RenderBundle>,
    pub surface: SurfaceWrapper<'a>,
    pub context: Context,
    window: Arc<Window>,

    camera_controller: CameraController,

    pub state: State,
    pub ui_state: gui::state::State,

    frame_counter: FrameCounter,
}

impl<'a> App<'a> {
    pub fn new(
        gui_renderer: Option<GuiRenderer>,
        scene_renderer: Option<Renderer<'a>>,
        surface: SurfaceWrapper<'a>,
        context: Context,
        window: Arc<Window>,
    ) -> Self {
        let camera_controller = CameraController::new(window.inner_size());
        let frame_counter = FrameCounter::new();

        Self {
            gui_renderer,
            scene_renderer,
            msaa_view: None,
            surface,
            context,
            window,
            state: State::default(),
            ui_state: gui::state::State::default(),
            camera_controller,
            frame_counter
        }
    }

    pub fn window_event_handler(&mut self, event: winit::event::WindowEvent) {
        if self.gui_renderer.is_some()
            && self
                .gui_renderer
                .as_mut()
                .unwrap()
                .handle_input(&self.window, &event)
                .consumed
        {}

        self.camera_controller.event_handler(event);
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if self.msaa_view.is_some() {
            self.create_msaa_view(self.state.msaa_count());
        }
        self.camera_controller.resize(size);
    }

    pub fn render(&mut self) {
        if self.state.rebuild_bundles() {
            if self.state.msaa_count() != 1 {
                self.create_msaa_view(self.state.msaa_count());
            } else {
                self.msaa_view = None;
            }
            if let Some(scene_renderer) = &mut self.scene_renderer {
                scene_renderer.set_msaa_count(self.state.msaa_count());
                scene_renderer.rebuild_pipelines(self.surface.config(), &self.context.device);
            }   
            self.state.set_rebuild_bundles(false);
        }

        let frame = self.surface.acquire(&self.context);

        let view = frame.texture.create_view(&TextureViewDescriptor {
            format: Some(self.surface.config().view_formats[0]),
            ..TextureViewDescriptor::default()
        });

        let mut encoder = self
            .context
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let window_size = self.window.inner_size();

        if let Some(scene_renderer) = &mut self.scene_renderer {
            scene_renderer.render(
                self.msaa_view.as_ref(),
                // None,
                &view,
                &self.context,
                &mut encoder,
                &mut self.camera_controller,
                &self.state.scene,
            );
        }

        if let Some(gui_renderer) = &mut self.gui_renderer {
            gui_renderer.draw(
                &self.context.device,
                &self.context.queue,
                &mut encoder,
                &self.window,
                None,
                &view,
                ScreenDescriptor {
                    size_in_pixels: [window_size.width, window_size.height],
                    pixels_per_point: self.window.scale_factor() as f32,
                },
                |ui| gui::builder::build(ui, &mut self.state, &mut self.ui_state),
            );
        }

        self.context.queue.submit(iter::once(encoder.finish()));

        frame.present();

        self.frame_counter.update();
        self.state.set_current_frame_time(self.frame_counter.frame_time());
    }

    fn create_msaa_view(&mut self, msaa_count: u32) {
        // let msaa_samples = 4;
        let msaa_texture = create_multisampled_framebuffer(
            &self.context.device,
            &self.surface.config(),
            msaa_count,
        );
        self.msaa_view = Some(msaa_texture);
    }

    
}
