use nalgebra::Vector2;
use encase::ShaderType;

use crate::renderer::StorageBufferData;

const MAX_FRAGMENTS: usize = 10;

#[derive(Debug, Clone, Copy, Default, ShaderType)]
pub struct CircleFragment {
    pub position: Vector2<f32>,
    pub radius: f32,
    pub color: u32
}

#[derive(Debug, Clone, Copy, Default, ShaderType)]
pub struct LineFragment {
    pub start: Vector2<f32>,
    pub end: Vector2<f32>,
    pub thickness: f32,
    /// 0 mid, 1 start, 2 end, 3 start and end
    pub ty: u32,
    pub color: u32
}

#[derive(Debug, Clone, Copy, Default, ShaderType)]
pub struct RectangleFragment {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub color: u32
}

#[derive(Debug, ShaderType)]
pub struct Fragments {
    pub circles: [CircleFragment; MAX_FRAGMENTS],
    pub n_circles: u32,
    
    pub lines: [LineFragment; MAX_FRAGMENTS],
    pub n_lines: u32,

    pub rectangles: [RectangleFragment; MAX_FRAGMENTS],
    pub n_rectangles: u32
}

pub struct FragmentsStorage {
    pub fragments: StorageBufferData<Vec<Fragments>>,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout
}