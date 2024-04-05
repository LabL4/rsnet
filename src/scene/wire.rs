use nalgebra::{ComplexField, Vector2};

use crate::{renderer::effects::grid, utils::merge_sorted_vecs};

use super::{ChunkId, FromPosition};

#[derive(Debug)]
pub struct WireSegment {
    id: u32,
    wire_id: u32,
    start: Vector2<f32>,
    end: Vector2<f32>,
}

impl WireSegment {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn wire_id(&self) -> u32 {
        self.wire_id
    }

    pub fn start(&self) -> &Vector2<f32> {
        &self.start
    }

    pub fn end(&self) -> &Vector2<f32> {
        &self.end
    }
}

#[derive(Debug)]
pub struct Wire {
    id: u32,
    start: Vector2<f32>,
    end: Vector2<f32>,
}

impl Wire {

    pub fn new(id: u32, start: Vector2<f32>, end: Vector2<f32>) -> Self {
        Wire { id, start, end }
    }

    pub fn to_segments(&self, chunk_size: f32) -> Vec<(ChunkId, WireSegment)> {
        let (y0, y1) = (self.start.y, self.end.y);
        let (x0, x1) = (self.start.x, self.end.x);

        // Vertical crossings
        let horz_ts = calc_crossings_t(x0, x1, chunk_size);

        // Horizontal crossings
        let vert_ts = calc_crossings_t(y0, y1, chunk_size);

        let mut ts_vec = merge_sorted_vecs(horz_ts, vert_ts);

        ts_vec.insert(0, 0.0);
        ts_vec.push(1.0);

        let mut segments = Vec::with_capacity(ts_vec.len() - 1);

        for i in 0..ts_vec.len() - 1 {
            let t0 = ts_vec[i];
            let t1 = ts_vec[i + 1];

            let (x_start, y_start) = eval_line_eq(t0, x0, x1, y0, y1);
            let (x_end, y_end) = eval_line_eq(t1, x0, x1, y0, y1);

            let start = Vector2::new(x_start, y_start);
            let end = Vector2::new(x_end, y_end);
            segments.push((
                ChunkId::from_position(&((start + end) / 2.0), chunk_size),
                WireSegment {
                    id: i as u32,
                    wire_id: self.id,
                    start,
                    end,
                },
            ));
        }

        segments
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn start(&self) -> &Vector2<f32> {
        &self.start
    }

    pub fn end(&self) -> &Vector2<f32> {
        &self.end
    }
}

fn eval_line_eq(t: f32, x0: f32, x1: f32, y0: f32, y1: f32) -> (f32, f32) {
    (x0 + (x1 - x0) * t, y0 + (y1 - y0) * t)
}

#[inline]
fn adjust_to_grid(x: f32, grid_size: f32) -> f32 {
    (x + grid_size / 2.0) / grid_size
}

fn calc_crossings_t(x0: f32, x1: f32, grid_size: f32) -> Vec<f32> {
    let range = if x1 > x0 {
        adjust_to_grid(x0, grid_size).ceil() as u32..=adjust_to_grid(x1, grid_size).floor() as u32
    } else {
        adjust_to_grid(x1, grid_size).ceil() as u32..=adjust_to_grid(x0, grid_size).floor() as u32
    };

    let n = range.end() - range.start() + 1;

    let mut ts = Vec::<f32>::with_capacity(n as usize);
    for n in range {
        let t = (n as f32 * grid_size - grid_size / 2.0 - x0) / (x1 - x0);
        ts.push(t);
    }

    ts
}
