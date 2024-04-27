//!include common.inc $bg=0

//!include scene_storage.inc $bg=1

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) world_coord: vec2<f32>,
    @location(2) ss_coords: vec2<f32>,
}

const THICKNESS: f32 = 0.1;

@vertex
fn vs_main(@builtin(vertex_index) vertex_idx: u32, @builtin(instance_index) instance_idx: u32) -> VertexOutput {

    var output: VertexOutput;

    let wire = wires[instance_idx];

    let right_bit = (vertex_idx >> 1u) & 1u;
    let up_bit = vertex_idx & 1u;
    
    let right_bit_f32 = f32(right_bit);
    let up_bit_f32 = f32(up_bit);

    let dir = normalize(wire.end - wire.start);
    let normal = vec2<f32>(-dir.y, dir.x); // Clockwise

    let normal_with_dir = (normal * up_bit_f32 - normal * (1.0 - up_bit_f32)) * THICKNESS / 2.0;


    let vertex_model = ((1.0 - right_bit_f32) * wire.start
                     + (right_bit_f32) * wire.end) + normal_with_dir;

    output.clip_pos = camera.view_proj * vec4<f32>(vertex_model, 0.0, 1.0);

    return output;
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@fragment
fn fs_main(input: VertexOutput) -> FragmentOutput {
    var output: FragmentOutput;
    
    output.color = vec4(0.0, 0.0, 0.0, 1.0);

    return output;
}