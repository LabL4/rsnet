use super::component;
use super::types;
use super::utils;
use super::wire;

use component::{Component, ComponentType};
use nalgebra::Vector2;
use tracing::info;
use types::*;
use utils::*;
use wire::{Wire, WireSegment};

use crate::{
    app::utils::chunk_size_from_step_idx,
    renderer::primitives::{
        common::{
            MEMRISTOR_PRIMITIVES_L0, MEMRISTOR_PRIMITIVES_L1, NMOS_PRIMITIVES_L0,
            OMP_AMP_PRIMITIVES_L0, RESISTOR_PRIMITIVES_L0,
        },
        ComponentTyPrimitives,
    },
    utils::Id,
};
use rsnet_derive::unwrap_option_or_return_none;

use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct Scene {
    // components: HashMap<ChunkId, Vec<Component>>,
    // components: HashMap<ChunkSize, HashMap<ChunkId, Vec<Component>>>,
    components: ChunkedStorage<Component>,
    id_to_chunksize_chunk: HashMap<Id, (ChunkSize, ChunkId)>,
    /// Stores the IDs of the wires contained in each chunk
    wires_chunk_cache: ChunkedStorage<Id>,
    wires: HashMap<Id, Wire>,

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
            wires_chunk_cache: HashMap::new(),
            wires: HashMap::new(),
            primitives: HashMap::new(),
        };

        let chunk_size = 10;
        let chunk_step_idx = 0;

        // Add 10M components to the scene
        let n_cols = 2;
        let n_rows = 1;

        let n_cols = 1000;
        let n_rows = 1000;

        for i in 0..n_rows {
            for j in 0..n_cols {
                let spacing = 2.0;
                let pos = Vector2::new(j as f32, i as f32) * spacing;
                let id = j + 1 + i * n_cols;
                // info!("Adding component with id {}", j + i*n_cols);
                // scene.add_component(chunk_step_idx, Component::new(id, 0, pos, 0.0  , id%3));
                // scene.add_component(chunk_step_idx, Component::new(id, 0, pos, 0.0, id % 3));
                scene.add_component(
                    chunk_step_idx,
                    Component::new(id, 0, pos, 0.0, (id - 1) % 4),
                    //Component::new(id, 0, pos, 0.0, 3),
                );
            }
        }

        // println!("components: {:#?}", scene.components);

        let chunk_size = chunk_size_from_step_idx(chunk_step_idx);

        scene.add_wire(
            chunk_step_idx,
            Wire::new(
                0,
                Vector2::new(0.0, 0.0),
                Vector2::new(10.0, 10.0),
                Vector2::new(1.0, 0.0),
                Vector2::new(1.0, 1.0),
                chunk_size,
            ),
        );

        scene.add_wire(
            chunk_step_idx,
            Wire::new(
                1,
                Vector2::new(10.0, 10.0),
                Vector2::new(20.0, 10.0),
                Vector2::new(1.0, 0.0),
                Vector2::new(1.0, 1.0),
                chunk_size,
            ),
        );

        scene.primitives.insert(
            0,
            vec![
                (&MEMRISTOR_PRIMITIVES_L0, 400.0),
                (&MEMRISTOR_PRIMITIVES_L1, 1200.0),
            ],
        );

        scene
            .primitives
            .insert(1, vec![(&OMP_AMP_PRIMITIVES_L0, 400.0)]);
        scene
            .primitives
            .insert(2, vec![(&NMOS_PRIMITIVES_L0, 400.0)]);

        scene
            .primitives
            .insert(3, vec![(&RESISTOR_PRIMITIVES_L0, 400.0)]);

        // scene.add_component(Component::new(0, 0, Vector2::new(1.0,0.0), 0.0, 0));
        // scene.add_component(Component::new(0, 0, Vector2::new(10.0,0.0), 0.0, 0));

        scene
    }

    pub fn primitives(
        &self,
    ) -> &HashMap<ComponentType, Vec<(&'static ComponentTyPrimitives, f32)>> {
        &self.primitives
    }

    pub fn add_component(&mut self, chunk_step_idx: u32, component: Component) {
        let chunk_id = chunk_id_from_position(
            &component.position(),
            chunk_size_from_step_idx(chunk_step_idx + 1),
        );
        // println!("Adding component to chunk (size: {:?}): {:?}", chunk_size_from_step_idx(chunk_step_idx+1), chunk_id);

        self.id_to_chunksize_chunk
            .insert(component.id(), (chunk_step_idx, chunk_id));

        let chunked_comps = self
            .components
            .entry(chunk_step_idx)
            .or_insert(HashMap::new());

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
        let chunk_size_chunk_id =
            unwrap_option_or_return_none!(self.id_to_chunksize_chunk.get(&id));
        let (chunk_size, chunk_id) = chunk_size_chunk_id;

        let chunked_components = unwrap_option_or_return_none!(self.components.get(chunk_size));

        let components = unwrap_option_or_return_none!(chunked_components.get(chunk_id));

        match components.binary_search_by_key(&id, |c| c.id()) {
            Ok(pos) => Some(&components[pos]),
            Err(_) => None,
        }
    }

    pub fn get_components_in_chunk(&self, chunk_id: &ChunkId) -> Option<&Vec<Component>> {
        // self.components.get(chunk_id)
        None
    }

    pub fn wire_segments(&self) -> &ChunkedStorage<Id> {
        &self.wires_chunk_cache
    }

    pub fn wires(&self) -> &HashMap<Id, Wire> {
        &self.wires
    }

    pub fn add_wire(&mut self, chunk_step_idx: u32, wire: Wire) {
        let wire_chunk_cache = &mut self.wires_chunk_cache;

        if self.wires.contains_key(&wire.id()) {
            // Remove the wire from the cache
            remove_wire_from_chunks_cache(wire_chunk_cache, chunk_step_idx, wire.id());
        }
        add_wire_to_chunk_cache(wire_chunk_cache, chunk_step_idx, &wire);
        self.wires.insert(wire.id(), wire);
    }
}

fn remove_wire_from_chunks_cache(
    wire_chunk_cache: &mut ChunkedStorage<Id>,
    chunk_step_idx: ChunkStepIdx,
    wire_id: Id,
) {
    if let Some(chunkid_wireids_map) = wire_chunk_cache.get_mut(&chunk_step_idx) {
        chunkid_wireids_map
            .iter_mut()
            .for_each(|(_chunk_id, wire_ids)| {
                wire_ids
                    .binary_search(&wire_id)
                    .map(|pos| wire_ids.remove(pos));
            });
    }
}

fn add_wire_to_chunk_cache(
    wire_chunk_cache: &mut ChunkedStorage<Id>,
    chunk_step_idx: ChunkStepIdx,
    wire: &Wire,
) {
    let occupied_chunkids: Vec<ChunkId> =
        wire.occupied_chunks(chunk_size_from_step_idx(chunk_step_idx));

    let chunked_wire_ids = wire_chunk_cache
        .entry(chunk_step_idx)
        .or_insert(HashMap::new());

    occupied_chunkids.into_iter().for_each(|chunk_id| {
        let ids = chunked_wire_ids.entry(chunk_id).or_insert(Vec::new());

        // info!("Adding wire segment to chunk: {:?}", chunk_id);

        match ids.binary_search(&wire.id()) {
            Ok(pos) => {
                ids[pos] = wire.id();
            }
            Err(pos) => {
                ids.insert(pos, wire.id());
            }
        }
    });
}
