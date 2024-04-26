use tracing::info;

pub struct FrameCounter {
    // Instant of the last time we printed the frame time.
    last_printed_instant: web_time::Instant,
    // Number of frames since the last time we printed the frame time.
    frame_count: u32,

    frame_time: f32,
}

impl FrameCounter {
    pub fn new() -> Self {
        Self {
            last_printed_instant: web_time::Instant::now(),
            frame_time: f32::MAX,
            frame_count: 0,
        }
    }

    pub fn frame_time(&self) -> f32 {
        self.frame_time
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
        let new_instant = web_time::Instant::now();
        let elapsed_secs = (new_instant - self.last_printed_instant).as_secs_f32();
        if elapsed_secs > 1.0 {
            let elapsed_ms = elapsed_secs * 1000.0;
            let frame_time = elapsed_ms / self.frame_count as f32;

            self.frame_time = frame_time;

            let fps = self.frame_count as f32 / elapsed_secs;
            // info!("Frame time {:.2}ms ({:.1} FPS)", frame_time, fps);

            self.last_printed_instant = new_instant;
            self.frame_count = 0;
        }
    }
}
