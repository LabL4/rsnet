use super::{component::Component, ChunkId};

use crate::renderer::utils::StorageBufferData;

use nalgebra::Matrix3;
use encase::ShaderType;


/// This will be the buffer that holds all the components for the entities
#[derive(ShaderType, Debug, Default, Clone)]
pub struct ComponentBufferEntry {
    pub model: Matrix3<f32>,
    pub id: u32,
    pub ty: u32
}

impl ComponentBufferEntry {
    pub fn from_component(component: &Component) -> Self {
        let model = Matrix3::new_translation(&component.position().xy().into())
                                                        * Matrix3::from_diagonal_element(component.scale())
                                                        * Matrix3::new_rotation(component.rotation());

        Self {
            id: component.id(),
            model,
            ty: component.ty().into()
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

/// The Scene struct contains the full Scene data, and the SceneStorage struct contains the data
/// that is used to render the visible part of the scene.
pub struct SceneStorage {
    pub components: StorageBufferData<Vec<ComponentBufferEntry>>,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}