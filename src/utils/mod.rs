pub mod frame_counter;
pub mod wgpu;

pub use frame_counter::FrameCounter;

use nalgebra::Vector2;
use rayon::prelude::*;
use std::{collections::HashMap, fmt::Debug};

pub type WindowSize = winit::dpi::PhysicalSize<u32>;
pub type Id = u32;

#[derive(Debug, Clone)]
pub struct AaBb {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
}

impl AaBb {
    pub fn inside(&self, container: &AaBb) -> bool {
        container.contains(&self.min) && container.contains(&self.max)
    }

    pub fn contains(&self, point: &Vector2<f32>) -> bool {
        self.min.x <= point.x
            && self.min.y <= point.y
            && self.max.x >= point.x
            && self.max.y >= point.y
    }
}

// Macro to measure time, taking a string for the name
#[macro_export]
macro_rules! timed {
    ($e:expr, $name:expr) => {{
        {
            let start = std::time::Instant::now();
            let result = $e;
            let elapsed = start.elapsed();

            result
        }
        // info!("{} took: {:?} ms", $name, elapsed.as_millis());
    }};
}

pub fn insert_ordered_at<T: Default + Clone + Debug>(
    vec: &mut Vec<T>,
    mut to_insert: HashMap<usize, Vec<T>>,
) {
    if to_insert.is_empty() {
        return;
    }

    let sorted_keys = {
        let mut sk = to_insert.keys().map(|k| *k).collect::<Vec<usize>>();
        sk.par_sort();
        sk
    };

    let move_vec = sorted_keys
        .iter()
        .scan(0, |acc, key| {
            let comps = to_insert.get(key).unwrap();
            let idx = key;
            *acc += comps.len();
            Some((*idx, *acc))
        })
        .collect::<Vec<(usize, usize)>>();

    let n_new = move_vec.last().unwrap_or(&(0, 0)).1;
    let orig_len = vec.len();

    vec.append(&mut vec![T::default(); n_new]);

    let mut move_vec_idx = move_vec.len() - 1;
    for i in (0..orig_len).rev() {
        if i < move_vec[move_vec_idx].0 && move_vec_idx > 0 {
            move_vec_idx = move_vec_idx - 1;
        }

        if i >= move_vec[move_vec_idx].0 {
            vec.swap(i, i + move_vec[move_vec_idx].1);
        }
    }

    let mut total_desp = 0;
    for key in sorted_keys.iter() {
        let idx = *key;
        let new_elements = to_insert.remove(key).unwrap();
        let desp = total_desp;
        for (i, element) in new_elements.into_iter().enumerate() {
            vec[idx + i + desp] = element;
            total_desp += 1;
        }
    }

    // println!("{:?}", vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_ordered_at() {
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut to_insert = HashMap::new();
        to_insert.insert(3, vec![10, 11, 12]);
        to_insert.insert(7, vec![13, 14, 15]);

        insert_ordered_at(&mut vec, to_insert);

        assert_eq!(vec, vec![1, 2, 3, 10, 11, 12, 4, 5, 6, 7, 13, 14, 15, 8, 9]);

        let mut vec = vec![1, 2, 3, 4, 5];
        let mut to_insert = HashMap::new();
        to_insert.insert(0, vec![10, 11, 12]);
        to_insert.insert(5, vec![13, 14, 15]);

        insert_ordered_at(&mut vec, to_insert);

        assert_eq!(vec, vec![10, 11, 12, 1, 2, 3, 4, 5, 13, 14, 15]);
    }

    #[test]
    fn test_merge_sorted_vecs() {
        let v1 = vec![1, 3, 5, 7, 9];
        let v2 = vec![2, 4, 6, 8, 10];

        let result = merge_sorted_vecs(v1, v2);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let v1 = vec![1, 3, 5, 7, 9];
        let v2 = vec![2, 4, 6, 8, 10, 11, 12, 13, 14, 15];

        let result = merge_sorted_vecs(v1, v2);
        assert_eq!(
            result,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );

        let v1 = vec![1, 3, 5, 7, 9, 11, 12, 13, 14, 15];
        let v2 = vec![1, 2, 3, 4, 6, 8, 10, 19];

        let result = merge_sorted_vecs(v1, v2);
        assert_eq!(
            result,
            vec![1, 1, 2, 3, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 19]
        );
    }

    #[test]
    fn test_retain_by_range() {
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        retain_by_range(&mut vec, (2, 6), false);
        assert_eq!(vec, vec![3, 4, 5, 6, 7]);

        let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        retain_by_range(&mut vec, (0, 8), false);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}

pub fn merge_sorted_vecs<T: PartialOrd + Clone>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
    let mut p1 = 0;
    let mut p2 = 0;

    let mut result = Vec::with_capacity(v1.len() + v2.len());

    while result.len() < v1.len() + v2.len() {
        if p1 < v1.len() && p2 < v2.len() {
            if v1[p1] < v2[p2] {
                result.push(v1[p1].clone());
                p1 += 1;
            } else {
                result.push(v2[p2].clone());
                p2 += 1;
            }
        } else if p1 < v1.len() {
            result.push(v1[p1].clone());
            p1 += 1;
        } else if p2 < v2.len() {
            result.push(v2[p2].clone());
            p2 += 1;
        }
    }

    result
}

/// Retains elements in a vector that are within a given range, inclusive.\
/// range: [min, max)
/// invert: if true, retains elements outside the range
pub fn retain_by_range<T>(vec: &mut Vec<T>, range: (usize, usize), invert: bool) {
    let mut idx = 0;
    vec.retain(|_el| {
        let keep = idx >= range.0 && idx < range.1;
        idx += 1;
        if invert {
            !keep
        } else {
            keep
        }
    });
}
