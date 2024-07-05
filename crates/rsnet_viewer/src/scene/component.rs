use std::collections::HashMap;

use crate::{
    renderer::primitives::{self, common::{
        DIODE_PRIMITIVES_L0, MEMRISTOR_PRIMITIVES_L0, MEMRISTOR_PRIMITIVES_L1, NMOS_PRIMITIVES_L0,
        OMP_AMP_PRIMITIVES_L0, RESISTOR_PRIMITIVES_L0,
    }, Port},
    types::Id,
};

use super::types::{ComponentType, Primitives};

use egui::epaint::Primitive;
use nalgebra::{Matrix3, Vector2};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Specifies the distance at which the component should be visible
#[derive(Debug, Copy, Clone)]
pub enum Range {
    Near,
    Mid,
    Far,
}

// A component is a renderable thing. It might be a single memristor or a full crossbar.
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

    transform: Matrix3<f32>,
    ports: Vec<Port>,
}

impl Component {
    pub fn new(
        id: u32,
        attachment: u32,
        position: Vector2<f32>,
        rotation: f32,
        ty: ComponentType,
    ) -> Component {
        Component {
            range: Range::Mid,
            ty: ty,
            id: id,
            attachment: attachment,
            position: position,
            rotation: rotation,
            scale: 1.0,
            transform: Component::compute_transform(&position, 1.0, rotation),
            ports: vec![]
        }
    }

    pub fn position(&self) -> &Vector2<f32> {
        &self.position
    }

    pub fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
        self.update_transform();
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.update_transform();
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.update_transform();
    }

    pub fn transform(&self) -> &Matrix3<f32> {
        &self.transform
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn ty(&self) -> ComponentType {
        self.ty
    }

    pub fn update_transform(&mut self) {
        self.transform = Component::compute_transform(&self.position, self.scale, self.rotation);
    }

    fn compute_transform(position: &Vector2<f32>, scale: f32, angle: f32) -> Matrix3<f32> {
        Matrix3::new_translation(position.into())
            * Matrix3::from_diagonal_element(scale)
            * Matrix3::new_rotation(angle)
    }

    pub fn ports(&self, primitives: &Primitives) -> Vec<Port> {
        vec![]
        // primitives
        //     .0
        //     .get(&self.ty.into())
        //     .unwrap_or_else(||vec![])
        //     .first()
        //     .unwrap_or_else(|| vec![])
    }

}

#[derive(Debug, EnumIter)]
#[repr(u32)]
pub enum DefaultComponentTypes {
    Memristor = 0,
    Resistor = 1,
    Nmos = 2,
    OpAmp = 3,
    Diode = 4,
}

impl Into<u32> for DefaultComponentTypes {
    fn into(self) -> u32 {
        self as u32
    }
}

impl DefaultComponentTypes {
    pub fn primitives() -> Primitives {
        let mut primitives = Primitives(HashMap::new());

        DefaultComponentTypes::iter().for_each(|ty| match ty {
            DefaultComponentTypes::Memristor => {
                primitives.0.insert(
                    ty as u32,
                    vec![
                        (&MEMRISTOR_PRIMITIVES_L0, 400.0),
                        (&MEMRISTOR_PRIMITIVES_L1, 1200.0),
                    ],
                );
            }
            DefaultComponentTypes::Resistor => {
                primitives
                    .0
                    .insert(ty as u32, vec![(&RESISTOR_PRIMITIVES_L0, 400.0)]);
            }
            DefaultComponentTypes::Nmos => {
                primitives
                    .0
                    .insert(ty as u32, vec![(&NMOS_PRIMITIVES_L0, 400.0)]);
            }
            DefaultComponentTypes::OpAmp => {
                primitives
                    .0
                    .insert(ty as u32, vec![(&OMP_AMP_PRIMITIVES_L0, 400.0)]);
            }
            DefaultComponentTypes::Diode => {
                primitives
                    .0
                    .insert(ty as u32, vec![(&DIODE_PRIMITIVES_L0, 400.0)]);
            }
        });

        primitives
    }
}
