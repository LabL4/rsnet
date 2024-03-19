struct AaBb {
    min: vec2<f32>,
    max: vec2<f32>,
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
    aabb: AaBb,
    radius: f32,
    // _pad: vec3<f32>,
};

struct MouseUniform {
    pos: vec2<f32>,
//    _pad: vec2<f32>,
};

struct WindowUniform {
    size: vec2<u32>,
    aspect: f32,
}


struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) world_coord: vec2<f32>,
    @location(2) ss_coords: vec2<f32>,
}

struct TimeData {
    time: u32,
}

struct ChunkData {
    chunk_size: f32,
    prev_chunk_size: f32,
    last_chunk_size_update: u32
}

struct Component {
    model: mat3x3<f32>,
    id: u32,
    ty: u32,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;
@group(0) @binding(1)
var<uniform> mouse: MouseUniform;
@group(0) @binding(2)
var<uniform> window: WindowUniform;

@group(1) @binding(0)
var<storage, read> components: array<Component>;

const ANIM_DURATION: u32 = 180u; // ms

// p1    p3
//  *    *
//  *    *
// p0   p2

@vertex
fn vs_main(@builtin(vertex_index) vertex_idx: u32, @builtin(instance_index) instance_idx: u32) -> VertexOutput {

    var output: VertexOutput;
    
    return output;
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@fragment
fn fs_main(input: VertexOutput) -> FragmentOutput {
    var output: FragmentOutput;
    
    output.color = vec4(1.0, 1.0, 0.0, 1.0);

    return output;
}