mod pipeline;
mod shared;
mod texture;

use shared::Vertex;
use super::CommonUniforms;
use crate::{app::camera::camera_controller, gui::renderer};

use nalgebra::{Vector2, Vector4};
use rsnet_derive::include_asset_bytes;
use tracing::{error, info};
use ttf_parser::{self, GlyphId};
use wgpu::{core::device, Device, MultisampleState, Queue, RenderPass, SurfaceConfiguration};


#[derive(Debug, Clone, Copy)]
pub enum TriangleKind {
    Solid, // Renders the solid (lineTo) part of the curve
    Quad,  // Renders the quadratic part of the curve
}

pub struct Glyph {
    vertices: Vec<Vertex>,
    name: String,
}

impl Glyph {
    pub fn from_builder(builder: GlyphBuilder, name: String) -> Self {
        Self {
            vertices: builder.vertices,
            name,
        }
    }
}

pub struct GlyphBuilder {
    start_pos: Vector2<f32>,
    current_pos: Vector2<f32>,
    contour_count: usize,
    vertices: Vec<Vertex>,
}

impl GlyphBuilder {
    pub fn new() -> Self {
        Self {
            start_pos: Vector2::new(0.0, 0.0),
            current_pos: Vector2::new(0.0, 0.0),
            contour_count: 0,
            vertices: Vec::new(),
        }
    }

    pub fn add_triangle(
        &mut self,
        a: Vector2<f32>,
        b: Vector2<f32>,
        c: Vector2<f32>,
        kind: TriangleKind,
    ) {
        match kind {
            // Barys act so that x^2 - y < 0 is always true
            TriangleKind::Solid => {
                self.vertices.push(Vertex {
                    position: [a.x, a.y],
                    bary: [0.0, 1.0],
                });
                self.vertices.push(Vertex {
                    position: [b.x, b.y],
                    bary: [0.0, 1.0],
                });
                self.vertices.push(Vertex {
                    position: [c.x, c.y],
                    bary: [0.0, 1.0],
                });
            }
            // Barys are set so that x^2 - y < 0 is only true inside the curve
            // Refer to figure 1 and the equation above in https://www.microsoft.com/en-us/research/wp-content/uploads/2005/01/p1000-loop.pdf
            TriangleKind::Quad => {
                self.vertices.push(Vertex {
                    position: [a.x, a.y],
                    bary: [0.0, 0.0],
                });
                self.vertices.push(Vertex {
                    position: [b.x, b.y],
                    bary: [0.5, 0.0],
                });
                self.vertices.push(Vertex {
                    position: [c.x, c.y],
                    bary: [1.0, 1.0],
                });
            }
        }
    }

    pub fn triangle_count(&self) -> usize {
        self.vertices.len() / 3
    }
}

impl ttf_parser::OutlineBuilder for GlyphBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.start_pos = Vector2::new(x, y);
        self.current_pos = Vector2::new(x, y);
        self.contour_count = 0;
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let to = Vector2::new(x, y);

        self.contour_count += 1;

        if self.contour_count >= 2 {
            self.add_triangle(self.start_pos, self.current_pos, to, TriangleKind::Solid);
        }

        self.current_pos = to;
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {

        let mid = Vector2::new(x1, y1);
        let to = Vector2::new(x, y);

        self.contour_count += 1;

        if self.contour_count >= 2 {
            self.add_triangle(self.start_pos, self.current_pos, to, TriangleKind::Solid);
        }

        self.add_triangle(self.current_pos, mid, to, TriangleKind::Quad);
        self.current_pos = to;

    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        unimplemented!("Cubic curves are not supported");
    }

    fn close(&mut self) {
        self.contour_count = 0;
        // self.line_to(self.start_pos.x, self.start_pos.y)
    }
}

pub struct TextRenderer<'a> {
    offscreen_pipeline: wgpu::RenderPipeline,
    onscreen_pipeline: wgpu::RenderPipeline,

    offscreen_vertex_buffer: shared::VertexBuffer<'a>,
    onscreen_vertex_buffer: shared::VertexBuffer<'a>,

    msaa_count: u32,
    offscreen_texture: texture::Texture,
    onscreen_texture: texture::Texture,

    face: ttf_parser::Face<'a>,
    font_data: &'a [u8],
    glyph_map: std::collections::HashMap<GlyphId, Glyph>,

}

