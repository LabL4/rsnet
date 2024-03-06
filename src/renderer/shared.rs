use crate::app::camera::Camera;
use super::utils::UniformBufferData;

use encase::ShaderType;
use nalgebra::{Vector2, Matrix4};

#[derive(Debug, ShaderType, Default)]
pub struct MouseUniform {
    /// Screen space position of the mouse [-1, 1]
    pub pos: Vector2<f32>
}

#[derive(Debug, Copy, Clone, ShaderType, Default)]
pub struct AaBb {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
}

#[derive(Debug, Copy, Clone, ShaderType, Default)]
pub struct CameraUniform {
    view_proj: Matrix4<f32>,
    aabb: AaBb,
    radius: f32,
    // _padding: [f32; 3],
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
    pub bind_group_layout: wgpu::BindGroupLayout
}

