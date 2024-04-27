use super::App;

use crate::gui::renderer::GuiRenderer;
use crate::renderer::Renderer;
use crate::utils::frame_counter::FrameCounter;
use crate::utils::wgpu::{Context, SurfaceWrapper};

use std::sync::Arc;
use tracing_subscriber::{filter::EnvFilter, fmt, prelude::*};
use winit::dpi::LogicalSize;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopWindowTarget};
use winit::keyboard::{Key, NamedKey};
use winit::{event_loop::EventLoop, window::Window};

pub struct EventLoopWrapper {
    pub event_loop: EventLoop<()>,
    pub window: Arc<Window>,
}

impl EventLoopWrapper {
    pub fn new(title: &str) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let mut builder = winit::window::WindowBuilder::new();
        builder = builder.with_title(title);
        builder = builder.with_inner_size(LogicalSize::new(900.0, 700.0));
        let window = Arc::new(builder.build(&event_loop).unwrap());

        #[cfg(arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            let canvas = window.canvas().expect("Could not get canvas");
            canvas.style().set_css_text("height: 100%; width: 100%");

            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| body.append_child(&canvas).ok())
                .expect("couldn't append canvas to document body");
        }

        Self { event_loop, window }
    }

    pub fn get_window(&self) -> Arc<Window> {
        self.window.clone()
    }
}

pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            tracing_subscriber::registry()
                .with(fmt::layer())
                .with(EnvFilter::from_default_env())
                .init();
        }
    }

    let window_loop = EventLoopWrapper::new("rsnet");
    // let initial_w_size = window_loop.get_window().inner_size();
    let mut surface = SurfaceWrapper::new();
    let context = Context::init(&mut surface, window_loop.window.clone()).await;

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            use winit::platform::web::EventLoopExtWebSys;
            let event_loop_function = EventLoop::spawn;
        } else {
            let event_loop_function = EventLoop::run;
        }
    }

    let _ = (event_loop_function)(
        window_loop.event_loop,
        event_handler(window_loop.window, surface, context),
    );
}

fn event_handler<'a>(
    window: Arc<Window>,
    surface: SurfaceWrapper<'a>,
    context: Context,
) -> impl FnMut(Event<()>, &EventLoopWindowTarget<()>) -> () + 'a {
    // let mut frame_counter = FrameCounter::new();

    let mut app = App::new(None, None, surface, context, window.clone());

    move |event: Event<()>, target: &EventLoopWindowTarget<()>| {
        target.set_control_flow(ControlFlow::Poll);

        match event {
            ref e if SurfaceWrapper::start_condition(e) => {
                app.surface.resume(&app.context, window.clone(), true);

                let scene_renderer = Renderer::new(&app.surface.config(), &app.context.device);

                let gui_renderer = GuiRenderer::new(
                    &app.context.device,
                    app.surface.config().view_formats[0],
                    None,
                    &app.window,
                );

                app.scene_renderer = Some(scene_renderer);
                app.gui_renderer = Some(gui_renderer);
            }
            Event::Suspended => {
                app.surface.suspend();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    app.surface.resize(&app.context, size);
                    app.resize(size);
                    app.window.request_redraw();
                }
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: Key::Named(NamedKey::Escape),
                            ..
                        },
                    ..
                }
                | WindowEvent::CloseRequested => {
                    target.exit();
                }
                #[cfg(not(target_arch = "wasm32"))]
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: Key::Character(s),
                            ..
                        },
                    ..
                } if s == "r" => {
                    println!("{:#?}", app.context.instance.generate_report());
                }
                WindowEvent::RedrawRequested => {
                    // frame_counter.update();

                    app.present();

                    app.window.request_redraw();
                }
                _ => app.window_event_handler(event),
            },
            _ => (),
        }
    }
}
