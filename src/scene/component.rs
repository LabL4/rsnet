use crate::utils::Id;

use nalgebra::Vector2;

// A component is a renderable thing. It might be a single memristor or a full crossbar.

/// Specifies the distance at which the component should be visible
#[derive(Debug, Copy, Clone)]
pub enum Range {
    Near,
    Mid,
    Far
}

pub type ComponentType = u32;

#[derive(Debug)]
pub struct Component {
    range: Range,
    ty: ComponentType,

    id: Id,
    attachment: u32,
    position: Vector2<f32>,
    /// The rotation of the component in radians
    rotation: f32,
    scale: f32,
}

impl Component {
    pub fn new(id: u32, attachment: u32, position: Vector2<f32>, rotation: f32, ty: ComponentType) -> Component {
        Component {
            range: Range::Mid,
            ty: ty,
            id: id,
            attachment: attachment,
            position: position,
            rotation: rotation,
            scale: 1.0
        }
    }

    pub fn position(&self) -> &Vector2<f32> {
        &self.position
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn ty(&self) -> ComponentType {
        self.ty
    }

}

