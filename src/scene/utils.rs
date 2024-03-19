use nalgebra::Vector2;

pub type ChunkId = (i32, i32);
pub type ChunkSize = u32;

#[derive(Debug, Default, Clone)]
pub struct ChunkRange {
    pub min_chunk: ChunkId,
    pub max_chunk: ChunkId
}

// Implement PartialEq for ChunkRange
impl PartialEq for ChunkRange {
    fn eq(&self, other: &Self) -> bool {
        self.min_chunk == other.min_chunk && self.max_chunk == other.max_chunk
    }
}

impl ChunkRange {
    pub fn contains(&self, chunk: &ChunkId) -> bool {
        let min_chunk = &self.min_chunk;
        let max_chunk = &self.max_chunk;

        chunk.0 >= min_chunk.0 && chunk.0 <= max_chunk.0 && chunk.1 >= min_chunk.1 && chunk.1 <= max_chunk.1
        
    }

    
    pub fn overlaps(&self, other: &ChunkRange) -> bool {
        overlap_1d(self.min_chunk.0, self.max_chunk.0, other.min_chunk.0, other.max_chunk.0) &&
        overlap_1d(self.min_chunk.1, self.max_chunk.1, other.min_chunk.1, other.max_chunk.1)
    }
    
    /// Returns the chunks that are in `self` but not in `other`
    /// Returns the chunks that are in `other` but not in `self`
    pub fn diff(&self, other: &ChunkRange) -> (Vec<ChunkId>, Vec<ChunkId>) {
        let mut in_self_not_other = Vec::new();
        let mut in_other_not_self = Vec::new();
        
        if !self.overlaps(other) {
            return (iter_chunks_in_range(&self.min_chunk, &self.max_chunk).collect(), iter_chunks_in_range(&other.min_chunk, &other.max_chunk).collect());
        }

        for chunk in iter_chunks_in_range(&self.min_chunk, &self.max_chunk) {
            if !other.contains(&chunk) {
                in_self_not_other.push(chunk);
            }
        }

        for chunk in iter_chunks_in_range(&other.min_chunk, &other.max_chunk) {
            if !self.contains(&chunk) {
                in_other_not_self.push(chunk);
            }
        }
        
        (in_self_not_other, in_other_not_self)
    }
}

pub struct ChunkRangeIterator {
    min_chunk: ChunkId,
    max_chunk: ChunkId,
    current_chunk: ChunkId
}

impl IntoIterator for ChunkRange {
    type Item = ChunkId;
    type IntoIter = ChunkRangeIterator;

    fn into_iter(self) -> Self::IntoIter {
        ChunkRangeIterator {
            min_chunk: self.min_chunk,
            max_chunk: self.max_chunk,
            current_chunk: self.min_chunk
        }
    }
}

impl Iterator for ChunkRangeIterator {
    type Item = ChunkId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_chunk.1 > self.max_chunk.1 {
            return None;
        }

        let current_chunk = self.current_chunk;
        self.current_chunk.0 += 1;

        if self.current_chunk.0 > self.max_chunk.0 {
            self.current_chunk.1 += 1;
            self.current_chunk.0 = self.min_chunk.0;
        }

        Some(current_chunk)
    }
}

fn overlap_1d(min1: i32, max1: i32, min2: i32, max2: i32) -> bool {
    min1 <= max2 && max1 >= min2
}

/// Chunk size is halved because the chunk is centered around the origin
pub fn chunk_id_from_position(position: &Vector2<f32>, chunk_size: f32) -> ChunkId {
    let chunk_x = ((position.x + chunk_size / 2.0) / chunk_size).floor() as i32;
    let chunk_y = ((position.y + chunk_size / 2.0) / chunk_size).floor() as i32;

    (chunk_x, chunk_y)
}

pub fn iter_chunks_in_range<'a>(min_chunk: &'a ChunkId, max_chunk: &'a ChunkId) -> impl Iterator<Item = ChunkId> {
    let min_chunk_x = min_chunk.0;
    let max_chunk_x = max_chunk.0;
    
    let min_chunk_y = min_chunk.1;
    let max_chunk_y = max_chunk.1;
    
    (min_chunk_x..=max_chunk_x).flat_map({
        move |x| (min_chunk_y..=max_chunk_y).map(move |y| (x, y))
    })
}

/// Tests
#[cfg(test)]
mod scene_utils_test {
    use super::ChunkRange;

    #[test]
    fn test_chunk_range_1() {
        let r1 = ChunkRange {
            min_chunk: (-2, -2),
            max_chunk: (0, 0)
        };

        let r2 = ChunkRange {
            min_chunk: (-1, -1),
            max_chunk: (1, 1)
        };

        let (in_self_not_other, in_other_not_self) = r1.diff(&r2);

        println!("{:?}", in_self_not_other);

        let in_self_not_other_target = vec![
            (-2, 0),
            (-2, -1),
            (-2, -2),
            (-1, -2),
            (0, -2)
        ];

        let in_other_not_self_target = vec![
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1)
        ];

        in_self_not_other.iter().for_each(|chunk_id| {
            assert!(in_self_not_other_target.contains(chunk_id));
        });

        in_other_not_self.iter().for_each(|chunk_id| {
            assert!(in_other_not_self_target.contains(chunk_id));
        });

    }

    #[test]
    fn test_chunk_range_2() {
        let r1 = ChunkRange {
            min_chunk: (-3, -1),
            max_chunk: (-2, 0)
        };

        let r2 = ChunkRange {
            min_chunk: (-1, -1),
            max_chunk: (1, 1)
        };

        let (in_self_not_other, in_other_not_self) = r1.diff(&r2);

        assert_eq!(in_self_not_other, vec![
            (-3, -1),
            (-3, 0),
            (-2, -1),
            (-2, 0)
        ]);
        assert_eq!(in_other_not_self, vec![
            (-1, -1),
            (-1, -0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1)
        ]);

    }

    #[test]
    fn test_chunk_range_3() {
        let r1 = ChunkRange {
            min_chunk: (0, 0),
            max_chunk: (2, 2)
        };

        let r2 = ChunkRange {
            min_chunk: (-1, -1),
            max_chunk: (1, 1)
        };

        let (in_self_not_other, in_other_not_self) = r1.diff(&r2);

        let in_self_not_other_target = vec![
            (0, 2),
            (1, 2),
            (2, 2),
            (2, 1),
            (2, 0)
        ];

        println!("in_self_not_other: \n{:?}", in_self_not_other);

        assert_eq!(in_self_not_other.len(), in_self_not_other_target.len());
        in_self_not_other.iter().for_each(|chunk_id| {
            assert!(in_self_not_other_target.contains(chunk_id));
        });

        let in_other_not_self_target = vec![
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1)
        ];

        assert_eq!(in_other_not_self.len(), in_other_not_self_target.len());
        in_other_not_self.iter().for_each(|chunk_id| {
            assert!(in_other_not_self_target.contains(chunk_id));
        });

    }
}

// pub fn range_diff(range1: ChunkRange, range2: ChunkRange) -> Vec<ChunkId> {
//     let diff = V


// }