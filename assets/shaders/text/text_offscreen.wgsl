//!include common.inc $bg=0

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) bary: vec2<f32>,
    // @location(1) world_coord: vec2<f32>,
    // @location(2) ss_coords: vec2<f32>,
}

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) bary: vec2<f32>,
};


@vertex
fn vs_offscreen(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;


    out.clip_pos = camera.view_proj * vec4<f32>(model.position, 0.0, 1.0);
    out.bary = model.bary;

    return out;
}

@fragment
fn fs_offscreen(in_frag: VertexOutput) -> @location(0) vec4<f32> {
    if (in_frag.bary.x * in_frag.bary.x - in_frag.bary.y > 0.0) {
        discard;
    }

    let color = vec4<f32>(1.0 / 255.0, 1.0 / 255.0, 1.0 / 255.0, 1.0);

    return color;
}
