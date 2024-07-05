use super::component;
use super::types;
use super::wire;

use component::Component;
use types::ChunkId;
use wire::{Wire, WireSegment};

use crate::renderer::utils::{storage_as_wgsl_bytes, StorageBufferData};

use encase::ShaderType;
use nalgebra::{Matrix3, Vector2};
use rayon::vec;
use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Device, Queue};

/// This will be the buffer that holds all the components for the entities
#[derive(ShaderType, Debug, Default, Clone)]
pub struct ComponentBufferEntry {
    pub model: Matrix3<f32>,
    pub id: u32,
    pub ty: u32,
}

impl ComponentBufferEntry {
    pub fn from_component(component: &Component) -> Self {
        let model = component.transform().clone();
        // let model = Matrix3::new_translation(&component.position().xy().into())
        //     * Matrix3::from_diagonal_element(component.scale())
        //     * Matrix3::new_rotation(component.rotation());

        Self {
            id: component.id(),
            model,
            ty: component.ty().into(),
        }
    }

    pub fn ty(&self) -> u32 {
        self.ty
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn model(&self) -> &Matrix3<f32> {
        &self.model
    }
}

#[derive(ShaderType, Debug, Default, Clone)]
pub struct WireBufferEntry {
    pub id: u32,
    pub start: Vector2<f32>,
    pub end: Vector2<f32>,
    pub prev_direction: Vector2<f32>,
    pub next_direction: Vector2<f32>,
    // pub circle_overlay: u32
}

impl WireBufferEntry {
    pub fn from_wire(wire: &Wire) -> Self {
        Self {
            id: wire.id(),
            start: wire.start().clone(),
            end: wire.end().clone(),
            prev_direction: wire.prev_direction().clone(),
            next_direction: wire.next_direction().clone(),
            // circle_overlay: wire.circle_overlay(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn start(&self) -> &Vector2<f32> {
        &self.start
    }

    pub fn end(&self) -> &Vector2<f32> {
        &self.end
    }
}

/// The Scene struct contains tdxe full Scene data, and the SceneStorage struct contains the data
/// that is used to render the visible part of the scene.
pub struct SceneStorage {
    pub wires: StorageBufferData<Vec<WireBufferEntry>>,
    pub components: StorageBufferData<Vec<ComponentBufferEntry>>,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl SceneStorage {
    pub fn write(&mut self, device: &Device, queue: &Queue) {
        let mut new_bg = false;

        let new_bg = vec![
            self.components.write_buffer(device, queue),
            self.wires.write_buffer(device, queue),
        ];

        if new_bg.iter().any(|v| *v) {
            self.bind_group = Self::create_bind_group(
                device,
                &self.bind_group_layout,
                self.components.buffer().unwrap(),
                self.wires.buffer().unwrap(),
            );
        }
    }

    pub fn create_bind_group(
        device: &Device,
        layout: &BindGroupLayout,
        components_buffer: &wgpu::Buffer,
        wire_segments_buffer: &wgpu::Buffer,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Scene storage bind group"),
            layout: layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: components_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wire_segments_buffer.as_entire_binding(),
                },
            ],
        })
    }

    pub fn attach_empty(device: &Device) -> Self {
        let mut components = StorageBufferData::empty(Vec::new());
        components.set_label(Some("Components storage buffer"));
        components.add_usages(wgpu::BufferUsages::STORAGE);
        components.add_usages(wgpu::BufferUsages::COPY_DST);

        let components_encase_buffer = storage_as_wgsl_bytes(&components.get()).unwrap();
        let components_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Components storage array"),
            contents: &components_encase_buffer.as_ref(),
            usage: components.usages(),
        });

        let mut wire_segments = StorageBufferData::empty(Vec::new());
        wire_segments.set_label(Some("Wire segments storage buffer"));
        wire_segments.add_usages(wgpu::BufferUsages::STORAGE);
        wire_segments.add_usages(wgpu::BufferUsages::COPY_DST);

        let wire_segments_encase_buffer = storage_as_wgsl_bytes(&wire_segments.get()).unwrap();
        let wire_segments_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Wire segments storage array"),
            contents: &wire_segments_encase_buffer.as_ref(),
            usage: wire_segments.usages(),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Scene storage bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    // Components
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
                    // Wire segments
                    binding: 1,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    visibility: wgpu::ShaderStages::VERTEX,
                },
            ],
        });

        let bind_group = create_scene_storage_bind_group(
            device,
            &bind_group_layout,
            &components_buffer,
            &wire_segments_buffer,
        );

        SceneStorage {
            wires: wire_segments,
            components,
            bind_group,
            bind_group_layout,
        }
    }
}

pub fn create_scene_storage_bind_group(
    device: &Device,
    layout: &wgpu::BindGroupLayout,
    components_buffer: &wgpu::Buffer,
    wire_segment_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Scene storage bind group"),
        layout: layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: components_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wire_segment_buffer.as_entire_binding(),
            },
        ],
    })
}
