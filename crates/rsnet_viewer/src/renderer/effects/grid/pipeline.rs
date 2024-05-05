use crate::renderer::{chunk_data_layout, common_uniforms_layout, time_data_layout};

use rsnet_derive::include_shader;
use wgpu::{Device, SurfaceConfiguration};

pub fn create_pipeline(
    config: &SurfaceConfiguration,
    device: &Device,
    msaa_count: u32,
) -> wgpu::RenderPipeline {
    let common_uniforms_layout = common_uniforms_layout(device);
    let time_data_layout = time_data_layout(device);
    let chunk_data_layout = chunk_data_layout(device);

    let bind_group_layouts = [
        &common_uniforms_layout,
        &time_data_layout,
        &chunk_data_layout,
    ];

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Main shader"),
        source: wgpu::ShaderSource::Wgsl(include_shader!("grid/grid.wgsl").into()),
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Primitive render pipeline layout"),
        bind_group_layouts: &bind_group_layouts,
        push_constant_ranges: &[],
    });

    let vertex_layouts = &[];
    let color_format = config.format;
    // let depth_format = None;

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Grid effect render pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: vertex_layouts,
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
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: msaa_count,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}
