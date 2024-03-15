use crate::shader_path;

use bytemuck::{Pod, Zeroable};
use wgpu::{util::DeviceExt, Device, SurfaceConfiguration};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    // pub component_idx: u32,
    pub fragments_idx: u32,
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
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
    pub value: Vec<Vertex>,
    pub label: Option<String>,
    // pub scratch: Option<Vec<Vertex>>,
    pub buffer: Option<wgpu::Buffer>,
    pub buffer_layout: wgpu::VertexBufferLayout<'a>
}

impl VertexBuffer<'_> {

    pub fn get(&self) -> &Vec<Vertex> {
        &self.value
    }

    pub fn set(&mut self, value: Vec<Vertex>) {
        self.value = value;
    }

    pub fn set_label(&mut self, label: Option<String>) {
        self.label = label;
    }

    pub fn buffer(&self) -> Option<&wgpu::Buffer> {
        self.buffer.as_ref()
    }

    pub fn buffer_layout(&self) -> &wgpu::VertexBufferLayout {
        &self.buffer_layout
    }

    pub fn write(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {

        let capacity: u64 = self.buffer.as_ref().map(wgpu::Buffer::size).unwrap_or(0);
        let byte_data = bytemuck::cast_slice(self.value.as_slice());

        if capacity < byte_data.len() as u64 {
            self.buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: self.label.as_deref(),
                contents: byte_data,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }));
        } else {
            queue.write_buffer(self.buffer.as_ref().unwrap(), 0, byte_data);
        }

    }
}

// pub fn attach_vertex_buffer<'a>(device: &Device, mut vertex_data: Option<Vec<Vertex>>) -> VertexBuffer<'a> {

//     if vertex_data.is_none() {
//         vertex_data = Some(vec![Vertex {
//             // component_idx: 0,
//             fragments_idx: 0
//         }]);
//     }

//     let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//         label: Some("Vertex buffer"),
//         contents: bytemuck::cast_slice(vertex_data.unwrap().as_slice()),
//         usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
//     });

//     let buffer_layout = Vertex::desc();

//     VertexBuffer {
//         buffer,
//         buffer_layout
//     }
// }

pub fn create_primitive_pipeline(config: &SurfaceConfiguration,
    device: &Device,
    msaa_samples: u32,
    common_uniform_bind_group_layout: &wgpu::BindGroupLayout,
    fragments_storage_bind_group_layout: &wgpu::BindGroupLayout,
    scene_storage_bind_group_layout: &wgpu::BindGroupLayout,
    fragments_data_uniform_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    create_primitive_pipeline_unnamed(config, device, msaa_samples, &[
        common_uniform_bind_group_layout,
        fragments_storage_bind_group_layout,
        scene_storage_bind_group_layout,
        fragments_data_uniform_bind_group_layout,
    ])
}

pub fn create_primitive_pipeline_unnamed(
    config: &SurfaceConfiguration,
    device: &Device,
    msaa_samples: u32,
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
                blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Cw,
            // cull_mode: Some(wgpu::Face::Back),
            cull_mode: None,
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
            count: msaa_samples,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}