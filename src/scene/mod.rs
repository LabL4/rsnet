pub mod component;
pub mod utils;
pub mod shared;

use nalgebra::Vector2;
use utils::*;
use component::Component;

use crate::utils::Id;

use std::collections::HashMap;


#[derive(Debug)]
pub struct Scene {
        
    components: HashMap<ChunkId, Vec<Component>>,
    id_to_chunk: HashMap<Id, ChunkId>,

    chunk_size: f32
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
            id_to_chunk: HashMap::new(),
            chunk_size: 10.0
        };

        // Add 10M components to the scene
        let n_cols = 1000;
        let n_rows = 1000;

        // let n_cols = 1000;
        // let n_rows = 1000;
        
        for i in 0..n_rows {
            for j in 0..n_cols {
                let spacing = 2.0;
                let pos = Vector2::new(i as f32, j as f32) * spacing;
                let id = j+1 + i*n_cols;
                // info!("Adding component with id {}", j + i*n_cols);
                scene.add_component(Component::new(id, 0, pos, (id % 3) as f32  , id%2));
            }
        } 

        // scene.add_component(Component::new(0, 0, Vector2::new(1.0,0.0), 0.0, 0));
        // scene.add_component(Component::new(0, 0, Vector2::new(10.0,0.0), 0.0, 0));

        scene
    }

    pub fn add_component(&mut self, component: Component) {
        let chunk_id = chunk_id_from_position(&component.position(), self.chunk_size);
        // info!("Adding component to chunk: {:?}", chunk_id);

        self.id_to_chunk.insert(component.id(), chunk_id);

        let components = self.components.entry(chunk_id).or_insert(Vec::new());
        match components.binary_search_by_key(&component.id(), |c| c.id()) {
            Ok(pos) => {
                components[pos] = component;
            }
            Err(pos) => components.insert(pos, component),
        }
    }

    pub fn components(&self) -> &HashMap<ChunkId, Vec<Component>>{
        &self.components
    }

    pub fn get_component(&self, id: Id) -> Option<&Component> {
        let chunk_id = self.id_to_chunk.get(&id)?;
        let components = self.components.get(chunk_id)?;
        
        match components.binary_search_by_key(&id, |c| c.id()) {
            Ok(pos) => Some(&components[pos]),
            Err(_) => None
        }
    }

    pub fn get_components_in_chunk(&self, chunk_id: &ChunkId) -> Option<&Vec<Component>> {
        self.components.get(chunk_id)
    }

    pub fn chunk_size(&self) -> f32 {
        self.chunk_size
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