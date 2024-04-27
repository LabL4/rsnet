pub mod camera;
pub mod event_loop;
pub mod state;
pub mod utils;

use camera::CameraController;
use smaa::SmaaTarget;

pub use self::state::State;
use self::utils::create_multisampled_framebuffer;
use crate::{
    gui::{self, renderer::GuiRenderer},
    renderer::Renderer,
    scene,
    utils::{
        frame_counter,
        wgpu::{context::Context, surface::SurfaceWrapper},
        FrameCounter,
    },
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
    smaa_target: Option<SmaaTarget>,
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
            frame_counter,
            smaa_target: None,
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
            self.create_msaa_view();
            self.create_smaa_target();
        }
        self.camera_controller.resize(size);
    }

    fn render(&mut self) {
        if self.state.rebuild_bundles() {
            if self.state.msaa_count() != 1 {
                self.create_msaa_view();
            } else {
                self.msaa_view = None;
            }
            if let Some(scene_renderer) = &mut self.scene_renderer {
                scene_renderer.set_msaa_count(self.state.msaa_count());
                scene_renderer.rebuild_pipelines(self.surface.config(), &self.context.device);
            }
            self.state.set_rebuild_bundles(false);
        }

        if self.state.rebuild_smaa() {
            self.create_smaa_target();
            self.state.set_rebuild_smaa(false);
        }

        let frame = self.surface.acquire(&self.context);

        let view = frame.texture.create_view(&TextureViewDescriptor {
            format: Some(self.surface.config().view_formats[0]),
            ..TextureViewDescriptor::default()
        });

        let mut smaa_frame = None;
        if let Some(smaa_target) = &mut self.smaa_target {
            smaa_frame =
                Some(smaa_target.start_frame(&self.context.device, &self.context.queue, &view));
        }

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
                smaa_frame.as_deref().unwrap_or(&view),
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
                smaa_frame.as_deref().unwrap_or(&view),
                ScreenDescriptor {
                    size_in_pixels: [window_size.width, window_size.height],
                    pixels_per_point: self.window.scale_factor() as f32,
                },
                |ui| gui::builder::build(ui, &mut self.state, &mut self.ui_state),
            );
        }

        self.context.queue.submit(iter::once(encoder.finish()));

        if let Some(smaa_frame) = smaa_frame {
            smaa_frame.resolve();
        }

        frame.present();

    }

    fn update_state(&mut self) {
        self.frame_counter.update();
        self.state
            .set_current_frame_time(self.frame_counter.frame_time());
        self.state.set_n_primitives_in_fragment_storage(
            self.scene_renderer
                .as_ref()
                .unwrap()
                .cache
                .compty_fragments_index_map
                .len(),
        );
        self.state.set_n_wires_in_buffer(
            self.scene_renderer
                .as_ref()
                .unwrap()
                .shared
                .scene_storage
                .wires
                .get()
                .len(),
        );
        self.state.set_n_components_in_buffer(
            self.scene_renderer
                .as_ref()
                .unwrap()
                .shared
                .scene_storage
                .components
                .get()
                .len(),
        );
        self.state.set_screen_chunk_range(
            self.scene_renderer
                .as_ref()
                .unwrap()
                .cache
                .chunk_range.clone().unwrap()
        );
        self.state.set_chunk_step_idx(
            self.camera_controller.chunk_step_idx
        );
        self.state.set_chunk_size(
            self.camera_controller.chunk_size
        );
    }

    pub fn present(&mut self) {
        self.render();
        self.update_state();
    }

    fn create_msaa_view(&mut self) {
        let msaa_count = self.state.msaa_count();
        // let msaa_samples = 4;
        let msaa_texture = create_multisampled_framebuffer(
            &self.context.device,
            &self.surface.config(),
            msaa_count,
        );
        self.msaa_view = Some(msaa_texture);
    }

    pub fn create_smaa_target(&mut self) {
        let smaa_target = SmaaTarget::new(
            &self.context.device,
            &self.context.queue,
            self.surface.config().width,
            self.surface.config().height,
            self.surface.config().view_formats[0],
            self.state.smaa_mode(),
        );
        self.smaa_target = Some(smaa_target);
    }
}
