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
    
    let dir = normalize(wire.end - wire.start);
    let normal = vec2<f32>(-dir.y, dir.x); // Clockwise

    let right_bit = (vertex_idx >> 1u) & 1u;
    let up_bit = (vertex_idx & 1u);

    // up_bit = ~(up_bit ^ (u32(step(0.0, dir.x)) | right_bit)) & 1u;
    // right_bit = ~(right_bit ^ u32(step(0.0, dir.x))) & 1u;


    // 0 0 1
    // 0 1 0
    // 1 0 0
    // 1 1 1
    
    let right_bit_f32 = f32(right_bit);
    let up_bit_f32 = f32(up_bit);
    
    let vertex = vec2<f32>(
        (right_bit_f32 - 0.5) * 2.0,
        (up_bit_f32 - 0.5) * 2.0
    );

    var selected_normal: vec2<f32>;

    // needs cap at end if 1u
    let cap_at_end = right_bit == 1u;
    let cap_at_start = right_bit == 0u;

    let next_prev_wire_dir =
        normalize(wire.next_dir) * f32(cap_at_end) +
        normalize(wire.prev_dir) * f32(cap_at_start);

    let prev_next_normal = vec2<f32>(-next_prev_wire_dir.y, next_prev_wire_dir.x);

    let intersection_vec_norm = normalize(normal + prev_next_normal);
    let cos_theta = dot(intersection_vec_norm, normal);
    let intersection_vec = 1.0 / cos_theta * intersection_vec_norm;

    selected_normal = intersection_vec;
    
    let normal_with_dir = (selected_normal * up_bit_f32 - selected_normal * (1.0 - up_bit_f32)) * THICKNESS / 2.0;


    let vertex_model = ((1.0 - right_bit_f32) * wire.start
                     + (right_bit_f32) * wire.end) + normal_with_dir;

    // output.clip_pos = vec4<f32>(vertex.x, vertex.y, 0.0, 1.0);
    // output.color = rgb_from_u32(wire.color);
    output.color = vec4<f32>(0.0, 0.0, 0.0, 1.0);

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