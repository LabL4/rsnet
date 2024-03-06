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

@group(0) @binding(0)
var<uniform> camera: CameraUniform;
@group(0) @binding(1)
var<uniform> mouse: MouseUniform;
@group(0) @binding(2)
var<uniform> window: WindowUniform;

// p1    p3
//  *    *
//  *    *
// p0   p2

@vertex
fn vs_main(@builtin(vertex_index) vertex_idx: u32, @builtin(instance_index) instance_idx: u32) -> VertexOutput {

    var output: VertexOutput;

    var vertex_x = (f32((vertex_idx >> 1u) & 1u) - 0.5) * 2.0;
    var vertex_y = (f32(vertex_idx & 1u) - 0.5) * 2.0;

    // var vertex_x: f32;
    // var vertex_y: f32;

    // if (vertex_idx == 0u) {
    //     vertex_x = -1.0;
    //     vertex_y = -1.0;
    // } else if (vertex_idx == 1u) {
    //     vertex_x = -1.0;
    //     vertex_y = 1.0;
    // } else if (vertex_idx == 2u) {
    //     vertex_x = 1.0;
    //     vertex_y = -1.0;
    // } else if (vertex_idx == 3u) {
    //     vertex_x = 1.0;
    //     vertex_y = 1.0;
    // }

    let right_bit = (vertex_idx >> 1u) & 1u;
    let up_bit = vertex_idx & 1u;

    let world_x = camera.aabb.min.x * (1.0 - f32(right_bit)) + camera.aabb.max.x * f32(right_bit);
    let world_y = camera.aabb.min.y * (1.0 - f32(up_bit)) + camera.aabb.max.y * f32(up_bit);

    // vertex_x *= 0.8;
    // vertex_y *= 0.8;

    output.clip_pos = vec4(vertex_x, vertex_y, 0.0, 1.0);
    output.color = vec4(0.0, 0.0, 0.0, 1.0);
    output.world_coord = vec2<f32>(world_x, world_y);
    output.ss_coords = vec2<f32>(vertex_x, vertex_y);
    
    return output;
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@fragment
fn fs_main(input: VertexOutput) -> FragmentOutput {

    var output: FragmentOutput;
    
    let radius = camera.radius;
    let grid_size = 10.0;//1.0 * max(floor(radius / 30.0)*1.5, 1.0);
    // let line_width = clamp(radius / 2000.0, 0.005, 0.01);
    let delta = fwidth(1.0 - min(input.world_coord.x, input.world_coord.y));
    let line_width = clamp(delta * 0.5, 0.0, 0.1);

    let world_coord = input.world_coord;

    var dist_x = abs((world_coord.x - grid_size / 2) / grid_size - round((world_coord.x - grid_size / 2) / grid_size));
    var dist_y = abs((world_coord.y - grid_size / 2) / grid_size - round((world_coord.y - grid_size / 2) / grid_size));

    output.color = vec4(1.0, 1.0, 1.0, 0.0);
    
    if ( dist_x < line_width || dist_y < line_width) {
        dist_x = (1.0 - step(line_width, dist_x)) * (dist_x - line_width);
        dist_y = (1.0 - step(line_width, dist_y)) * (dist_y - line_width);
        
        var total_distance = length(vec2<f32>(dist_x , dist_y));

        output.color = vec4(0.0, 0.0, 0.0, smoothstep(0.0, line_width*0.1, total_distance));
    }


    // output.color = vec4(1.0, 1.0, 0.0, 1.0) * abs(input.tex_coord.x);
    // output.color.w = 1.0;


    return output;
}