use crate::{gui, scene, utils::FrameCounter};

use smaa::SmaaMode;

pub struct State {
    pub scene: scene::Scene,

    current_frame_time: f32,
    msaa_count: u32,
    rebuild_bundles: bool, // Controls whether to rebuild the render pipelines and texture views
    smaa: SmaaMode,
    rebuild_smaa: bool,
    n_primitives_in_fragment_storage: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            scene: scene::Scene::default(),
            current_frame_time: f32::MAX,
            msaa_count: 4,
            rebuild_bundles: true,
            smaa: SmaaMode::Smaa1X,
            rebuild_smaa: true,
            n_primitives_in_fragment_storage: 0,
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

    pub fn rebuild_bundles(&self) -> bool {
        self.rebuild_bundles
    }

    pub fn set_rebuild_bundles(&mut self, rebuild: bool) {
        self.rebuild_bundles = rebuild;
    }

    pub fn msaa_count(&self) -> u32 {
        self.msaa_count
    }

    pub fn set_msaa_count(&mut self, count: u32) {
        self.msaa_count = count;
        self.rebuild_bundles = true;
    }

    pub fn smaa_mode(&self) -> SmaaMode {
        self.smaa
    }

    pub fn set_smaa_mode(&mut self, smaa: SmaaMode) {
        if self.smaa != smaa {
            self.smaa = smaa;
            self.rebuild_smaa = true;
        }
    }

    pub fn rebuild_smaa(&self) -> bool {
        self.rebuild_smaa
    }

    pub fn set_rebuild_smaa(&mut self, rebuild: bool) {
        self.rebuild_smaa = rebuild
    }

    pub fn n_primitives_in_fragment_storage(&self) -> usize {
        self.n_primitives_in_fragment_storage
    }

    pub fn set_n_primitives_in_fragment_storage(&mut self, n: usize) {
        self.n_primitives_in_fragment_storage = n;
    }
}
