use std::primitive;

use wgpu::util::DeviceExt;
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device};

use super::super::utils::{storage_as_wgsl_bytes, StorageBufferData};
use super::shared::*;
use super::Primitives;

pub fn attach_empty_fragment_storage(device: &Device) -> FragmentsStorage {
    let mut fragments = StorageBufferData::empty(Vec::new());
    fragments.set_label(Some("Components storage buffer"));
    fragments.add_usages(wgpu::BufferUsages::STORAGE);
    fragments.add_usages(wgpu::BufferUsages::COPY_DST);

    let fragments_encase_buffer = storage_as_wgsl_bytes(&fragments.get()).unwrap();
    let fragments_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Scene storage buffer"),
        contents: &fragments_encase_buffer.as_ref(),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Fragments storage bind group layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            count: None,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            visibility: wgpu::ShaderStages::VERTEX,
        }],
    });

    let bind_group = create_fragment_storage_bind_group(device, &bind_group_layout, &fragments_buffer);

    FragmentsStorage {
        fragments,
        bind_group,
        bind_group_layout,
    }
}

pub fn create_fragment_storage_bind_group(device: &Device, layout: &BindGroupLayout, fragments_buffer: &Buffer) -> BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Fragments storage bind group"),
        layout: layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: fragments_buffer.as_entire_binding(),
        }],
    })
}