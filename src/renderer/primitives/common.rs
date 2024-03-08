use super::Primitives;

use crate::renderer::primitives::{CirclePrimitive, LinePrimitive, RectanglePrimitive, shared::Fragments};

use egui::Rect;
use lazy_static::lazy_static;
use nalgebra::Vector2;
use rayon::vec;

const MEMRISTOR_HEIGHT: f32 = 1.0;
const MEMRISTOR_WIDTH: f32 = 0.4;
const MEMRISTOR_LOWER_RECT_HEIGHT: f32 = 0.05;
const MEMRISTOR_UPPER_HEIGHT: f32 = MEMRISTOR_HEIGHT - MEMRISTOR_LOWER_RECT_HEIGHT;
const N_VERTICAL_DIVS: f32 = 5.0;
const LINE_THICKNESS: f32 = 0.03;

lazy_static!(
    pub static ref MEMRISTOR_PRIMITIVES_L0: Primitives = {


        let mut central_line: Vec<Vector2<f32>> = vec![];

        central_line.push(Vector2::new(0.0, MEMRISTOR_HEIGHT / 2.0));
        central_line.push(Vector2::new(central_line.last().unwrap().x, central_line.last().unwrap().y - MEMRISTOR_UPPER_HEIGHT / N_VERTICAL_DIVS));
        central_line.push(Vector2::new(central_line.last().unwrap().x + MEMRISTOR_WIDTH / 4.0, central_line.last().unwrap().y));
        central_line.push(Vector2::new(central_line.last().unwrap().x, central_line.last().unwrap().y - MEMRISTOR_UPPER_HEIGHT / N_VERTICAL_DIVS));
        central_line.push(Vector2::new(central_line.last().unwrap().x - MEMRISTOR_WIDTH / 2.0, central_line.last().unwrap().y));
        central_line.push(Vector2::new(central_line.last().unwrap().x, central_line.last().unwrap().y - MEMRISTOR_UPPER_HEIGHT / N_VERTICAL_DIVS));
        central_line.push(Vector2::new(central_line.last().unwrap().x + MEMRISTOR_WIDTH / 2.0, central_line.last().unwrap().y));
        central_line.push(Vector2::new(central_line.last().unwrap().x, central_line.last().unwrap().y - MEMRISTOR_UPPER_HEIGHT / N_VERTICAL_DIVS));
        central_line.push(Vector2::new(central_line.last().unwrap().x - MEMRISTOR_WIDTH / 4.0, central_line.last().unwrap().y));
        central_line.push(Vector2::new(central_line.last().unwrap().x, central_line.last().unwrap().y - MEMRISTOR_UPPER_HEIGHT / N_VERTICAL_DIVS));

        let mut outline: Vec<Vector2<f32>> = vec![];

        outline.push(Vector2::new(-MEMRISTOR_WIDTH / 2.0 - LINE_THICKNESS/2.0, -MEMRISTOR_HEIGHT / 2.0));
        outline.push(Vector2::new(MEMRISTOR_WIDTH / 2.0, -MEMRISTOR_HEIGHT / 2.0));
        outline.push(Vector2::new(MEMRISTOR_WIDTH / 2.0, MEMRISTOR_HEIGHT / 2.0));
        outline.push(Vector2::new(-MEMRISTOR_WIDTH / 2.0, MEMRISTOR_HEIGHT / 2.0));
        outline.push(Vector2::new(-MEMRISTOR_WIDTH / 2.0, -MEMRISTOR_HEIGHT / 2.0));

        // Primitives {
        //     circles: vec![],
        //     lines: vec![],
        //     rectangles: vec![
        //         RectanglePrimitive {
        //             position: Vector2::new(0.0, 0.0),
        //             size: Vector2::new(MEMRISTOR_WIDTH, MEMRISTOR_HEIGHT),
        //             color: 0x000000
        //         },
        //     ]
        // }

        Primitives {
            circles: vec![
                // CirclePrimitive {
                //     position: Vector2::new(0.0, 0.0),
                //     radius: 1.0,
                //     color: 0x000000
                // },
                // CirclePrimitive {
                //     position: Vector2::new(2.0, -10.0),
                //     radius: 1.0,
                //     color: 0xFFFFFF
                // },

            ],
            lines: vec![
                LinePrimitive {
                    positions: central_line,
                    thickness: LINE_THICKNESS,
                    // color: 0xF5A142
                    color: 0x000000
                },
                LinePrimitive {
                    positions: outline,
                    thickness: LINE_THICKNESS,
                    // color: 0xF5A142
                    color: 0x000000
                }
            ],
            rectangles: vec![
                RectanglePrimitive {
                    position: Vector2::new(0.0, -MEMRISTOR_HEIGHT/2.0 + MEMRISTOR_LOWER_RECT_HEIGHT/2.0),
                    size: Vector2::new(MEMRISTOR_WIDTH, MEMRISTOR_LOWER_RECT_HEIGHT),
                    color: 0x000000
                },
            ]
        }
    
    };

    pub static ref MEMRISTOR_PRIMITIVES_L1: Primitives = {
        Primitives {
            circles: vec![],
            lines: vec![],
            rectangles: vec![
                RectanglePrimitive {
                    position: Vector2::new(0.0, 0.0),
                    size: Vector2::new(MEMRISTOR_WIDTH, MEMRISTOR_HEIGHT),
                    color: 0x000000
                },
            ]
        }
    };
    
    
    pub static ref OMP_AMP_PRIMITIVES_L1: Primitives = {
        
        const OPAMP_WIDTH: f32 = 1.0;
        const OPAMP_HEIGHT: f32 = 1.0;

        let mut outline: Vec<Vector2<f32>> = vec![];
        outline.push(Vector2::new(-OPAMP_WIDTH / 2.0, 0.0));
        outline.push(Vector2::new(-OPAMP_WIDTH / 2.0, OPAMP_HEIGHT / 2.0));
        outline.push(Vector2::new(OPAMP_WIDTH / 2.0, 0.0));
        outline.push(Vector2::new(-OPAMP_WIDTH / 2.0, -OPAMP_HEIGHT / 2.0));
        outline.push(Vector2::new(-OPAMP_WIDTH / 2.0, 0.0));

        Primitives {
            circles: vec![],
            lines: vec![
                LinePrimitive {
                    positions: outline,
                    thickness: LINE_THICKNESS,
                    color: 0x000000
                }
            ],
            rectangles: vec![
                
            ]
        }
    };
);