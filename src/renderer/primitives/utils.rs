use super::super::utils::{storage_as_wgsl_bytes, StorageBufferData};
use super::{shared::*, ComponentTyPrimitives};

use crate::renderer::{uniform_as_wgsl_bytes, UniformBufferData};

use encase::internal::WriteInto;
use encase::ShaderType;
use tracing::info;
use wgpu::util::DeviceExt;
use wgpu::{BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, Buffer, Device};

pub fn component_primitives_vec_to_fragments(
    primitives: Vec<&ComponentTyPrimitives>,
) -> (
    Vec<ComponentTyFragments>,
    Vec<CircleFragment>,
    Vec<LineFragment>,
    Vec<RectangleFragment>,
    Vec<TriangleFragment>,
) {
    let mut component_ty_fragments: Vec<ComponentTyFragments> = Vec::new();
    let mut circles: Vec<CircleFragment> = Vec::new();
    let mut lines: Vec<LineFragment> = Vec::new();
    let mut rectangles: Vec<RectangleFragment> = Vec::new();
    let mut triangles: Vec<TriangleFragment> = Vec::new();

    for primitive in primitives {
        let (circles_, lines_, rectangles_, triangles_) = primitive.to_fragments();

        component_ty_fragments.push(ComponentTyFragments {
            circles_start: circles.len() as u32,
            n_circles: circles_.len() as u32,

            lines_start: lines.len() as u32,
            n_lines: lines_.len() as u32,

            rectangles_start: rectangles.len() as u32,
            n_rectangles: rectangles_.len() as u32,

            triangles_start: triangles.len() as u32,
            n_triangles: triangles_.len() as u32,
        });

        circles.extend(circles_);
        lines.extend(lines_);
        rectangles.extend(rectangles_);
        triangles.extend(triangles_);
    }

    (
        component_ty_fragments,
        circles,
        lines,
        rectangles,
        triangles,
    )
}

pub fn attach_buffer<T: ShaderType + WriteInto>(
    device: &Device,
    label: &str,
    data: T,
) -> (StorageBufferData<T>, Buffer) {
    let mut storage = StorageBufferData::empty(data);
    storage.set_label(Some(label));
    storage.add_usages(wgpu::BufferUsages::STORAGE);
    storage.add_usages(wgpu::BufferUsages::COPY_DST);

    let storage_encase_buffer = storage_as_wgsl_bytes(&storage.get()).unwrap();
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents: &storage_encase_buffer.as_ref(),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });
    (storage, buffer)
}

pub fn attach_fragment_storage(
    device: &Device,
    primitives: Vec<&ComponentTyPrimitives>,
) -> FragmentsStorage {
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

    info!("Componentstype fragments: {:?}", component_ty_fragments_storage.get());
    info!("Rectangles: {:?}", rectangles_storage.get());

    let bind_group = create_fragment_storage_bind_group(
        device,
        &bind_group_layout,
        &[
            &component_ty_fragments_buffer,
            &circles_buffer,
            &lines_buffer,
            &rectangles_buffer,
            &triangles_buffer,
        ],
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

pub fn fragments_bind_group_layout_descriptor<'a>() -> BindGroupLayoutDescriptor<'a> {
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

pub fn attach_fragment_data_uniform(device: &Device) -> FragmentsDataUniform {
    let fragments_data = FragmentsData { fragments_idx: 0 };

    let fragments_data_encase_buffer = uniform_as_wgsl_bytes(&fragments_data).unwrap();
    let fragments_data_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(format!("{} buffer", "Camera").as_str()),
        contents: &fragments_data_encase_buffer.as_ref(),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Fragments data storage bind group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            // Circles
            binding: 0,
            resource: fragments_data_buffer.as_entire_binding(),
        }],
    });

    FragmentsDataUniform {
        buffer: UniformBufferData {
            uniform: fragments_data,
            buffer: fragments_data_buffer,
            encase_buffer: fragments_data_encase_buffer,
        },
        bind_group,
        bind_group_layout,
    }
}

pub fn create_fragment_storage_bind_group(
    device: &Device,
    layout: &BindGroupLayout,
    buffers: &[&Buffer; 5],
) -> BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Fragments storage bind group"),
        layout: layout,
        entries: &[
            wgpu::BindGroupEntry {
                // Circles
                binding: 0,
                resource: buffers[0].as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                // Lines
                binding: 1,
                resource: buffers[1].as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                // Rectangles
                binding: 2,
                resource: buffers[2].as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                // Triangles
                binding: 3,
                resource: buffers[3].as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                // Triangles
                binding: 4,
                resource: buffers[4].as_entire_binding(),
            },
        ],
    })
}
