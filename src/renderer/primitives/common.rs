use super::Primitives;

use crate::renderer::primitives::{CirclePrimitive, LinePrimitive, RectanglePrimitive, shared::Fragments};

use egui::Rect;
use lazy_static::lazy_static;
use nalgebra::Vector2;
use rayon::vec;

lazy_static!(
    pub static ref MEMRISTOR_PRIMITIVES: Primitives = Primitives {
        circles: vec![
            // CirclePrimitive {
            //     position: Vector2::new(0.0, 0.0),
            //     radius: 1.0,
            //     color: 0x000000
            // }
        ],
        lines: vec![
            LinePrimitive {
                positions: vec![Vector2::new(0.0, 0.0), Vector2::new(2.0, 1.0)],
                thickness: 1.0,
                color: 0xF5A142
            }
        ],
        rectangles: vec![
            // RectanglePrimitive {
            //     position: Vector2::new(2.0, 1.0),
            //     size: Vector2::new(2.0, 1.0),
            //     color: 0xFFFFFF
            // }
        ]
    };
);