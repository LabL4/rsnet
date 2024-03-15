use crate::{gui, scene, utils::FrameCounter};

pub struct State {
    pub scene: scene::Scene,

    current_frame_time: f32,
    msaa_count: u32,
    rebuild_bundles: bool, // Controls whether to rebuild the render pipelines and texture views   
}

impl Default for State {
    fn default() -> Self {
        Self {
            scene: scene::Scene::default(),
            current_frame_time: f32::MAX,
            msaa_count: 1,
            rebuild_bundles: false,
        }
    }
}

impl State {

    pub fn current_frame_time(&self) -> f32 {
        self.current_frame_time
    }

    pub fn set_current_frame_time(&mut self, time: f32) {
        self.current_frame_time = time;
    }

    pub fn msaa_count(&self) -> u32 {
        self.msaa_count
    }

    pub fn set_msaa_count(&mut self, count: u32) {
        self.msaa_count = count;
        self.rebuild_bundles = true;
    }
    
    pub fn rebuild_bundles(&self) -> bool {
        self.rebuild_bundles
    }

    pub fn set_rebuild_bundles(&mut self, rebuild: bool) {
        self.rebuild_bundles = rebuild;
    }
}