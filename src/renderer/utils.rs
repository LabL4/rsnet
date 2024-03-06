use crate::scene::shared::SceneStorage;

use super::shared::*;

use std::{fmt::Debug, num::NonZeroU32};
use bytemuck::{Pod, Zeroable};
use encase::{internal::WriteInto, ShaderType, StorageBuffer, UniformBuffer};
use tracing::info;
use wgpu::{util::DeviceExt, Device};

const SHADER_ROOT: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/");

#[macro_export]
macro_rules! shader_path {
    ( $x:expr ) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/", $x)
    };
}

pub struct UniformBufferData<T: ShaderType + WriteInto> {
    pub uniform: T,
    pub encase_buffer: UniformBuffer<Vec<u8>>, // Hope this is a good idea
    pub buffer: wgpu::Buffer,
}

pub fn uniform_as_wgsl_bytes<'a, T: ShaderType + WriteInto + Debug>(
    value: &T,
) -> encase::internal::Result<UniformBuffer<Vec<u8>>> {
    // println!("as_wgsl_bytes: value: {:?}", value);
    let mut buffer = encase::UniformBuffer::new(Vec::new());
    buffer.write(value)?;
    Ok(buffer)
}

pub fn storage_as_wgsl_bytes<'a, T: ShaderType + WriteInto + Debug>(
    value: &T,
) -> encase::internal::Result<StorageBuffer<Vec<u8>>> {
    // println!("as_wgsl_bytes: value: {:?}", value);
    let mut buffer = encase::StorageBuffer::new(Vec::new());
    buffer.write(value)?;
    Ok(buffer)
}

pub struct StorageBufferData<T> {
    value: T,
    scratch: StorageBuffer<Vec<u8>>, // This is Bevys "scratch"
    buffer: Option<wgpu::Buffer>,
    len: Option<NonZeroU32>,
    label: Option<String>,
    changed: bool,
    buffer_usage: wgpu::BufferUsages,
    last_update: std::time::Instant
}

impl<T: ShaderType + WriteInto> StorageBufferData<T> {

    pub fn empty(value: T) -> Self {
        Self {
            value,
            scratch: StorageBuffer::new(Vec::new()),
            buffer: None,
            len: None,
            label: None,
            changed: false,
            buffer_usage: wgpu::BufferUsages::STORAGE,
            last_update: std::time::Instant::now()
        }
    }

    pub fn buffer(&self) -> Option<&wgpu::Buffer> {
        self.buffer.as_ref()
    }

    pub fn get_scratch(&self) -> &StorageBuffer<Vec<u8>> {
        &self.scratch
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
        // self.changed = true;
    }

    pub fn set_label(&mut self, label: Option<&str>) {
        let label = label.map(str::to_string);

        if label != self.label {
            self.changed = true;
        }

        self.label = label;
    }

    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn add_usages(&mut self, usage: wgpu::BufferUsages) {
        self.buffer_usage |= usage;
        self.changed = true;
    }

    pub fn write_buffer(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        // self.scratch.as_mut().clear();
        // self.scratch.set_offset(0);
        self.scratch.write(&self.value).unwrap();

        self.last_update = std::time::Instant::now();

        // println!("Scratch len: {}", self.scratch.as_ref().len());

        let capacity: u64 = self.buffer.as_ref().map(wgpu::Buffer::size).unwrap_or(0);
        let size = self.scratch.as_ref().len() as u64;

        if capacity < size || self.changed {
            info!("Creating new buffer, capacity: {}, size: {}", capacity, size);
            self.buffer = Some(
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    contents: &self.scratch.as_ref(),
                    label: Some("Storage Buffer"),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                }),
            );
            self.changed = false;
        } else if let Some(buffer) = &self.buffer {
            queue.write_buffer(buffer, 0, &self.scratch.as_ref());
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.changed
    }
}


