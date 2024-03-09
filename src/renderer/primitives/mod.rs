pub mod pipeline;
pub mod shared;
pub mod utils;
pub mod common; // Common set of primitives (Memristor, Resistor, etc)

use nalgebra::Vector2;

use self::shared::{CircleFragment, Fragments, LineFragment, RectangleFragment, MAX_FRAGMENTS};

#[derive(Debug)]
pub struct Primitives {
    pub circles: Vec<CirclePrimitive>,
    pub lines: Vec<LinePrimitive>,
    pub rectangles: Vec<RectanglePrimitive>
}

impl Primitives {
    pub fn to_fragments(&self) -> Fragments {
        let mut circles = [CircleFragment::default(); MAX_FRAGMENTS];
        let mut n_circles = 0;
    
        let mut lines = [LineFragment::default(); MAX_FRAGMENTS];
        let mut n_lines = 0;
    
        let mut rectangles = [RectangleFragment::default(); MAX_FRAGMENTS];
        let mut n_rectangles = 0;
    
        for circle in &self.circles {
            let circle_fragments = circle.to_fragments();
            for i in 0..circle_fragments.len() {
                circles[n_circles] = circle_fragments[i];
                n_circles += 1;
            }
        }
    
        for line in &self.lines {
            let line_fragments = line.to_fragments();
            for i in 0..line_fragments.len() {
                lines[n_lines] = line_fragments[i];
                n_lines += 1;
            }
        }

        // info!("Line fragments!!!: {:?}", lines[0..n_lines].to_vec());
    
        for rectangle in &self.rectangles {
            let rectangle_fragments = rectangle.to_fragments();
            for i in 0..rectangle_fragments.len() {
                rectangles[n_rectangles] = rectangle_fragments[i];
                n_rectangles += 1;
            }
        }
    
        Fragments {
            circles: circles,
            n_circles: n_circles as u32,
    
            lines: lines,
            n_lines: n_lines as u32,
    
            rectangles: rectangles,
            n_rectangles: n_rectangles as u32,
        }
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
                ty: if i == 0 { 1 } else if i == self.positions.len() - 2 { 2 } else { 0 },
                color: self.color
            });
        }

        if self.positions.len() == 1 {
            let start = self.positions[self.positions.len() - 1];

            fragments.push(LineFragment {
                start,
                end: start,
                thickness: self.thickness,
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