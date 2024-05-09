use std::thread::panicking;

use encase::ShaderType;
use nalgebra::Vector2;
use tracing::info;
use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, Buffer, Device, Queue,
};

use crate::{
    renderer::{
        primitives::utils::{
            attach_buffer, component_primitives_vec_to_fragments,
            fragments_bind_group_layout_descriptor,
        },
        utils::{uniform_as_wgsl_bytes, StorageBufferData, UniformBufferData},
    },
    utils::retain_by_range,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub bary: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    // Position
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    // Barycentric coordinates
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub struct VertexBuffer<'a> {
    pub value: Vec<Vertex>,
    pub label: Option<String>,
    // pub scratch: Option<Vec<Vertex>>,
    pub buffer: Option<wgpu::Buffer>,
    pub buffer_layout: wgpu::VertexBufferLayout<'a>,
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
            self.buffer = Some(
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: self.label.as_deref(),
                    contents: byte_data,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }),
            );
        } else {
            queue.write_buffer(self.buffer.as_ref().unwrap(), 0, byte_data);
        }
    }
}