// pub fn attach_uniform<T: ShaderType + WriteInto + Default + Debug>(
//     device: &Device,
//     uniform: Option<T>,
//     id: &str,
// ) -> UniformBufferData<T> {
//     let uniform = uniform.unwrap_or_default();
//     let encase_buffer = uniform_as_wgsl_bytes(&uniform).unwrap();
//     let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//         label: Some(format!("{} buffer", id).as_str()),
//         contents: &encase_buffer.as_ref(),
//         usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
//     });

//     let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//         label: Some(format!("{} bind group layout", id).as_str()),
//         entries: &[wgpu::BindGroupLayoutEntry {
//             binding: 0,
//             count: None,
//             ty: wgpu::BindingType::Buffer {
//                 ty: wgpu::BufferBindingType::Uniform,
//                 has_dynamic_offset: false,
//                 min_binding_size: None,
//             },
//             visibility: wgpu::ShaderStages::VERTEX,
//         }],
//     });

//     let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
//         label: Some(format!("{} bind group", id).as_str()),
//         layout: &bind_group_layout,
//         entries: &[wgpu::BindGroupEntry {
//             binding: 0,
//             resource: buffer.as_entire_binding(),
//         }],
//     });

//     UniformBufferData {
//         uniform,
//         encase_buffer,
//         buffer,
//         bind_group,
//         bind_group_layout,
//     }
// }

// trait UniformTrait: 

pub fn common_uniforms_layout(device: &Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(format!("{} bind group layout", "uniforms").as_str()),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                count: None,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                count: None,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                count: None,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            },
        ],
    })
}

pub fn attach_common_uniforms(device: &Device, camera_uniform: CameraUniform, mouse_uniform: MouseUniform, window_uniform: WindowUniform) -> CommonUniforms {
    let camera_encase_buffer = uniform_as_wgsl_bytes(&camera_uniform).unwrap();
    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(format!("{} buffer", "Camera").as_str()),
        contents: &camera_encase_buffer.as_ref(),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let mouse_encase_buffer = uniform_as_wgsl_bytes(&mouse_uniform).unwrap();
    let mouse_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(format!("{} buffer", "Mouse").as_str()),
        contents: &mouse_encase_buffer.as_ref(),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let window_encase_buffer = uniform_as_wgsl_bytes(&window_uniform).unwrap();
    let window_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(format!("{} buffer", "Window").as_str()),
        contents: &window_encase_buffer.as_ref(),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let bind_group_layout = common_uniforms_layout(device);

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(format!("{} bind group", "uniforms").as_str()),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: mouse_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: window_buffer.as_entire_binding(),
            },
        ],
    });

    CommonUniforms {
        mouse: UniformBufferData {
            uniform: mouse_uniform,
            encase_buffer: mouse_encase_buffer,
            buffer: mouse_buffer,
        },
        camera: UniformBufferData {
            uniform: camera_uniform,
            encase_buffer: camera_encase_buffer,
            buffer: camera_buffer,
        },
        window: UniformBufferData {
            uniform: window_uniform,
            encase_buffer: window_encase_buffer,
            buffer: window_buffer,
        },
        bind_group,
        bind_group_layout,
    }

}

pub fn attach_empty_scene_storage(device: &Device) -> SceneStorage {
    
    let mut components = StorageBufferData::empty(Vec::new());
    components.set_label(Some("Components storage buffer"));
    components.add_usages(wgpu::BufferUsages::STORAGE);
    components.add_usages(wgpu::BufferUsages::COPY_DST);


    let components_encase_buffer = storage_as_wgsl_bytes(&components.value).unwrap();
    let components_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(format!("{} storage buffer", "Scene").as_str()),
        contents: &components_encase_buffer.as_ref(),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Scene storage bind group layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            count: None,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            visibility: wgpu::ShaderStages::VERTEX,
        }],
    });

    let bind_group = create_scene_storage_bind_group(device, &bind_group_layout, &components_buffer);


    SceneStorage {
        components,
        chunks: None,
        bind_group,
        bind_group_layout,
    }
}

pub fn create_scene_storage_bind_group(device: &Device, layout: &wgpu::BindGroupLayout, components_buffer: &wgpu::Buffer) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Scene storage bind group"),
        layout: layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: components_buffer.as_entire_binding(),
        }],
    })
}