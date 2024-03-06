use crate::shader_path;

use bytemuck::{Pod, Zeroable};
use wgpu::{util::DeviceExt, Device, SurfaceConfiguration};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    // pub component_idx: u32,
    pub fragment_idx: u32,
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Uint32,
                },
                // wgpu::VertexAttribute {
                //     offset: std::mem::size_of::<u32>() as wgpu::BufferAddress,
                //     shader_location: 1,
                //     format: wgpu::VertexFormat::Uint32,
                // }
            ]
        }
    }
}

pub struct VertexBuffer<'a> {
    pub buffer: wgpu::Buffer,
    pub buffer_layout: wgpu::VertexBufferLayout<'a>
}

pub fn attach_vertex_buffer<'a>(device: &Device, mut vertex_data: Option<Vec<Vertex>>) -> VertexBuffer<'a> {

    if vertex_data.is_none() {
        vertex_data = Some(vec![Vertex {
            // component_idx: 0,
            fragment_idx: 0
        }]);
    }

    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex buffer"),
        contents: bytemuck::cast_slice(vertex_data.unwrap().as_slice()),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let buffer_layout = Vertex::desc();

    VertexBuffer {
        buffer,
        buffer_layout
    }
}

pub fn create_primitive_pipeline(
    config: &SurfaceConfiguration,
    device: &Device,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Main shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!(shader_path!("primitives/primitive.wgsl")).into()),
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Primitive render pipeline layout"),
        bind_group_layouts: &bind_group_layouts,
        push_constant_ranges: &[],
    });

    // let vertex_layouts = &[ Vertex::desc() ];
    let vertex_layouts = &[ ];
    let color_format = config.format;
    let depth_format = None;

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: vertex_layouts
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleStrip,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Cw,
            cull_mode: Some(wgpu::Face::Back),
            // cull_mode: None,
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: depth_format.map(|format| wgpu::DepthStencilState {
            format,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}