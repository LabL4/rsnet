pub mod event_loop;
pub mod camera;

use camera::CameraController;

use crate::{gui::{self, renderer::GuiRenderer}, renderer::Renderer, scene, utils::wgpu::{context::Context, surface::SurfaceWrapper}};

use std::{sync::Arc, iter};
use winit::{event::WindowEvent, window::Window};
use wgpu::{TextureViewDescriptor, CommandEncoderDescriptor};
use egui_wgpu::ScreenDescriptor;

use self::camera::camera_controller;

#[derive(Default)]
pub struct State {
    pub ui: gui::state::State,
    pub scene: scene::Scene,
}

pub struct App<'a> {
    pub gui_renderer: Option<GuiRenderer>,
    pub scene_renderer: Option<Renderer<'a>>, 

    pub surface: SurfaceWrapper<'a>,
    pub context: Context,
    window: Arc<Window>,

    camera_controller: CameraController,

    pub state: State,
}

impl<'a>App<'a> {
    pub fn new(gui_renderer: Option<GuiRenderer>, scene_renderer: Option<Renderer<'a>>, surface: SurfaceWrapper<'a>, context: Context, window: Arc<Window>) -> Self {

        let camera_controller = CameraController::new(window.inner_size());

        Self { gui_renderer, scene_renderer, surface, context, window, state: State::default(), camera_controller }
    }

    pub fn window_event_handler(&mut self, event: winit::event::WindowEvent) {
        if self.gui_renderer.is_some()
        && self.gui_renderer.as_mut().unwrap().handle_input(&self.window, &event).consumed {}

        self.camera_controller.event_handler(event);
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.camera_controller.resize(size);
    }

    pub fn render(&mut self) {

        let frame = self.surface.acquire(&self.context);

        let view = frame.texture.create_view(&TextureViewDescriptor {
            format: Some(self.surface.config().view_formats[0]),
            ..TextureViewDescriptor::default()
        });

        let mut encoder = self.context.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let window_size = self.window.inner_size();


        if let Some(scene_renderer) = &mut self.scene_renderer {
            scene_renderer.render(&view, &self.context, &mut encoder, &mut self.camera_controller, &self.state.scene);
        }

        if let Some(gui_renderer) = &mut self.gui_renderer {
            gui_renderer.draw(&self.context.device, &self.context.queue, &mut encoder, &self.window, &view,
                ScreenDescriptor {
                size_in_pixels: [window_size.width, window_size.height],
                pixels_per_point: self.window.scale_factor() as f32,
                },
                |ui| gui::builder::build(ui, &mut self.state)
            );
        }

        self.context.queue.submit(iter::once(encoder.finish()));

        frame.present();
    }
}