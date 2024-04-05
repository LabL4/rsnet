use super::ComponentTyPrimitives;

use crate::renderer::primitives::*;

use lazy_static::lazy_static;
use nalgebra::Vector2;

const MEMRISTOR_HEIGHT: f32 = 0.6;
const MEMRISTOR_WIDTH: f32 = 0.2;
const MEMRISTOR_LOWER_RECT_HEIGHT: f32 = 0.05;
const MEMRISTOR_UPPER_HEIGHT: f32 = MEMRISTOR_HEIGHT - MEMRISTOR_LOWER_RECT_HEIGHT;
const MEMRISTOR_N_VERTICAL_DIVS: f32 = 5.0;
const MEMRISTOR_LINE_THICKNESS: f32 = 0.02;

const LINE_THICKNESS: f32 = 0.03;

lazy_static!(
    pub static ref UNKNOWN_PRIMITIVE: ComponentTyPrimitives = ComponentTyPrimitives {
        circles: vec![],
        lines: vec![],
        rectangles: vec![
            RectanglePrimitive {
                position: Vector2::new(0.0, 0.0),
                size: Vector2::new(1.0, 1.0),
                color: 0xFF0000
            }
        ],
        triangles: vec![]
    };

    pub static ref MEMRISTOR_PRIMITIVES_L0: ComponentTyPrimitives = {

        let mut central_line: Vec<Vector2<f32>> = vec![];

        central_line.push(Vector2::new(0.0, MEMRISTOR_HEIGHT / 2.0));
        central_line.push(Vector2::new(central_line.last().unwrap().x, central_line.last().unwrap().y - MEMRISTOR_UPPER_HEIGHT / MEMRISTOR_N_VERTICAL_DIVS));
        central_line.push(Vector2::new(central_line.last().unwrap().x + MEMRISTOR_WIDTH / 4.0, central_line.last().unwrap().y));
        central_line.push(Vector2::new(central_line.last().unwrap().x, central_line.last().unwrap().y - MEMRISTOR_UPPER_HEIGHT / MEMRISTOR_N_VERTICAL_DIVS));
        central_line.push(Vector2::new(central_line.last().unwrap().x - MEMRISTOR_WIDTH / 2.0, central_line.last().unwrap().y));
        central_line.push(Vector2::new(central_line.last().unwrap().x, central_line.last().unwrap().y - MEMRISTOR_UPPER_HEIGHT / MEMRISTOR_N_VERTICAL_DIVS));
        central_line.push(Vector2::new(central_line.last().unwrap().x + MEMRISTOR_WIDTH / 2.0, central_line.last().unwrap().y));
        central_line.push(Vector2::new(central_line.last().unwrap().x, central_line.last().unwrap().y - MEMRISTOR_UPPER_HEIGHT / MEMRISTOR_N_VERTICAL_DIVS));
        central_line.push(Vector2::new(central_line.last().unwrap().x - MEMRISTOR_WIDTH / 4.0, central_line.last().unwrap().y));
        central_line.push(Vector2::new(central_line.last().unwrap().x, central_line.last().unwrap().y - MEMRISTOR_UPPER_HEIGHT / MEMRISTOR_N_VERTICAL_DIVS));

        let mut outline: Vec<Vector2<f32>> = vec![];

        outline.push(Vector2::new(-MEMRISTOR_WIDTH / 2.0 - MEMRISTOR_LINE_THICKNESS/2.0, -MEMRISTOR_HEIGHT / 2.0));
        outline.push(Vector2::new(MEMRISTOR_WIDTH / 2.0, -MEMRISTOR_HEIGHT / 2.0));
        outline.push(Vector2::new(MEMRISTOR_WIDTH / 2.0, MEMRISTOR_HEIGHT / 2.0));
        outline.push(Vector2::new(-MEMRISTOR_WIDTH / 2.0, MEMRISTOR_HEIGHT / 2.0));
        outline.push(Vector2::new(-MEMRISTOR_WIDTH / 2.0, -MEMRISTOR_HEIGHT / 2.0));

        ComponentTyPrimitives {
            circles: vec![],
            lines: vec![
                LinePrimitive {
                    positions: central_line,
                    thickness: MEMRISTOR_LINE_THICKNESS,
                    // color: 0xF5A142
                    color: 0x000000,
                    line_cap_ty: 0
                },
                LinePrimitive {
                    positions: outline,
                    thickness: MEMRISTOR_LINE_THICKNESS,
                    // color: 0xF5A142
                    color: 0x000000,
                    line_cap_ty: 0
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
        
        const OPAMP_WIDTH: f32 = 0.65;
        const OPAMP_HEIGHT: f32 = 0.8;
        const SYMBOLS_DESPL: f32 = 0.11;
        const SYMBOLS_SIZE: f32 = 0.1;
        const SYMBOLS_THICKNESS: f32 = 0.02;
        const TERMINAL_LEN: f32 = 0.2;

        let mut outline: Vec<Vector2<f32>> = vec![];
        outline.push(Vector2::new(-OPAMP_WIDTH / 2.0, 0.0));
        outline.push(Vector2::new(-OPAMP_WIDTH / 2.0, OPAMP_HEIGHT / 2.0));
        outline.push(Vector2::new(OPAMP_WIDTH / 2.0, 0.0));
        outline.push(Vector2::new(-OPAMP_WIDTH / 2.0, -OPAMP_HEIGHT / 2.0));
        outline.push(Vector2::new(-OPAMP_WIDTH / 2.0, 0.0));

        let plus_sign_center = Vector2::new(-OPAMP_WIDTH / 2.0 + SYMBOLS_DESPL, 0.45 * -OPAMP_HEIGHT/2.0);
        let minus_sign_center = Vector2::new(-OPAMP_WIDTH / 2.0 + SYMBOLS_DESPL, 0.45 * OPAMP_HEIGHT/2.0);

        let minus_terminal = vec![
            Vector2::new(-OPAMP_WIDTH / 2.0, minus_sign_center.y),
            Vector2::new(-OPAMP_WIDTH / 2.0 - TERMINAL_LEN, minus_sign_center.y)
        ];

        let plus_terminal = vec![
            Vector2::new(-OPAMP_WIDTH / 2.0, plus_sign_center.y),
            Vector2::new(-OPAMP_WIDTH / 2.0 - TERMINAL_LEN, plus_sign_center.y)
        ];

        let out_terminal = vec![
            Vector2::new(OPAMP_WIDTH / 2.0, 0.0),
            Vector2::new(OPAMP_WIDTH / 2.0 + TERMINAL_LEN, 0.0)
        ];


        ComponentTyPrimitives {
            circles: vec![],
            lines: vec![
                LinePrimitive {
                    positions: outline,
                    thickness: LINE_THICKNESS,
                    color: 0x000000,
                    line_cap_ty: 1
                },
                LinePrimitive {
                    positions: plus_terminal,
                    thickness: LINE_THICKNESS,
                    color: 0x000000,
                    line_cap_ty: 0
                },
                LinePrimitive {
                    positions: minus_terminal,
                    thickness: LINE_THICKNESS,
                    color: 0x000000,
                    line_cap_ty: 0
                },
                LinePrimitive {
                    positions: out_terminal,
                    thickness: LINE_THICKNESS,
                    color: 0x000000,
                    line_cap_ty: 0
                }
            ],
            rectangles: vec![
                RectanglePrimitive {
                    position: plus_sign_center,
                    size: Vector2::new(SYMBOLS_SIZE, SYMBOLS_THICKNESS),
                    color: 0x000000
                },
                RectanglePrimitive {
                    position: plus_sign_center,
                    size: Vector2::new(SYMBOLS_THICKNESS, SYMBOLS_SIZE),
                    color: 0x000000
                },
                RectanglePrimitive {
                    position: minus_sign_center,
                    size: Vector2::new(SYMBOLS_SIZE, SYMBOLS_THICKNESS),
                    color: 0x000000
                }
            ],
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
                    color: 0x000000,
                    line_cap_ty: 0
                },
                LinePrimitive {
                    positions: drain_line,
                    thickness: LINE_THICKNESS*0.8,
                    color: 0x000000,
                    line_cap_ty: 0
                },
                LinePrimitive {
                    positions: source_line,
                    thickness: LINE_THICKNESS*0.8,
                    color: 0x000000,
                    line_cap_ty: 0
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