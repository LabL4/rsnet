struct AaBb {
    min: vec2<f32>,
    max: vec2<f32>,
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
    aabb: AaBb,
    radius: f32,
};

struct MouseUniform {
    pos: vec2<f32>,
};

struct WindowUniform {
    size: vec2<u32>,
    aspect: f32,
};

@group($bg) @binding(0)
var<uniform> camera: CameraUniform;
@group($bg) @binding(1)
var<uniform> mouse: MouseUniform;
@group($bg) @binding(2)
var<uniform> window: WindowUniform;