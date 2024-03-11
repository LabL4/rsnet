use super::ComponentTyPrimitives;

use crate::renderer::primitives::*;

use lazy_static::lazy_static;
use nalgebra::Vector2;

const MEMRISTOR_HEIGHT: f32 = 1.0;
const MEMRISTOR_WIDTH: f32 = 0.4;
const MEMRISTOR_LOWER_RECT_HEIGHT: f32 = 0.05;
const MEMRISTOR_UPPER_HEIGHT: f32 = MEMRISTOR_HEIGHT - MEMRISTOR_LOWER_RECT_HEIGHT;
const N_VERTICAL_DIVS: f32 = 5.0;
const LINE_THICKNESS: f32 = 0.03;

lazy_static!(
    pub static ref MEMRISTOR_PRIMITIVES_L0: ComponentTyPrimitives = {


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

        ComponentTyPrimitives {
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
            ],
            triangles: vec![]
        }
    
    };

    pub static ref MEMRISTOR_PRIMITIVES_L1: ComponentTyPrimitives = {
        ComponentTyPrimitives {
            circles: vec![],
            lines: vec![],
            rectangles: vec![
                RectanglePrimitive {
                    position: Vector2::new(0.0, 0.0),
                    size: Vector2::new(MEMRISTOR_WIDTH, MEMRISTOR_HEIGHT),
                    color: 0x000000
                },
            ],
            triangles: vec![]
        }
    };
    
    
    pub static ref OMP_AMP_PRIMITIVES_L0: ComponentTyPrimitives = {
        
        const OPAMP_WIDTH: f32 = 1.0;
        const OPAMP_HEIGHT: f32 = 1.0;

        let mut outline: Vec<Vector2<f32>> = vec![];
        outline.push(Vector2::new(-OPAMP_WIDTH / 2.0, 0.0));
        outline.push(Vector2::new(-OPAMP_WIDTH / 2.0, OPAMP_HEIGHT / 2.0));
        outline.push(Vector2::new(OPAMP_WIDTH / 2.0, 0.0));
        outline.push(Vector2::new(-OPAMP_WIDTH / 2.0, -OPAMP_HEIGHT / 2.0));
        outline.push(Vector2::new(-OPAMP_WIDTH / 2.0, 0.0));

        ComponentTyPrimitives {
            circles: vec![],
            lines: vec![
                LinePrimitive {
                    positions: outline,
                    thickness: LINE_THICKNESS,
                    color: 0x000000
                }
            ],
            rectangles: vec![],
            triangles: vec![]
        }
    };

    pub static ref NMOS_PRIMITIVES_L0: ComponentTyPrimitives = {

        const NMOS_WIDTH: f32 = 0.7;
        const NMOS_HEIGHT: f32 = 1.0;
        // const TERMINAL_WIDTH: f32 = ;
        const OXIDE_WIDTH: f32 = 0.1;
        const GATE_THICKNESS: f32 = 0.04;

        let gate_line = vec![
            Vector2::new(-NMOS_WIDTH / 2.0, 0.0),
            Vector2::new(-NMOS_WIDTH / 2.0 + NMOS_WIDTH / 2.8, 0.0)
        ];
        
        let drain_line = vec![
            Vector2::new(NMOS_WIDTH / 2.0, -NMOS_HEIGHT / 2.0),
            Vector2::new(NMOS_WIDTH / 2.0, -NMOS_HEIGHT / 2.0 + NMOS_HEIGHT / 3.0),
            Vector2::new(gate_line.last().unwrap().x + OXIDE_WIDTH, -NMOS_HEIGHT / 2.0 + NMOS_HEIGHT / 3.0),
        ];
        
        let source_line = vec![
            Vector2::new(NMOS_WIDTH / 2.0, NMOS_HEIGHT / 2.0),
            Vector2::new(NMOS_WIDTH / 2.0, NMOS_HEIGHT / 2.0 - NMOS_HEIGHT / 3.0),
            Vector2::new(gate_line.last().unwrap().x + OXIDE_WIDTH, NMOS_HEIGHT / 2.0 - NMOS_HEIGHT / 3.0),
        ];

        let mid_horz_terminal = Vector2::new((drain_line.last().unwrap().x + NMOS_WIDTH / 2.0) / 2.0, drain_line.last().unwrap().y);

        ComponentTyPrimitives {
            circles: vec![],
            lines: vec![
                LinePrimitive {
                    positions: gate_line.clone(),
                    thickness: LINE_THICKNESS*0.8,
                    color: 0x000000
                },
                LinePrimitive {
                    positions: drain_line,
                    thickness: LINE_THICKNESS*0.8,
                    color: 0x000000
                },
                LinePrimitive {
                    positions: source_line,
                    thickness: LINE_THICKNESS*0.8,
                    color: 0x000000
                }
            ],
            rectangles: vec![
                RectanglePrimitive {
                    position: gate_line.last().unwrap().clone() + Vector2::new(OXIDE_WIDTH + GATE_THICKNESS/2.0, 0.0),
                    size: Vector2::new(GATE_THICKNESS, 0.52 * NMOS_HEIGHT),
                    color: 0x000000
                },
                RectanglePrimitive {
                    position: gate_line.last().unwrap().clone() - Vector2::new(- GATE_THICKNESS/2.0, 0.0),
                    size: Vector2::new(GATE_THICKNESS, 0.32 * NMOS_HEIGHT),
                    color: 0x000000
                }
            ],
            triangles: vec![
                TrianglePrimitive {
                    position: mid_horz_terminal + Vector2::new(GATE_THICKNESS/2.0, 0.0),
                    size: Vector2::new(NMOS_WIDTH * 0.2, NMOS_WIDTH * 0.2),
                    dir_vec: Vector2::new(0.0, -1.0),
                    color: 0x000000
                }
            ]
        }
    };
);