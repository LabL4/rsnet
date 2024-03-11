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
}

const MAX_FRAGMENTS = 15;

@group(0) @binding(0)
var<uniform> camera: CameraUniform;
@group(0) @binding(1)
var<uniform> mouse: MouseUniform;
@group(0) @binding(2)
var<uniform> window: WindowUniform;

@group(1) @binding(0)
var<storage, read> component_ty_fragments: array<ComponentTyFragments>;
@group(1) @binding(1)
var<storage, read> circles: array<CircleFragment>;
@group(1) @binding(2)
var<storage, read> lines: array<LineFragment>;
@group(1) @binding(3)
var<storage, read> rectangles: array<RectangleFragment>;
@group(1) @binding(4)
var<storage, read> triangles: array<TriangleFragment>;

@group(2) @binding(0)
var<storage, read> components: array<Component>;

@group(3) @binding(0)
var<uniform> fragments_data: FragmentsData;

struct FragmentsData {
    fragments_idx: u32,
}

struct ComponentTyFragments {
    circles_start: u32,
    n_circles: u32,

    lines_start: u32,
    n_lines: u32,

    rectangles_start: u32,
    n_rectangles: u32,

    triangles_start: u32,
    n_triangles: u32,
}

struct CircleFragment {
    center: vec2<f32>,
    radius: f32,
    color: u32,
};

struct LineFragment {
    start: vec2<f32>,
    end: vec2<f32>,
    thickness: f32,
    ty: u32, // 0 - middle fragment, 1 start fragment, 2 end fragment, 3 single fragment
    color: u32,
};

struct RectangleFragment {
    center: vec2<f32>,
    size: vec2<f32>,
    color: u32
}

struct TriangleFragment {
    center: vec2<f32>,
    size: vec2<f32>,
    dir_vec: vec2<f32>,
    color: u32
}

struct Component {
    model: mat3x3<f32>,
    id: u32,
    ty: u32,
}


struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) fragment_idx: u32,
    @location(2) tex_coords: vec2<f32>,
    @location(3) component_idx: u32,
    @location(4) fragment_ty: u32,
}

