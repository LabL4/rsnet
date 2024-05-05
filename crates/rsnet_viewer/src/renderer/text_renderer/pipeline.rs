use bytemuck::{Pod, Zeroable};
use rsnet_derive::include_shader;
use wgpu::{util::DeviceExt, BlendComponent, Device, SurfaceConfiguration};

use crate::renderer::shader;

use super::shared::Vertex;

pub enum TextPipelineType {
    Offscreen,
    Onscreen
}

pub fn create_offscreen_pipeline(
    config: &SurfaceConfiguration,
    device: &Device,
    msaa_count: u32,
    common_uniforms_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader_str = include_shader!("text/text_offscreen.wgsl");
    create_pipeline(
        config,
        device,
        msaa_count,
        "Offscreen text pipeline",
        &[common_uniforms_bind_group_layout],
        "vs_offscreen",
        "fs_offscreen",
        shader_str,
        TextPipelineType::Offscreen
    )
}

pub fn create_onscreen_pipeline(
    config: &SurfaceConfiguration,
    device: &Device,
    msaa_count: u32,
    common_uniforms_bind_group_layout: &wgpu::BindGroupLayout,
    texture_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader_str = include_shader!("text/text_onscreen.wgsl");
    create_pipeline(
        config,
        device,
        msaa_count,
        "Onscreen text pipeline",
        &[common_uniforms_bind_group_layout, texture_bind_group_layout],
        "vs_onscreen",
        "fs_onscreen",
        shader_str,
        TextPipelineType::Onscreen
    )
}

pub fn create_pipeline(
    config: &SurfaceConfiguration,
    device: &Device,
    msaa_count: u32,
    label: &str,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
    vs_entry_point: &str,
    fs_entry_point: &str,
    shader_str: &str,
    pipeline_ty: TextPipelineType
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some((label.to_string() + " shader").as_str()),
        source: wgpu::ShaderSource::Wgsl(shader_str.into()),
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some((label.to_string() + " layout").as_str()),
        bind_group_layouts: bind_group_layouts,
        push_constant_ranges: &[],
    });

    let vertex_layouts = &[Vertex::desc()];
    let color_format = config.format;
    let depth_format = None;

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Text render pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: vs_entry_point,
            buffers: vertex_layouts,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: fs_entry_point,
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: Some(
                    match pipeline_ty {
                        TextPipelineType::Offscreen => {
                            wgpu::BlendState {
                                color: wgpu::BlendComponent {
                                    dst_factor: wgpu::BlendFactor::One,
                                    src_factor: wgpu::BlendFactor::One,
                                    operation: wgpu::BlendOperation::Add
                                },
                                alpha: wgpu::BlendComponent {
                                    dst_factor: wgpu::BlendFactor::One,
                                    src_factor: wgpu::BlendFactor::One,
                                    operation: wgpu::BlendOperation::Add
                                }
                            }
                        }
                        TextPipelineType::Onscreen => {
                            wgpu::BlendState::ALPHA_BLENDING
                        }
                    }
            ),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
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
            count: msaa_count,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}
