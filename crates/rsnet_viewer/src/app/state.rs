use crate::{
    gui,
    scene::{self, utils::ChunkRange},
    utils::FrameCounter,
};

use smaa::SmaaMode;

pub struct State {
    pub scene: scene::Scene,

    grid: bool, // If the grid is visible
    current_frame_time: f32,
    msaa_count: u32,
    rebuild_bundles: bool, // Controls whether to rebuild the render pipelines and texture views
    smaa: SmaaMode,
    rebuild_smaa: bool,
    n_primitives_in_fragment_storage: usize,
    n_wires_in_buffer: usize,
    n_components_in_buffer: usize,
    chunk_step_idx: usize,
    chunk_size: f32,
    screen_chunk_range: ChunkRange,
}

impl Default for State {
    fn default() -> Self {
        Self {
            scene: scene::Scene::default(),
            grid: false,
            current_frame_time: f32::MAX,
            msaa_count: 8,
            rebuild_bundles: true,
            smaa: SmaaMode::Smaa1X,
            rebuild_smaa: true,
            n_primitives_in_fragment_storage: 0,
            n_wires_in_buffer: 0,
            n_components_in_buffer: 0,
            chunk_step_idx: 0,
            chunk_size: 0.0,
            screen_chunk_range: ChunkRange::default(),
        }
    }
}

impl State {

    pub fn grid(&self) -> bool {
        self.grid
    }

    pub fn set_grid(&mut self, grid: bool) {
        self.grid = grid;
    }

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

    pub fn n_wires_in_buffer(&self) -> usize {
        self.n_wires_in_buffer
    }

    pub fn set_n_wires_in_buffer(&mut self, n: usize) {
        self.n_wires_in_buffer = n;
    }

    pub fn n_components_in_buffer(&self) -> usize {
        self.n_components_in_buffer
    }

    pub fn set_n_components_in_buffer(&mut self, n: usize) {
        self.n_components_in_buffer = n;
    }

    pub fn chunk_step_idx(&self) -> usize {
        self.chunk_step_idx
    }

    pub fn set_chunk_step_idx(&mut self, idx: usize) {
        self.chunk_step_idx = idx;
    }

    pub fn chunk_size(&self) -> f32 {
        self.chunk_size
    }

    pub fn set_chunk_size(&mut self, size: f32) {
        self.chunk_size = size;
    }

    pub fn screen_chunk_range(&self) -> &ChunkRange {
        &self.screen_chunk_range
    }

    pub fn set_screen_chunk_range(&mut self, range: ChunkRange) {
        self.screen_chunk_range = range;
    }
}
