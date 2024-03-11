use nalgebra::Vector2;
use encase::ShaderType;

use crate::renderer::{StorageBufferData, UniformBufferData};

pub const MAX_FRAGMENTS: usize = 15;

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

#[derive(Debug, Clone, Copy, Default, ShaderType)]
pub struct TriangleFragment {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub dir_vec: Vector2<f32>,
    pub color: u32
}

#[derive(Debug, ShaderType)]
pub struct ComponentTyFragments {
    pub circles_start: u32,
    pub n_circles: u32,

    pub lines_start: u32,
    pub n_lines: u32,

    pub rectangles_start: u32,
    pub n_rectangles: u32,

    pub triangles_start: u32,
    pub n_triangles: u32,
}

impl ComponentTyFragments {
    pub fn n_fragments(&self) -> u32 {
        self.n_circles + self.n_lines + self.n_rectangles + self.n_triangles
    }
}

pub struct FragmentsStorage {

    pub component_ty_fragments: StorageBufferData<Vec<ComponentTyFragments>>,

    pub circle_fragments: StorageBufferData<Vec<CircleFragment>>,
    pub line_fragments: StorageBufferData<Vec<LineFragment>>,
    pub rectangle_fragments: StorageBufferData<Vec<RectangleFragment>>,
    pub triangles_fragments: StorageBufferData<Vec<TriangleFragment>>,

    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout
}


#[derive(Debug, ShaderType)]
pub struct FragmentsData {
    pub fragments_idx: u32
}

pub struct FragmentsDataUniform {
    pub fragments_data: UniformBufferData<FragmentsData>,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout
}