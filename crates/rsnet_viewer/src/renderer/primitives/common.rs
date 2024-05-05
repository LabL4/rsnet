use super::ComponentTyPrimitives;

use crate::renderer::primitives::*;

use lazy_static::lazy_static;
use nalgebra::Vector2;

const MEMRISTOR_HEIGHT: f32 = 0.6 * 1.3;
const MEMRISTOR_WIDTH: f32 = 0.2 * 1.3;
const MEMRISTOR_LOWER_RECT_HEIGHT: f32 = 0.05;
const MEMRISTOR_UPPER_HEIGHT: f32 = MEMRISTOR_HEIGHT - MEMRISTOR_LOWER_RECT_HEIGHT;
const MEMRISTOR_N_VERTICAL_DIVS: f32 = 5.0;
const MEMRISTOR_LINE_THICKNESS: f32 = 0.02;
const MEMRISTOR_TERMINAL_LEN: f32 = 0.1;

const LINE_THICKNESS: f32 = 0.02;

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
            triangles: vec![],
            ports: vec![]
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

        let mut top_terminal: Vec<Vector2<f32>> = vec![];

        top_terminal.push(Vector2::new(0.0, MEMRISTOR_HEIGHT / 2.0));
        top_terminal.push(Vector2::new(top_terminal.last().unwrap().x, top_terminal.last().unwrap().y + MEMRISTOR_TERMINAL_LEN));

        let mut bottom_terminal = vec![];

        bottom_terminal.push(Vector2::new(0.0, -MEMRISTOR_HEIGHT / 2.0));
        bottom_terminal.push(Vector2::new(bottom_terminal.last().unwrap().x, bottom_terminal.last().unwrap().y - MEMRISTOR_TERMINAL_LEN));

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
                },
                LinePrimitive {
                    positions: bottom_terminal.clone(),
                    thickness: LINE_THICKNESS,
                    color: 0x000000,
                    line_cap_ty: 0
                },
                LinePrimitive {
                    positions: top_terminal.clone(),
                    thickness: LINE_THICKNESS,
                    color: 0x000000,
                    line_cap_ty: 0
                },
            ],
            rectangles: vec![
                RectanglePrimitive {
                    position: Vector2::new(0.0, -MEMRISTOR_HEIGHT/2.0 + MEMRISTOR_LOWER_RECT_HEIGHT/2.0),
                    size: Vector2::new(MEMRISTOR_WIDTH, MEMRISTOR_LOWER_RECT_HEIGHT),
                    color: 0x000000
                },
            ],
            triangles: vec![],
            ports: vec![
                Port {
                    name: "bottom".to_string(),
                    relative_position: top_terminal[1],
                    ty: PortType::InOut
                },
                Port {
                    name: "top".to_string(),
                    relative_position: bottom_terminal[1],
                    ty: PortType::InOut
                },
            ]
        }

    };

    pub static ref RESISTOR_PRIMITIVES_L0: ComponentTyPrimitives = {

        const RESISTOR_WIDTH: f32 = MEMRISTOR_WIDTH;
        const RESISTOR_HEIGHT: f32 = MEMRISTOR_HEIGHT;
        const RESISTOR_TERMINAL_OFFSET: f32 = 0.012;
        const DIV_LENGTH: f32 = MEMRISTOR_HEIGHT / 12.0;

        let mut central_line: Vec<Vector2<f32>> = vec![];

        central_line.push(Vector2::new(0.0, RESISTOR_HEIGHT / 2.0));
        central_line.push(Vector2::new( RESISTOR_WIDTH / 2.0, central_line.last().unwrap().y - DIV_LENGTH));
        central_line.push(Vector2::new(-RESISTOR_WIDTH / 2.0, central_line.last().unwrap().y - DIV_LENGTH * 2.0));
        central_line.push(Vector2::new( RESISTOR_WIDTH / 2.0, central_line.last().unwrap().y - DIV_LENGTH * 2.0));
        central_line.push(Vector2::new(-RESISTOR_WIDTH / 2.0, central_line.last().unwrap().y - DIV_LENGTH * 2.0));
        central_line.push(Vector2::new( RESISTOR_WIDTH / 2.0, central_line.last().unwrap().y - DIV_LENGTH * 2.0));
        central_line.push(Vector2::new(-RESISTOR_WIDTH / 2.0, central_line.last().unwrap().y - DIV_LENGTH * 2.0));
        central_line.push(Vector2::new(0.0, central_line.last().unwrap().y - DIV_LENGTH));



        let mut top_terminal: Vec<Vector2<f32>> = vec![];

        top_terminal.push(Vector2::new(0.0, RESISTOR_HEIGHT / 2.0 - RESISTOR_TERMINAL_OFFSET));
        top_terminal.push(Vector2::new(top_terminal.last().unwrap().x, top_terminal.last().unwrap().y + MEMRISTOR_TERMINAL_LEN));

        let mut bottom_terminal = vec![];

        bottom_terminal.push(Vector2::new(0.0, -RESISTOR_HEIGHT / 2.0 + RESISTOR_TERMINAL_OFFSET));
        bottom_terminal.push(Vector2::new(bottom_terminal.last().unwrap().x, bottom_terminal.last().unwrap().y - MEMRISTOR_TERMINAL_LEN));

        ComponentTyPrimitives {
            circles: vec![],
            lines: vec![
                // LinePrimitive {
                //     positions: vec![
                //         Vector2::new(0.0, -RESISTOR_HEIGHT / 2.0),
                //         Vector2::new(-0.3, -RESISTOR_HEIGHT / 2.0-0.1),
                //         Vector2::new(-0.3 + 0.3, -RESISTOR_HEIGHT / 2.0-0.2),
                //     ],
                //     thickness: LINE_THICKNESS*1.1,
                //     color: 0x000000,
                //     line_cap_ty: 1
                // },
                // LinePrimitive {
                //     positions: vec![
                //         Vector2::new(0.0, -RESISTOR_HEIGHT / 2.0 + 0.2),
                //         Vector2::new(0.3, -RESISTOR_HEIGHT / 2.0 + 0.3),
                //     ],
                //     thickness: LINE_THICKNESS*1.1,
                //     color: 0x000000,
                //     line_cap_ty: 1
                // },
                LinePrimitive {
                    positions: central_line[0..2].to_vec(),
                    thickness: LINE_THICKNESS*0.05,
                    color: 0x000000,
                    line_cap_ty: 1
                },
                LinePrimitive {
                    positions: central_line[1..3].to_vec(),
                    thickness: LINE_THICKNESS*0.05,
                    color: 0x000000,
                    line_cap_ty: 1
                },
                LinePrimitive {
                    positions: central_line,
                    thickness: LINE_THICKNESS*1.4,
                    color: 0x000000,
                    line_cap_ty: 1
                },
                LinePrimitive {
                    positions: top_terminal.clone(),
                    thickness: LINE_THICKNESS,
                    color: 0x000000,
                    line_cap_ty: 0
                },
                LinePrimitive {
                    positions: bottom_terminal.clone(),
                    thickness: LINE_THICKNESS,
                    color: 0x000000,
                    line_cap_ty: 0
                }
            ],
            rectangles: vec![
            ],
            triangles: vec![],
            ports: vec![
                Port {
                    name: "top".to_string(),
                    relative_position: top_terminal[1],
                    ty: PortType::InOut
                },
                Port {
                    name: "bottom".to_string(),
                    relative_position: bottom_terminal[1],
                    ty: PortType::InOut
                },
            ]
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
            triangles: vec![],
            ports: MEMRISTOR_PRIMITIVES_L0.ports.clone()
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
                    line_cap_ty: 0
                },
                LinePrimitive {
                    positions: plus_terminal.clone(),
                    thickness: LINE_THICKNESS,
                    color: 0x000000,
                    line_cap_ty: 0
                },
                LinePrimitive {
                    positions: minus_terminal.clone(),
                    thickness: LINE_THICKNESS,
                    color: 0x000000,
                    line_cap_ty: 0
                },
                LinePrimitive {
                    positions: out_terminal.clone(),
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
            triangles: vec![],
            ports: vec![
                Port {
                    name: "plus".to_string(),
                    relative_position: plus_terminal[1],
                    ty: PortType::InOut
                },
                Port {
                    name: "minus".to_string(),
                    relative_position: minus_terminal[1],
                    ty: PortType::InOut
                },
                Port {
                    name: "out".to_string(),
                    relative_position: out_terminal[1],
                    ty: PortType::InOut
                }
            ]
        }
    };

    pub static ref NMOS_PRIMITIVES_L0: ComponentTyPrimitives = {

        const NMOS_WIDTH: f32 = 0.7;
        const NMOS_HEIGHT: f32 = 1.0;
        // const TERMINAL_WIDTH: f32 = ;
        const OXIDE_WIDTH: f32 = 0.1;
        const GATE_THICKNESS: f32 = 0.04;

        let gate_terminal = vec![
            Vector2::new(-NMOS_WIDTH / 2.0, 0.0),
            Vector2::new(-NMOS_WIDTH / 2.0 + NMOS_WIDTH / 2.8, 0.0)
        ];

        let drain_terminal = vec![
            Vector2::new(NMOS_WIDTH / 2.0, -NMOS_HEIGHT / 2.0),
            Vector2::new(NMOS_WIDTH / 2.0, -NMOS_HEIGHT / 2.0 + NMOS_HEIGHT / 3.0),
            Vector2::new(gate_terminal.last().unwrap().x + OXIDE_WIDTH, -NMOS_HEIGHT / 2.0 + NMOS_HEIGHT / 3.0),
        ];

        let source_terminal = vec![
            Vector2::new(NMOS_WIDTH / 2.0, NMOS_HEIGHT / 2.0),
            Vector2::new(NMOS_WIDTH / 2.0, NMOS_HEIGHT / 2.0 - NMOS_HEIGHT / 3.0),
            Vector2::new(gate_terminal.last().unwrap().x + OXIDE_WIDTH, NMOS_HEIGHT / 2.0 - NMOS_HEIGHT / 3.0),
        ];

        let mid_horz_terminal = Vector2::new((drain_terminal.last().unwrap().x + NMOS_WIDTH / 2.0) / 2.0, drain_terminal.last().unwrap().y);

        ComponentTyPrimitives {
            circles: vec![],
            lines: vec![
                LinePrimitive {
                    positions: gate_terminal.clone(),
                    thickness: LINE_THICKNESS*0.8,
                    color: 0x000000,
                    line_cap_ty: 0
                },
                LinePrimitive {
                    positions: drain_terminal.clone(),
                    thickness: LINE_THICKNESS*0.8,
                    color: 0x000000,
                    line_cap_ty: 0
                },
                LinePrimitive {
                    positions: source_terminal.clone(),
                    thickness: LINE_THICKNESS*0.8,
                    color: 0x000000,
                    line_cap_ty: 0
                }
            ],
            rectangles: vec![
                RectanglePrimitive {
                    position: gate_terminal.last().unwrap().clone() + Vector2::new(OXIDE_WIDTH + GATE_THICKNESS/2.0, 0.0),
                    size: Vector2::new(GATE_THICKNESS, 0.52 * NMOS_HEIGHT),
                    color: 0x000000
                },
                RectanglePrimitive {
                    position: gate_terminal.last().unwrap().clone() - Vector2::new(- GATE_THICKNESS/2.0, 0.0),
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
            ],
            ports: vec![
                Port {
                    name: "gate".to_string(),
                    relative_position: gate_terminal[0],
                    ty: PortType::InOut
                },
                Port {
                    name: "drain".to_string(),
                    relative_position: drain_terminal[0],
                    ty: PortType::InOut
                },
                Port {
                    name: "source".to_string(),
                    relative_position: source_terminal[0],
                    ty: PortType::InOut
                }
            ]
        }
    };
);
