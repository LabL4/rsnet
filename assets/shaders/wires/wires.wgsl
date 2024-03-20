//!include common.inc $bg=0

//!include fragments_storage.inc $bg=1

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) world_coord: vec2<f32>,
    @location(2) ss_coords: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_idx: u32, @builtin(instance_index) instance_idx: u32) -> VertexOutput {

    var output: VertexOutput;
    // var a = 10;
    // if a == 10 {
    //     output.color = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    // } else {
    //     output.color = vec4<f32>(0.0, 1.0, 0.0, 1.0);
    // }
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