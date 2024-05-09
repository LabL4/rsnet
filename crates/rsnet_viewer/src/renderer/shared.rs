use super::shared;
use super::utils;

use crate::app::camera::Camera;
use utils::{uniform_as_wgsl_bytes, UniformBufferData};

use encase::ShaderType;
use nalgebra::{Matrix4, Vector2};
use wgpu::{util::DeviceExt, Device};

#[derive(Debug, ShaderType, Default)]
pub struct MouseUniform {
    /// Screen space position of the mouse [-1, 1]
    pub pos: Vector2<f32>,
}

#[derive(Debug, Copy, Clone, ShaderType, Default)]
pub struct AaBb {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
}

#[derive(Debug, Copy, Clone, ShaderType)]
pub struct CameraUniform {
    view_proj: Matrix4<f32>,
    aabb: AaBb,
    radius: f32,
    // _padding: [f32; 3],
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraUniform {
    pub fn new() -> Self {
        // 4x4 identity matrix
        Self {
            view_proj: Matrix4::identity(),
            aabb: AaBb::default(),
            radius: 0.0,
            // _padding: [0.0; 3],
        }
    }

    pub fn update_view_projection(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_proj().into();
        self.radius = camera.get_view_matrix()[(2, 3)].abs();
        self.aabb = AaBb {
            min: camera.aabb().min.into(),
            max: camera.aabb().max.into(),
        };
    }
}

#[derive(Debug, ShaderType, Default)]
pub struct WindowUniform {
    pub size: Vector2<u32>,
    aspect: f32,
}

impl WindowUniform {
    pub fn new(size: Vector2<u32>) -> Self {
        Self {
            size,
            aspect: size.x as f32 / size.y as f32,
        }
    }

    pub fn compute_aspect(&mut self) {
        self.aspect = self.size.x as f32 / self.size.y as f32;
    }
}

pub struct CommonUniforms {
    pub mouse: UniformBufferData<MouseUniform>,
    pub camera: UniformBufferData<CameraUniform>,
    pub window: UniformBufferData<WindowUniform>,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

macro_rules! common_uniforms_bind_group_layout_descriptor {
    () => {
        wgpu::BindGroupLayoutDescriptor {
            label: Some("Common uniforms bind group layout"),
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
        }
    };
}

impl CommonUniforms {
    pub fn attach(device: &Device) -> Self {
        let camera_uniform = CameraUniform::default();
        let mouse_uniform = MouseUniform::default();
        let window_uniform = WindowUniform::default();

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

        let bind_group_layout =
            device.create_bind_group_layout(&common_uniforms_bind_group_layout_descriptor!());

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

        Self {
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
}
