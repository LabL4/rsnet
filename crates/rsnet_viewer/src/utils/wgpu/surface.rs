use super::context::Context;

use crate::types::WindowSize;

use std::sync::Arc;
use tracing::info;
use wgpu::{Instance, Surface};
use winit::{
    event::{Event, StartCause},
    window::Window,
};

pub struct SurfaceWrapper<'a> {
    surface: Option<wgpu::Surface<'a>>,
    config: Option<wgpu::SurfaceConfiguration>,
}

impl<'a> SurfaceWrapper<'a> {
    pub fn new() -> Self {
        Self {
            surface: None,
            config: None,
        }
    }

    /// Called after the instance is created, but before we request an adapter.
    ///
    /// On wasm, we need to create the surface here, as the WebGL backend needs
    /// a surface (and hence a canvas) to be present to create the adapter.
    ///
    /// We cannot unconditionally create a surface here, as Android requires
    /// us to wait until we recieve the `Resumed` event to do so.
    pub fn pre_adapter(&mut self, instance: &Instance, window: Arc<Window>) {
        if cfg!(target_arch = "wasm32") {
            self.surface = Some(instance.create_surface(window).unwrap());
        }
    }

    /// Check if the event is the start condition for the surface.
    pub fn start_condition(e: &Event<()>) -> bool {
        match e {
            // On all other platforms, we can create the surface immediately.
            Event::NewEvents(StartCause::Init) => !cfg!(target_os = "android"),
            // On android we need to wait for a resumed event to create the surface.
            Event::Resumed => cfg!(target_os = "android"),
            _ => false,
        }
    }

    /// Called when an event which matches [`Self::start_condition`] is recieved.
    ///
    /// On all native platforms, this is where we create the surface.
    ///
    /// Additionally, we configure the surface based on the (now valid) window size.
    pub fn resume(&mut self, context: &Context, window: Arc<Window>, srgb: bool) {
        // Window size is only actually valid after we enter the event loop.
        let window_size = window.inner_size();
        let width = window_size.width.max(1);
        let height = window_size.height.max(1);

        info!("Surface resume {window_size:?}");

        // We didn't create the surface in pre_adapter, so we need to do so now.
        if !cfg!(target_arch = "wasm32") {
            self.surface = Some(context.instance.create_surface(window).unwrap());
        }

        // From here on, self.surface should be Some.
        let surface = self.surface.as_ref().unwrap();

        let mut config = surface
            .get_default_config(&context.adapter, width, height)
            .expect("Surface isn't supported by the adapter.");

        // This I dont really know
        config.present_mode = wgpu::PresentMode::Fifo;
        if srgb {
            // Not all platforms (WebGPU) support sRGB swapchains, so we need to use camera formats
            let view_format = config.format.add_srgb_suffix();
            config.view_formats.push(view_format);
        } else {
            // All platforms support non-sRGB swapchains, so we can just use the format directly.
            let format = config.format.remove_srgb_suffix();
            config.format = format;
            config.view_formats.push(format);
        }

        surface.configure(&context.device, &config);
        self.config = Some(config);
    }

    /// Resize the surface, making sure not to resize to zero
    pub fn resize(&mut self, context: &Context, size: WindowSize) {
        let config = self.config.as_mut().unwrap();
        config.width = size.width.max(1);
        config.height = size.height.max(1);
        let surface = self.surface.as_ref().unwrap();
        surface.configure(&context.device, config);
    }

    /// Acquire the next surface texture
    pub fn acquire(&mut self, context: &Context) -> wgpu::SurfaceTexture {
        // info!("Surface acquire!");
        let surface = self.surface.as_ref().unwrap();
        match surface.get_current_texture() {
            Ok(frame) => frame,
            Err(_) => {
                surface.configure(&context.device, self.config());
                surface
                    .get_current_texture()
                    .expect("Failed to acquire next surface texture!")
            }
        }
    }

    /// On suspend on android we drop the surface, as it will no longer be valid
    ///
    /// A suspend event is always followed by at least one resume event
    pub fn suspend(&mut self) {
        if cfg!(target_arch = "android") {
            self.surface = None
        }
    }

    pub fn get(&self) -> Option<&Surface> {
        self.surface.as_ref()
    }

    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        self.config.as_ref().unwrap()
    }
}