impl<'a> TextRenderer<'a> {
    pub fn new(config: &SurfaceConfiguration, device: &Device, queue: &Queue, common_uniforms_bind_group_layout: &wgpu::BindGroupLayout) -> Self {
        let font_data = include_asset_bytes!("fonts/cmunrm.ttf");

        let face = match ttf_parser::Face::parse(font_data, 0) {
            Ok(face) => face,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        };

        face.number_of_glyphs();
        let a_gid = face.glyph_index('5').unwrap();
        let u32_num = u32::from('Ç');
        let name = face.glyph_name(a_gid).unwrap();
        info!(
            "\nGlyph index for 'a': {:?}\n\tname: {}\n\t u32_num: {}",
            a_gid, name, u32_num
        );

        let mut triangle_count: usize = 0;

        let mut glyph_map = std::collections::HashMap::new();

        for id in 0..face.number_of_glyphs() {
            let gid = ttf_parser::GlyphId(id);
            let name = face.glyph_name(gid).unwrap();

            match face.glyph_svg_image(gid) {
                Some(_svg) => {
                    error!(
                        "Glyph {} has SVG image, we can currently only render outlines",
                        name
                    );
                }
                None => {
                    let mut builder = GlyphBuilder::new();
                    face.outline_glyph(gid, &mut builder);
                    triangle_count += builder.triangle_count();

                    if gid == a_gid {
                        info!("\'a\' triangle_count: {}", builder.triangle_count());
                    }

                    let glyph = Glyph::from_builder(builder, name.to_string());
                    glyph_map.insert(gid, glyph);
                }
            }
        }

        info!("Total triangle count: {}", triangle_count);

        let msaa_count = 1;

        let offscreen_pipeline = pipeline::create_offscreen_pipeline(config, device, 1, common_uniforms_bind_group_layout);

        let mut offscreen_vertex_buffer = shared::VertexBuffer {
            value: Vec::new(),
            label: Some("Text vertex buffer".to_string()),
            buffer: None,
            buffer_layout: Vertex::desc(),
        };
        offscreen_vertex_buffer.set(glyph_map.get(&a_gid).unwrap().vertices.clone());
        offscreen_vertex_buffer.write(device, queue);

        let offscreen_texture = texture::Texture::new(device, 1, config.width, config.height);
        let onscreen_texture = texture::Texture::new(device, msaa_count, config.width, config.height);
        
        let onscreen_pipeline = pipeline::create_onscreen_pipeline(config, device, msaa_count, common_uniforms_bind_group_layout, onscreen_texture.bind_group_layout());

        let mut onscreen_vertex_buffer = shared::VertexBuffer {
            value: Vec::new(),
            label: Some("Text vertex buffer".to_string()),
            buffer: None,
            buffer_layout: Vertex::desc(),
        };
        onscreen_vertex_buffer.set(vec![
            Vertex {
                position: [-1.0, -1.0],
                bary: [0.0, 1.0],
            },
            Vertex {
                position: [-1.0, 1.0],
                bary: [0.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0],
                bary: [1.0, 0.0],
            },
            Vertex {
                position: [-1.0, -1.0],
                bary: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0],
                bary: [1.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0],
                bary: [1.0, 0.0],
            },
        ]);
        onscreen_vertex_buffer.write(device, queue);

        Self {
            face,
            font_data,
            glyph_map,
            offscreen_pipeline,
            onscreen_pipeline,
            msaa_count,
            offscreen_vertex_buffer,
            onscreen_vertex_buffer,
            offscreen_texture,
            onscreen_texture
        }
    }

    pub fn set_msaa_count(&mut self, count: u32) {
        self.msaa_count = count;
    }

    pub fn rebuild_pipeline(&mut self, config: &SurfaceConfiguration, device: &Device, common_uniforms_bind_group_layout: &wgpu::BindGroupLayout) {
        self.offscreen_texture.rebuild(device);
        self.onscreen_texture.rebuild(device);

        self.offscreen_pipeline =
            pipeline::create_offscreen_pipeline(config, device, 1, common_uniforms_bind_group_layout);
        self.onscreen_pipeline =
            pipeline::create_onscreen_pipeline(config, device, self.msaa_count, common_uniforms_bind_group_layout, self.onscreen_texture.bind_group_layout());
    }

    pub fn resize(&mut self, device: &Device, width: u32, height: u32) {
        self.offscreen_texture.resize(device, width, height);
        self.onscreen_texture.resize(device, width, height);
    }

    pub fn render<'b, 'c>(
        &'b mut self,
        device: &'b Device,
        queue: &'b Queue,
        render_pass: &mut RenderPass<'c>,
        common_uniforms_bind_group: &'b wgpu::BindGroup,
        camera_controller: &'b camera_controller::CameraController,
    ) where
        'b: 'c,
    {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let render_pass_desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.offscreen_texture.view,
                    resolve_target: None,
                    // resolve_target: if self.msaa_count == 1 {
                    //     None
                    // } else {
                    //     Some(&self.onscreen_texture.view)
                    // },
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            };

            let mut render_pass = encoder.begin_render_pass(&render_pass_desc);

            render_pass.set_pipeline(&self.offscreen_pipeline);
            
            render_pass.set_bind_group(0, &common_uniforms_bind_group, &[]);
            render_pass
                .set_vertex_buffer(0, self.offscreen_vertex_buffer.buffer().unwrap().slice(..));

            render_pass.draw(0..self.offscreen_vertex_buffer.get().len() as u32, 0..1);
        }

        
        queue.submit(Some(encoder.finish()));

        render_pass.set_pipeline(&self.onscreen_pipeline);
        render_pass.set_vertex_buffer(0, self.onscreen_vertex_buffer.buffer().unwrap().slice(..));

        render_pass.set_bind_group(0, &common_uniforms_bind_group, &[]);
        render_pass.set_bind_group(1, &self.offscreen_texture.bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}
