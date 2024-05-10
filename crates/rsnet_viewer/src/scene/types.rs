use std::collections::HashMap;

use crate::renderer::primitives::ComponentTyPrimitives;


pub type ComponentType = u32;

pub type ChunkId = (i32, i32);

pub type ChunkSize = u32;

/// This is the "LOD", 0 is the highest detail (component level), only shown when zoomed in
pub type ChunkStepIdx = u32;

/// Maps a Chunk step index to a ChunkSize
pub type ChunkedStorage<T> = HashMap<ChunkStepIdx, HashMap<ChunkId, Vec<T>>>;

// Primitives
#[derive(Debug)]
pub struct Primitives(pub HashMap<ComponentType, Vec<(&'static ComponentTyPrimitives, f32)>>);
