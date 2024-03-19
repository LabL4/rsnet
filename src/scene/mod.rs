pub mod component;
pub mod wire;
pub mod utils;
pub mod shared;

use nalgebra::Vector2;
use tracing::info;
use utils::*;
use component::Component;

use rsnet_derive::unwrap_or_return_none;
use crate::{app::utils::chunk_size_from_step_idx, utils::Id};

use std::{collections::HashMap, hash::Hash};

use self::wire::WireSegment;

pub type ChunkedStorage<T> = HashMap<u32, HashMap<ChunkId, Vec<T>>>;

#[derive(Debug)]
pub struct Scene {

    // components: HashMap<ChunkId, Vec<Component>>,
    // components: HashMap<ChunkSize, HashMap<ChunkId, Vec<Component>>>,
    components: ChunkedStorage<Component>,
    id_to_chunksize_chunk: HashMap<Id, (ChunkSize, ChunkId)>,
    wire_segments: ChunkedStorage<WireSegment>,

}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl Scene {
    pub fn new() -> Self {
        let mut scene = Scene {
            components: HashMap::new(),
            id_to_chunksize_chunk: HashMap::new(),
            wire_segments: HashMap::new(),
        };

        let chunk_size = 10;
        let chunk_step_idx = 0;

        // Add 10M components to the scene
        let n_cols = 4;
        let n_rows = 4;

        let n_cols = 1000;
        let n_rows = 1000;
        
        for i in 0..n_rows {
            for j in 0..n_cols {
                let spacing = 2.0;
                let pos = Vector2::new(i as f32, j as f32) * spacing;
                let id = j+1 + i*n_cols;
                // info!("Adding component with id {}", j + i*n_cols);
                scene.add_component(chunk_step_idx, Component::new(id, 0, pos, 0.0  , id%3));
            }
        }


        // scene.add_component(Component::new(0, 0, Vector2::new(1.0,0.0), 0.0, 0));
        // scene.add_component(Component::new(0, 0, Vector2::new(10.0,0.0), 0.0, 0));

        scene
    }

    pub fn add_component(&mut self, chunk_step_idx: u32, component: Component) {
        let chunk_id = chunk_id_from_position(&component.position(), chunk_size_from_step_idx(chunk_step_idx));
        // info!("Adding component to chunk: {:?}", chunk_id);

        self.id_to_chunksize_chunk.insert(component.id(), (chunk_step_idx, chunk_id));

        let chunked_comps = self.components.entry(chunk_step_idx).or_insert(HashMap::new());

        let components = chunked_comps.entry(chunk_id).or_insert(Vec::new());
        match components.binary_search_by_key(&component.id(), |c| c.id()) {
            Ok(pos) => {
                components[pos] = component;
            }
            Err(pos) => components.insert(pos, component),
        }
    }

    pub fn components(&self) -> &ChunkedStorage<Component> {
        &self.components
    }

    pub fn get_component(&self, id: Id) -> Option<&Component> {
        unwrap_or_return_none!(chunk_size_chunk_id, self.id_to_chunksize_chunk.get(&id));
        let (chunk_size, chunk_id) = chunk_size_chunk_id;

        unwrap_or_return_none!(chunked_components, self.components.get(chunk_size));

        unwrap_or_return_none!(components, chunked_components.get(chunk_id));
        
        match components.binary_search_by_key(&id, |c| c.id()) {
            Ok(pos) => Some(&components[pos]),
            Err(_) => None
        }
    }

    pub fn get_components_in_chunk(&self, chunk_id: &ChunkId) -> Option<&Vec<Component>> {
        // self.components.get(chunk_id)
        None
    }

    pub fn wire_segments(&self) -> &ChunkedStorage<WireSegment> {
        &self.wire_segments
    }


    // pub fn compute_id_to_chunk(&mut self, chunk_size: f32) {
    //     self.chunk_size = chunk_size;
    //     let mut id_to_chunk = HashMap::new();
    //     self.components.iter().for_each(|(chunk_id, component)| {
    //         component.iter().for_each(|c| {
    //                 id_to_chunk.insert(c.id(), *chunk_id);
    //         });
    //     });

    //     self.id_to_chunk = id_to_chunk;
    // }
}