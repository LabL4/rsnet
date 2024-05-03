//!include common.inc $bg=0

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    // @location(1) world_coord: vec2<f32>,
    // @location(2) ss_coords: vec2<f32>,
}


struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vs_onscreen(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.clip_pos = vec4<f32>(model.position, 0.0, 1.0);
    out.uv = model.uv;

    return out;
}

@fragment
fn fs_onscreen(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    let scaled_input = textureSample(t_diffuse, s_diffuse, in.uv) * 255.0;
    let mult = f32(u32(round(scaled_input.x)) % 2u);

    return color * mult;
}
