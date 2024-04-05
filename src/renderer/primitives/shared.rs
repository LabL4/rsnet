use encase::ShaderType;
use nalgebra::Vector2;
use tracing::info;
use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, Buffer, Device, Queue,
};

use crate::renderer::{
    primitives::utils::{
        attach_buffer, component_primitives_vec_to_fragments,
        fragments_bind_group_layout_descriptor,
    },
    uniform_as_wgsl_bytes, StorageBufferData, UniformBufferData,
};

use super::ComponentTyPrimitives;

pub const MAX_FRAGMENTS: usize = 15;

#[derive(Debug, Clone, Copy, Default, ShaderType)]
pub struct CircleFragment {
    pub position: Vector2<f32>,
    pub radius: f32,
    pub color: u32,
}

#[derive(Debug, Clone, Copy, Default, ShaderType)]
pub struct LineFragment {
    pub start: Vector2<f32>,
    pub end: Vector2<f32>,
    pub thickness: f32,
    /// 0 mid, 1 start, 2 end, 3 start and end
    pub ty: u32,
    pub line_cap_ty: u32,
    pub color: u32,
}

#[derive(Debug, Clone, Copy, Default, ShaderType)]
pub struct RectangleFragment {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub color: u32,
}

#[derive(Debug, Clone, Copy, Default, ShaderType)]
pub struct TriangleFragment {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub dir_vec: Vector2<f32>,
    pub color: u32,
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
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl FragmentsStorage {
    pub fn bind_group_layout_descriptor<'a>() -> BindGroupLayoutDescriptor<'a> {
        BindGroupLayoutDescriptor {
            label: Some("Fragments storage bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    visibility: wgpu::ShaderStages::VERTEX,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    visibility: wgpu::ShaderStages::VERTEX,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    visibility: wgpu::ShaderStages::VERTEX,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    visibility: wgpu::ShaderStages::VERTEX,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    visibility: wgpu::ShaderStages::VERTEX,
                },
            ],
        }
    }

