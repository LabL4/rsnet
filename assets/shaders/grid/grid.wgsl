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

@group(0) @binding(0)
var<uniform> camera: CameraUniform;
@group(0) @binding(1)
var<uniform> mouse: MouseUniform;
@group(0) @binding(2)
var<uniform> window: WindowUniform;

@group(1) @binding(0)
var<uniform> time: TimeData;

@group(2) @binding(0)
var<uniform> chunk_data: ChunkData;

const ANIM_DURATION: u32 = 180u; // ms

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

    let world_coord = input.world_coord;
        
    var grid_size = chunk_data.chunk_size;
    
    if time.time < chunk_data.last_chunk_size_update + ANIM_DURATION {
        let t = f32(time.time - chunk_data.last_chunk_size_update) / f32(ANIM_DURATION);
        grid_size = mix(chunk_data.prev_chunk_size, chunk_data.chunk_size, smoothstep(0.0, 1.0, t));
    }

    let grid_size_fourth = grid_size / 4.0;

    let delta = fwidth(min(world_coord.x, world_coord.y));
    let line_width = delta / grid_size * 4.0;
    let line_width_fourth = line_width * 2.0;

    let scaled_x = (world_coord.x - grid_size / 2.0) / grid_size;
    let scaled_y = (world_coord.y - grid_size / 2.0) / grid_size;

    let scaled_x_fourth = scaled_x * grid_size / grid_size_fourth;
    let scaled_y_fourth = scaled_y * grid_size / grid_size_fourth;

    var dist_x = abs(scaled_x - round(scaled_x));
    var dist_y = abs(scaled_y - round(scaled_y));

    let dist_x_fourth = abs(scaled_x_fourth - round(scaled_x_fourth));
    let dist_y_fourth = abs(scaled_y_fourth - round(scaled_y_fourth));

    output.color = vec4(1.0, 1.0, 1.0, 0.0);
    
    if ( dist_x < line_width && dist_y < line_width) {      

        var total_distance = length(vec2<f32>(dist_x , dist_y));
        let p = 0.9;

        output.color = vec4(0.0, 0.0, 0.0, 1.0 - smoothstep(line_width*p, line_width, total_distance));
    } else if ( dist_x_fourth < line_width_fourth && dist_y_fourth < line_width_fourth) {

        var total_distance = length(vec2<f32>(dist_x_fourth , dist_y_fourth));
        let p = 0.9;

        output.color = vec4(0.2, 0.2, 0.2, 1.0 - smoothstep(line_width_fourth*p, line_width_fourth, total_distance));
    }


    // output.color = vec4(1.0, 1.0, 0.0, 1.0) * abs(input.tex_coord.x);
    // output.color.w *= 0.7;


    return output;
}