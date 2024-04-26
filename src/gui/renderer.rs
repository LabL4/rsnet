use egui::Context;
use egui_wgpu::Renderer;
use egui_wgpu::ScreenDescriptor;
use egui_winit::{EventResponse, State};
use wgpu::{CommandEncoder, Device, Queue, TextureFormat, TextureView};
use winit::{event::WindowEvent, window::Window};

pub struct GuiRenderer {
    // egui context
    state: State,
    // egui_winit state
    renderer: Renderer, // egui_wgpu renderer
}

const MSAA_SAMPLES: usize = 1;

impl GuiRenderer {
    pub fn new(
        device: &Device,
        output_color_format: TextureFormat,
        output_depth_format: Option<TextureFormat>,
        window: &Window,
    ) -> Self {
        let egui_context = Context::default();
        let vid = egui_context.viewport_id();
        let egui_state = State::new(
            egui_context,
            vid,
            window,
            Some(window.scale_factor() as f32),
            None,
        );

        let egui_renderer = Renderer::new(
            device,
            output_color_format,
            output_depth_format,
            MSAA_SAMPLES as u32,
        );

        Self {
            state: egui_state,
            renderer: egui_renderer,
        }
    }

    pub fn handle_input(&mut self, window: &Window, event: &WindowEvent) -> EventResponse {
        self.state.on_window_event(window, event)
    }

    pub fn draw(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        window: &Window,
        window_msaa_view: Option<&TextureView>,
        window_surface_view: &TextureView,
        screen_descriptor: ScreenDescriptor,
        run_ui: impl FnOnce(&Context),
    ) {
        let raw_input = self.state.take_egui_input(window);
        let context = self.state.egui_ctx();
        let full_output = context.run(raw_input, |_ui| {
            run_ui(&context);
        });

        self.state
            .handle_platform_output(window, full_output.platform_output);

        let context = self.state.egui_ctx();
        let tris = context.tessellate(full_output.shapes, window.scale_factor() as f32);
        for (id, image_delta) in full_output.textures_delta.set {
            self.renderer
                .update_texture(&device, &queue, id, &image_delta);
        }

        self.renderer
            .update_buffers(&device, &queue, encoder, &tris, &screen_descriptor);
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: if window_msaa_view.is_some() {
                    window_msaa_view.unwrap()
                } else {
                    window_surface_view
                },
                resolve_target: if window_msaa_view.is_some() {
                    Some(window_surface_view)
                } else {
                    None
                },
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            label: Some("egui main render pass"),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        self.renderer.render(&mut rpass, &tris, &screen_descriptor);
        drop(rpass);
        for x in &full_output.textures_delta.free {
            self.renderer.free_texture(x);
        }
    }
}
