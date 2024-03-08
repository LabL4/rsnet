use super::surface::SurfaceWrapper;

use std::sync::Arc;
use tracing::info;
use winit::window::Window;

pub struct Context {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl Context {
    pub async fn init<'a>(surface: &mut SurfaceWrapper<'a>, window: Arc<Window>) -> Self {
        info!("Initializing wgpu...");

        // let backends = wgpu::Backends::GL;
        let backends = wgpu::util::backend_bits_from_env().unwrap_or(wgpu::Backends::all());
        let dx12_shader_compiler = wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default();
        let gles_minor_version = wgpu::util::gles_minor_version_from_env().unwrap_or_default();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            flags: wgpu::InstanceFlags::from_build_config().with_env(),// | wgpu::InstanceFlags::debugging(),
            dx12_shader_compiler,
            gles_minor_version,
        });

        // let instance = wgpu::Instance::new(wgpu::BackendBit::GL);

        surface.pre_adapter(&instance, window);

        let adapter = wgpu::util::initialize_adapter_from_env_or_default(&instance, surface.get())
            .await
            .expect("No suitable GPU adapters found in the system");

        let adapter_info = adapter.get_info();
        info!("Using {} ({:?})", adapter_info.name, adapter_info.backend);

        let adapter_features = adapter.features();
        let _downlevel_capabilities = adapter.get_downlevel_capabilities();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: adapter_features,
                    required_limits: {
                        if cfg!(target_arch = "wasm32") {
                            wgpu::Limits::downlevel_webgl2_defaults()
                        } else {
                            wgpu::Limits::default()
                        }
                    },
                },
                None,
            )
            .await
            .expect("Unable to find suitable GPU adapter!");

        Self {
            instance,
            adapter,
            device,
            queue,
        }
    }
}