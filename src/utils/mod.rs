pub mod wgpu;
pub mod frame_counter;

use nalgebra::Vector2;

pub type WindowSize = winit::dpi::PhysicalSize<u32>;
pub type Id = u32;

#[derive(Debug, Clone)]
pub struct AaBb {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>
}

impl AaBb {
    pub fn inside(&self, container: &AaBb) -> bool {
        container.contains(&self.min) && container.contains(&self.max)
    }

    pub fn contains(&self, point: &Vector2<f32>) -> bool {
        self.min.x <= point.x
            && self.min.y <= point.y
            && self.max.x >= point.x
            && self.max.y >= point.y
    }
}

// Macro to measure time, taking a string for the name
#[macro_export]
macro_rules! timed {
    ($e:expr, $name:expr) => {{
        let start = std::time::Instant::now();
        let result = $e;
        let elapsed = start.elapsed();
        info!("{} took: {:?} ms", $name, elapsed.as_millis());
        result
    }};
}
