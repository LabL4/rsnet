use std::collections::HashMap;

pub type ChunkId = (i32, i32);

pub type ChunkSize = u32;

/// This is the "LOD", 0 is the highest detail (component level), only shown when zoomed in
pub type ChunkStepIdx = u32;

/// Maps a Chunk step index to a ChunkSize
pub type ChunkedStorage<T> = HashMap<ChunkStepIdx, HashMap<ChunkId, Vec<T>>>;
