pub mod pipeline;
pub mod shared;
pub mod utils;
pub mod common; // Common set of primitives (Memristor, Resistor, etc)
pub mod render;

use nalgebra::Vector2;
use tracing::info;

use self::shared::*;

#[derive(Debug)]
pub struct ComponentTyPrimitives {
    pub circles: Vec<CirclePrimitive>,
    pub lines: Vec<LinePrimitive>,
    pub rectangles: Vec<RectanglePrimitive>,
    pub triangles: Vec<TrianglePrimitive>
}

impl ComponentTyPrimitives {
    pub fn to_fragments(&self) -> (
        Vec<CircleFragment>,
        Vec<LineFragment>,
        Vec<RectangleFragment>,
        Vec<TriangleFragment>
    ) {

        let circles = self.circles.iter().flat_map(|circle| circle.to_fragments()).collect();
        let lines = self.lines.iter().flat_map(|line| line.to_fragments()).collect();
        let rectangles = self.rectangles.iter().flat_map(|rectangle| rectangle.to_fragments()).collect();
        let triangles = self.triangles.iter().flat_map(|triangle| triangle.to_fragments()).collect();

        (circles, lines, rectangles, triangles)
    }
}

#[derive(Debug)]
pub struct CirclePrimitive {
    pub position: Vector2<f32>,
    pub radius: f32,
    pub color: u32
}

impl CirclePrimitive {
    pub fn to_fragments(&self) -> Vec<CircleFragment> {
        vec![
            CircleFragment {
                position: self.position,
                radius: self.radius,
                color: self.color
            }
        ]
    }
}

#[derive(Debug)]
pub struct LinePrimitive {
    pub positions: Vec<Vector2<f32>>,
    pub line_cap_ty: u32,
    pub thickness: f32,
    pub color: u32
}

impl LinePrimitive {
    pub fn to_fragments(&self) -> Vec<LineFragment> {
        let mut fragments = Vec::new();

        for i in 0..self.positions.len() - 1 {
            let start = self.positions[i];
            let end = self.positions[i + 1];

            fragments.push(LineFragment {
                start,
                end,
                thickness: self.thickness,
                line_cap_ty: self.line_cap_ty,
                ty: if i == 0 { 1 } else if i == self.positions.len() - 2 { 2 } else { 0 },
                color: self.color
            });
        }

        if self.positions.len() == 2 {
            fragments.get_mut(0).as_mut().unwrap().ty = 3;
        }

        if self.positions.len() == 1 {
            let start = self.positions[self.positions.len() - 1];

            fragments.push(LineFragment {
                start,
                end: start,
                thickness: self.thickness,
                line_cap_ty: self.line_cap_ty,
                ty: 3,
                color: self.color
            });
        }

        // info!("Line fragments: {:?}", fragments);

        fragments
    }
}

#[derive(Debug)]
pub struct RectanglePrimitive {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub color: u32
}

impl RectanglePrimitive {
    pub fn to_fragments(&self) -> Vec<RectangleFragment> {
        vec![
            RectangleFragment {
                position: self.position,
                size: self.size,
                color: self.color
            }
        ]        
    }
}

#[derive(Debug)]
pub struct TrianglePrimitive {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub dir_vec: Vector2<f32>,
    pub color: u32,
}

impl TrianglePrimitive {
    pub fn to_fragments(&self) -> Vec<TriangleFragment> {
        vec![
            TriangleFragment {
                position: self.position,
                size: self.size,
                dir_vec: self.dir_vec,
                color: self.color
            }
        ]        
    }
}