    pub fn create_bind_group(
        device: &Device,
        layout: &BindGroupLayout,
        component_ty_fragments_buffer: &wgpu::Buffer,
        circles_buffer: &wgpu::Buffer,
        lines_buffer: &wgpu::Buffer,
        rectangles_buffer: &wgpu::Buffer,
        triangles_buffer: &wgpu::Buffer,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Fragments storage bind group"),
            layout: layout,
            entries: &[
                wgpu::BindGroupEntry {
                    // Circles
                    binding: 0,
                    resource: component_ty_fragments_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    // Lines
                    binding: 1,
                    resource: circles_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    // Rectangles
                    binding: 2,
                    resource: lines_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    // Triangles
                    binding: 3,
                    resource: rectangles_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    // Triangles
                    binding: 4,
                    resource: triangles_buffer.as_entire_binding(),
                },
            ],
        })
    }

    pub fn attach_from_primitives(
        device: &Device,
        primitives: Vec<&ComponentTyPrimitives>,
    ) -> Self {
        let (component_ty_fragments, circles, lines, rectangles, triangles) =
            component_primitives_vec_to_fragments(primitives);

        let (component_ty_fragments_storage, component_ty_fragments_buffer) = attach_buffer(
            device,
            "component_ty_fragments storage buffer",
            component_ty_fragments,
        );

        let (circles_storage, circles_buffer) =
            attach_buffer(device, "circles storage buffer", circles);

        let (lines_storage, lines_buffer) = attach_buffer(device, "lines storage buffer", lines);

        let (rectangles_storage, rectangles_buffer) =
            attach_buffer(device, "rectangles storage buffer", rectangles);

        let (triangles_storage, triangles_buffer) =
            attach_buffer(device, "triangles storage buffer", triangles);

        let bind_group_layout =
            device.create_bind_group_layout(&fragments_bind_group_layout_descriptor());

        // info!(
        //     "Componentstype fragments: {:?}",
        //     component_ty_fragments_storage.get()
        // );
        // info!("Rectangles: {:?}", rectangles_storage.get());

        let bind_group = Self::create_bind_group(
            device,
            &bind_group_layout,
            &component_ty_fragments_buffer,
            &circles_buffer,
            &lines_buffer,
            &rectangles_buffer,
            &triangles_buffer,
        );

        FragmentsStorage {
            component_ty_fragments: component_ty_fragments_storage,
            circle_fragments: circles_storage,
            line_fragments: lines_storage,
            rectangle_fragments: rectangles_storage,
            triangles_fragments: triangles_storage,
            bind_group,
            bind_group_layout,
        }
    }

    pub fn add_primitives(&mut self, primitives: &ComponentTyPrimitives) {
        let (circle_fragments, line_fragments, rectangle_fragments, triangle_fragments) =
            primitives.to_fragments();

        let component_ty_fragments = ComponentTyFragments {
            circles_start: self.circle_fragments.get().len() as u32,
            n_circles: circle_fragments.len() as u32,
            lines_start: self.line_fragments.get().len() as u32,
            n_lines: line_fragments.len() as u32,
            rectangles_start: self.rectangle_fragments.get().len() as u32,
            n_rectangles: rectangle_fragments.len() as u32,
            triangles_start: self.triangles_fragments.get().len() as u32,
            n_triangles: triangle_fragments.len() as u32,
        };

        self.component_ty_fragments.get_mut().push(
            component_ty_fragments
        );

        self.circle_fragments.get_mut().extend(circle_fragments);
        self.line_fragments.get_mut().extend(line_fragments);
        self.rectangle_fragments.get_mut().extend(rectangle_fragments);
        self.triangles_fragments.get_mut().extend(triangle_fragments);

        
        //     lines_start: self.line_fragments.data.len() as u32,
        //     n_lines: lines.len() as u32,

        //     rectangles_start: self.rectangle_fragments.data.len() as u32,
        //     n_rectangles: rectangles.len() as u32,

        //     triangles_start: self.triangles_fragments.data.len() as u32,
        //     n_triangles: triangles.len() as u32,
        // };

        // self.component_ty_fragments
        //     .data
        //     .push(component_ty_fragments);

        // self.circle_fragments.data.extend(circles);
        // self.line_fragments.data.extend(lines);
        // self.rectangle_fragments.data.extend(rectangles);
        // self.triangles_fragments.data.extend(triangles);
    }

    pub fn write(&mut self, device: &Device, queue: &Queue) {
        let mut new_bg = false;

        let new_bg = vec![
            self.component_ty_fragments.write_buffer(device, queue),
            self.circle_fragments.write_buffer(device, queue),
            self.line_fragments.write_buffer(device, queue),
            self.rectangle_fragments.write_buffer(device, queue),
            self.triangles_fragments.write_buffer(device, queue)];

        if new_bg.iter().any(|v| *v) {
            self.bind_group = Self::create_bind_group(
                device,
                &self.bind_group_layout,
                self.component_ty_fragments.buffer().unwrap(),
                self.circle_fragments.buffer().unwrap(),
                self.line_fragments.buffer().unwrap(),
                self.triangles_fragments.buffer().unwrap(),
                self.rectangle_fragments.buffer().unwrap()
            );
        }

    }
}

#[derive(Debug, ShaderType)]
pub struct FragmentsData {
    pub fragments_idx: u32,
}

pub struct FragmentsDataUniform {
    pub buffer: UniformBufferData<FragmentsData>,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl FragmentsDataUniform {
    pub fn bind_group_layout_descriptor<'a>() -> BindGroupLayoutDescriptor<'a> {
        wgpu::BindGroupLayoutDescriptor {
            label: Some("Fragments data uniform bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                count: None,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                visibility: wgpu::ShaderStages::VERTEX,
            }],
        }
    }

    pub fn create_bind_group(
        device: &Device,
        layout: &BindGroupLayout,
        buffer: &Buffer,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Fragments data storage bind group"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                // Circles
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        })
    }

    pub fn attach(device: &Device) -> Self {
        let fragments_data = FragmentsData { fragments_idx: 0 };

        let fragments_data_encase_buffer = uniform_as_wgsl_bytes(&fragments_data).unwrap();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(format!("{} buffer", "Camera").as_str()),
            contents: &fragments_data_encase_buffer.as_ref(),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout =
            device.create_bind_group_layout(&Self::bind_group_layout_descriptor());

        let bind_group = Self::create_bind_group(device, &bind_group_layout, &buffer);

        Self {
            buffer: UniformBufferData {
                uniform: fragments_data,
                buffer,
                encase_buffer: fragments_data_encase_buffer,
            },
            bind_group,
            bind_group_layout,
        }
    }
}
