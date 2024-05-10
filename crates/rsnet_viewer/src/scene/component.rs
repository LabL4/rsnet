use std::collections::HashMap;

use crate::{
    renderer::primitives::common::{
        DIODE_PRIMITIVES_L0,
        MEMRISTOR_PRIMITIVES_L0,
        MEMRISTOR_PRIMITIVES_L1,
        NMOS_PRIMITIVES_L0,
        OMP_AMP_PRIMITIVES_L0,
        RESISTOR_PRIMITIVES_L0
    },
    types::Id
};

use super::types::{ComponentType, Primitives};

use egui::epaint::Primitive;
use nalgebra::Vector2;
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


#[derive(Debug, EnumIter)]
#[repr(u32)]
pub enum DefaultComponentTypes {
    Memristor = 0,
    Resistor = 1,
    Nmos = 2,
    OpAmp = 3,
    Diode = 4,
}

impl DefaultComponentTypes {
    pub fn primitives() -> Primitives {
        let mut primitives = Primitives(HashMap::new());
        // primitives.0.insert(
        //     0,
        //     vec![
        //         (&MEMRISTOR_PRIMITIVES_L0, 400.0),
        //         (&MEMRISTOR_PRIMITIVES_L1, 1200.0),
        //     ],
        // );

        
        // primitives.0
        //     .insert(1, vec![(&OMP_AMP_PRIMITIVES_L0, 400.0)]);
        
        // primitives.0
        //     .insert(2, vec![(&NMOS_PRIMITIVES_L0, 400.0)]);

        
        // primitives.0
        //     .insert(3, vec![(&RESISTOR_PRIMITIVES_L0, 400.0)]);

        DefaultComponentTypes::iter().for_each(|ty| {
            match ty {
                DefaultComponentTypes::Memristor => {
                    primitives.0.insert(ty as u32, vec![(&MEMRISTOR_PRIMITIVES_L0, 400.0), (&MEMRISTOR_PRIMITIVES_L1, 1200.0)]);
                }
                DefaultComponentTypes::Resistor => {
                    primitives.0.insert(ty as u32, vec![(&RESISTOR_PRIMITIVES_L0, 400.0)]);
                }
                DefaultComponentTypes::Nmos => {
                    primitives.0.insert(ty as u32, vec![(&NMOS_PRIMITIVES_L0, 400.0)]);
                }
                DefaultComponentTypes::OpAmp => {
                    primitives.0.insert(ty as u32, vec![(&OMP_AMP_PRIMITIVES_L0, 400.0)]);
                },
                DefaultComponentTypes::Diode => {
                    primitives.0.insert(ty as u32, vec![(&DIODE_PRIMITIVES_L0, 400.0)]);
                }
            }
        });

        primitives
    }
}