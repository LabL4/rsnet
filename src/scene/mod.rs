pub mod component;
pub mod wire;
pub mod utils;
pub mod shared;
pub mod scene_manager;

use nalgebra::Vector2;
use tracing::info;
use utils::*;
use component::Component;

use rsnet_derive::unwrap_or_return_none;
use crate::{app::utils::chunk_size_from_step_idx, renderer::primitives::{common::{MEMRISTOR_PRIMITIVES_L0, MEMRISTOR_PRIMITIVES_L1, NMOS_PRIMITIVES_L0, OMP_AMP_PRIMITIVES_L0}, ComponentTyPrimitives}, utils::Id};

use std::{collections::HashMap, hash::Hash};

use self::{component::ComponentType, wire::{Wire, WireSegment}};

pub type ChunkedStorage<T> = HashMap<u32, HashMap<ChunkId, Vec<T>>>;

#[derive(Debug)]
pub struct Scene {
    // components: HashMap<ChunkId, Vec<Component>>,
    // components: HashMap<ChunkSize, HashMap<ChunkId, Vec<Component>>>,
    components: ChunkedStorage<Component>,
    id_to_chunksize_chunk: HashMap<Id, (ChunkSize, ChunkId)>,
    wire_segments: ChunkedStorage<WireSegment>,
    wires: HashMap<u32, Vec<Wire>>,
    
    primitives: HashMap<ComponentType, Vec<(&'static ComponentTyPrimitives, f32)>>,
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
            wires: HashMap::new(),
            primitives: HashMap::new()
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

        scene.add_wire(chunk_step_idx, Wire::new(0, Vector2::new(0.0, 0.0), Vector2::new(1000.0, 10000.0)));

        scene.primitives.insert(0, vec![
            (&MEMRISTOR_PRIMITIVES_L0, 400.0), (&MEMRISTOR_PRIMITIVES_L1, 1200.0)
        ]);

        scene.primitives.insert(1, vec![(&OMP_AMP_PRIMITIVES_L0, 400.0)]);
        scene.primitives.insert(2, vec![(&NMOS_PRIMITIVES_L0, 400.0)]);

        // scene.add_component(Component::new(0, 0, Vector2::new(1.0,0.0), 0.0, 0));
        // scene.add_component(Component::new(0, 0, Vector2::new(10.0,0.0), 0.0, 0));

        scene
    }

    pub fn primitives(&self) -> &HashMap<ComponentType, Vec<(&'static ComponentTyPrimitives, f32)>> {
        &self.primitives
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

    pub fn add_wire(&mut self, chunk_step_idx: u32, wire: Wire) {

        let wires_vec = self.wires.entry(wire.id()).or_insert(Vec::new());
        let wire_segments = &mut self.wire_segments;

        match wires_vec.binary_search_by_key(&wire.id(), |c| c.id()) {
            Ok(pos) => {
                remove_wire_segments(wire_segments, chunk_step_idx, wire.id());
                add_wire_segments(wire_segments, chunk_step_idx, &wire);
                wires_vec[pos] = wire;
            }
            Err(pos) => {
                add_wire_segments(wire_segments, chunk_step_idx, &wire);
                wires_vec.insert(pos, wire);
            },
        };
    }

}

fn remove_wire_segments(wire_segments: &mut ChunkedStorage<WireSegment>, chunk_step_idx: u32, wire_id: u32) {
    wire_segments.remove(&wire_id);
}

fn add_wire_segments(wire_segments: &mut ChunkedStorage<WireSegment>, chunk_step_idx: u32, wire: &Wire) {
    let this_wire_segments: Vec<(ChunkId, WireSegment)> = wire.to_segments(chunk_size_from_step_idx(chunk_step_idx));
    
    let chunked_wire_segments = wire_segments.entry(wire.id()).or_insert(HashMap::new());
    
    this_wire_segments.into_iter().for_each(|(chunk_id, wire_segment)| {
        let segments = chunked_wire_segments.entry(chunk_id).or_insert(Vec::new());

        info!("Adding wire segment to chunk: {:?}", chunk_id);

        match segments.binary_search_by_key(&wire_segment.id(), |w| w.id()) {
            Ok(pos) => {
                segments[pos] = wire_segment;
            }
            Err(pos) => {
                segments.insert(pos, wire_segment);
            }
        }
    });
}