fn mat3_to_mat4(m: mat3x3<f32>) -> mat4x4<f32> {
    return transpose(mat4x4<f32>(
        vec4<f32>(m[0][0], m[1][0], 0.0, m[2][0]),
        vec4<f32>(m[0][1], m[1][1], 0.0, m[2][1]),
        vec4<f32>(m[0][2], m[1][2], 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}

// ------ / Top side
// p1    p2
//  *    *
//  *    *
// p0   p3

// 00
// 01
// 10
// 11

fn rgb_from_u32(color: u32) -> vec4<f32> {
    return vec4<f32>(
        f32((color >> 16) & 0xFF) / 255.0,
        f32((color >> 8) & 0xFF) / 255.0,
        f32(color & 0xFF) / 255.0,
        1.0
    );
}

fn vs_circle(vertex_idx: u32, fragment: CircleFragment) -> VertexOutput {
    var output: VertexOutput;

    let vertex_x = (f32((vertex_idx >> 1u) & 1u) - 0.5) * 2.0;
    let vertex_y = (f32(vertex_idx & 1u) - 0.5) * 2.0;

    let vertex_pos = vec2<f32>(vertex_x, vertex_y) * fragment.radius + fragment.center;
    
    output.clip_pos = vec4<f32>(vertex_pos, 0.0, 1.0);
    output.color = rgb_from_u32(fragment.color);
    output.tex_coords = vec2<f32>(vertex_x, vertex_y);
    
    return output;
}

fn vs_line(vertex_idx: u32, fragment: LineFragment, prev_fragment: LineFragment, next_fragment: LineFragment) -> VertexOutput {
    var output: VertexOutput;
    
    let dir = normalize(fragment.end - fragment.start);
    let normal = vec2<f32>(-dir.y, dir.x); // Clockwise

    let right_bit = (vertex_idx >> 1u) & 1u;
    let up_bit = vertex_idx & 1u;
    
    let right_bit_f32 = f32(right_bit);
    let up_bit_f32 = f32(up_bit);
    
    let vertex = vec2<f32>(
        (right_bit_f32 - 0.5) * 2.0,
        (up_bit_f32 - 0.5) * 2.0
    );

    output.tex_coords = vertex;

    var selected_normal: vec2<f32>;
    
    if (right_bit == 1u && (fragment.ty == 1u || fragment.ty == 0u)) {
        let next_fragment_dir = normalize(next_fragment.end - next_fragment.start);
        let next_normal = vec2<f32>(-next_fragment_dir.y, next_fragment_dir.x);
        let intersection_vec_norm = normalize(normal + next_normal);
        let cos_theta = dot(intersection_vec_norm, normal);
        let intersection_vec = 1.0 / cos_theta * intersection_vec_norm;

        selected_normal = intersection_vec;

    } else if (right_bit == 0u && (fragment.ty == 2u || fragment.ty == 0u)) {
        let prev_fragment_dir = normalize(prev_fragment.end - prev_fragment.start);
        let prev_normal = vec2<f32>(-prev_fragment_dir.y, prev_fragment_dir.x);
        // let intersection_vec = normal + prev_normal;
        let intersection_vec_norm = normalize(normal + prev_normal);
        let cos_theta = dot(intersection_vec_norm, normal);
        let intersection_vec = 1.0 / cos_theta * intersection_vec_norm;

        selected_normal = intersection_vec;
    } else {
        selected_normal = normal;
    }
    
    let normal_with_dir = (selected_normal * up_bit_f32 - selected_normal * (1.0 - up_bit_f32)) * fragment.thickness / 2.0;


    let vertex_model = ((1.0 - right_bit_f32) * fragment.start
                     + (right_bit_f32) * fragment.end) + normal_with_dir;

    output.clip_pos = vec4<f32>(vertex_model, 0.0, 1.0);
    // output.clip_pos = vec4<f32>(vertex.x, vertex.y, 0.0, 1.0);
    output.color = rgb_from_u32(fragment.color);
    // output.color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    
    return output;
}

fn vs_rectangle(vertex_idx: u32, fragment: RectangleFragment) -> VertexOutput {
    var output: VertexOutput;

    let vertex = vec2<f32>(
        (f32((vertex_idx >> 1u) & 1u) - 0.5) * 2.0,
        (f32(vertex_idx & 1u) - 0.5) * 2.0
    );
    
    // output.clip_pos = vec4<f32>(vertex.x , vertex.y, 0.0, 1.0) + vec4<f32>(fragment.center, 0.0, 0.0);
    output.clip_pos = vec4<f32>(vertex.x * fragment.size.x / 2.0, vertex.y * fragment.size.y / 2.0, 0.0, 1.0) + vec4<f32>(fragment.center, 0.0, 0.0);

    output.color = rgb_from_u32(fragment.color);
    output.tex_coords = vertex;

    
    return output;
}

fn vs_triangle(vertex_idx: u32, real_vertex_idx: u32, fragment: TriangleFragment) -> VertexOutput {
    var output: VertexOutput;

    if (real_vertex_idx > 2u) {
        // Apparently GPUs understand this magic. It's a way to discard a fragment.
        output.clip_pos.x = bitcast<f32>(0x7FC00000u); // NaN
        return output;
    }

    //   *   // 1
    //  / \
    // *---*
    // 0   2

    let up_bit = vertex_idx & 1u;
    let right_bit = (vertex_idx >> 1u) & 1u;

    let vertex = vec2<f32>(
        (f32(right_bit) - 0.5) * 2.0 * (1.0 - f32(up_bit)),
        (f32(up_bit) - 0.5) * 2.0
    );

    let dir_normal = vec2<f32>(-fragment.dir_vec.y, fragment.dir_vec.x); // CCW
    
    let pos = vertex * fragment.size / 2.0;
    output.clip_pos = vec4<f32>((pos.x * fragment.dir_vec + pos.y * dir_normal) + fragment.center, 0.0, 1.0);


    output.color = rgb_from_u32(fragment.color);
    output.tex_coords = vertex;

    
    return output;
}

@vertex
fn vs_main(
    // @location(0) component_idx: u32,
    // @location(0) fragments_idx: u32,
    @builtin(vertex_index) vertex_idx: u32,
    @builtin(instance_index) instance_index: u32,
) -> VertexOutput {

    let fragment_idx = vertex_idx / 6;
    let component_idx = instance_index;

    var output: VertexOutput;

    let component = components[component_idx];
    var fragments = component_ty_fragments[fragments_data.fragments_idx];
    let vertex_idx_in_fragment = vertex_idx % 6 - (vertex_idx % 6) / 3u * 2u;

    
    let first_idx = fragments.n_circles + 1u;
    let second_idx = first_idx + fragments.n_lines;
    let third_idx = second_idx + fragments.n_rectangles;


    let n_desp = 1u - clamp(first_idx, 0u, 1u);
    let calc_idx = fragment_idx + 1u;


    var idx = u32(max(0u,
        max(
            clamp(calc_idx / first_idx, 0u, 1u),
            max(
                clamp(calc_idx / second_idx, 0u, 1u) * 2u,
                clamp(calc_idx / third_idx, 0u, 1u) * 3u
            )
        )
    )) ;

    if (fragment_idx == 14u) {
        idx = 3u;
    }


    switch idx {
        case 3u: {
            var fragment = triangles[ fragments.triangles_start + fragment_idx - fragments.n_rectangles - fragments.n_lines - fragments.n_circles ];
            output = vs_triangle(vertex_idx_in_fragment, vertex_idx % 6, fragment);
            if (output.clip_pos.x == bitcast<f32>(0x7FC00000u)) {
                return output;
            }
            output.fragment_ty = 3u;
        }
        case 2u: {
            var fragment = rectangles[ fragments.rectangles_start + fragment_idx - fragments.n_lines - fragments.n_circles];
            output = vs_rectangle(vertex_idx_in_fragment, fragment);
            output.fragment_ty = 2u;
            // output.color = vec4<f32>(1.0, 0.0, f32(fragments.rectangles_start + fragment_idx), 1.0);
        }
        case 1u: {
            let idx = fragment_idx - fragments.n_circles;
            let prev_idx = u32(max(0i, i32(idx) - 1i));
            let next_idx = min(arrayLength(&lines) - 1u, idx + 1u);

            var fragment = lines[fragments.lines_start + idx];
            var prev_fragment = lines[fragments.lines_start + prev_idx];
            var next_fragment = lines[fragments.lines_start + next_idx];

            output = vs_line(vertex_idx_in_fragment, fragment, prev_fragment, next_fragment);
            output.fragment_ty = 1u;
        }
        case 0u, default {
            var fragment = circles[fragments.circles_start + fragment_idx];
            output = vs_circle(vertex_idx_in_fragment, fragment);
            output.fragment_ty = 0u;
        }
    }

    output.clip_pos = camera.view_proj * mat3_to_mat4(component.model) * output.clip_pos;
    output.component_idx = component_idx;


    return output;
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

fn fs_circle(in: VertexOutput) -> FragmentOutput {
    var output: FragmentOutput;
    var dist = length(in.tex_coords);
    if (dist < 1.0) {
        output.color = in.color;
        output.color.w = smoothstep(0.0, camera.radius / 10.0 * 0.02, 1.0 - dist);
    } else {
        discard;
    }
    return output;
}

fn fs_line(in: VertexOutput) -> FragmentOutput {
    var output: FragmentOutput;

    output.color = in.color;

    return output;
}

fn fs_rectangle(in: VertexOutput) -> FragmentOutput {
    var output: FragmentOutput;

    output.color = in.color;

    return output;
}

fn fs_triangle(in: VertexOutput) -> FragmentOutput {
    var output: FragmentOutput;

    output.color = in.color;

    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {
    var output: FragmentOutput;

    switch in.fragment_ty {
        case 0u: {
            output = fs_circle(in);
        }
        case 1u: {
            output = fs_line(in);
        }
        case 2u: {
            output = fs_rectangle(in);
        }
        case 3u, default: {
            output = fs_triangle(in);
        }
    };
    
    return output;
}