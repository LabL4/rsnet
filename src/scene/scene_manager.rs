use egui::ahash::HashMap;

use super::Scene;

pub struct SceneManager {
    last_id: HashMap<u32, u32>, // There can be different IDs in different chunk_steps
    last_wire_id: u32,
    scene: Scene
}

impl SceneManager {
    
}