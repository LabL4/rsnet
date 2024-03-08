pub mod frame_counter;
pub mod wgpu;

use nalgebra::Vector2;
use rayon::prelude::*;
use std::collections::HashMap;

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
        let start = std::time::Instant::now();
        let result = $e;
        let elapsed = start.elapsed();
        info!("{} took: {:?} ms", $name, elapsed.as_millis());
        result
    }};
}

pub fn insert_ordered_at<T: Default + Clone>(
    vec: &mut Vec<T>,
    mut to_insert: HashMap<usize, Vec<T>>,
) {
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

    println!("{:?}", move_vec);

    let n_new = move_vec.last().unwrap_or(&(0, 0)).1;
    let orig_len = vec.len();

    vec.append(&mut vec![T::default(); n_new]);

    println!("{:?}", vec.len());

    let mut move_vec_idx = move_vec.len() - 1;
    for i in (0..orig_len).rev() {
        if (i < move_vec[move_vec_idx].0 && move_vec_idx > 0) {
            move_vec_idx = move_vec_idx - 1;
        }

        if i >= move_vec[move_vec_idx].0 {
            vec.swap(i, i + move_vec[move_vec_idx].1);
        }
    }

    let mut move_vec_idx = 0;
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
}