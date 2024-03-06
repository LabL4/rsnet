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

const MAX_FRAGMENTS = 10;

@group(0) @binding(0)
var<uniform> camera: CameraUniform;
@group(0) @binding(1)
var<uniform> mouse: MouseUniform;
@group(0) @binding(2)
var<uniform> window: WindowUniform;

@group(1) @binding(0)
var<storage, read> component_fragments: array<Fragments>;

@group(2) @binding(0)
var<storage, read> components: array<Component>;

struct Fragments {
    circles: array<CircleFragment, MAX_FRAGMENTS>,
    n_circles: u32,
    
    lines: array<LineFragment, MAX_FRAGMENTS>,
    n_lines: u32,
    
    rectangles: array<RectangleFragment, MAX_FRAGMENTS>,
    n_rectangles: u32
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
    ty: u32, // 0 - middle fragment, 1 start fragment, 2 end fragment
    color: u32,
};

struct RectangleFragment {
    center: vec2<f32>,
    size: vec2<f32>,
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

fn vs_line(vertex_idx: u32, fragment: LineFragment) -> VertexOutput {
    var output: VertexOutput;
    
    let dir = normalize(fragment.end - fragment.start);
    let normal = vec2<f32>(-dir.y, dir.x); // Clockwise
    
    let right_bit_f32 = f32((vertex_idx >> 1u) & 1u);
    let up_bit_f32 = f32(vertex_idx & 1u);
    
    let vertex = vec2<f32>(
        (right_bit_f32 - 0.5) * 2.0,
        (up_bit_f32 - 0.5) * 2.0
    );

    let normal_with_dir = (normal * up_bit_f32 - normal * (1.0 - up_bit_f32)) * fragment.thickness / 2.0;

    let vertex_world = ((1.0 - right_bit_f32) * fragment.start
                     + (right_bit_f32) * fragment.end) + normal_with_dir;

    // output.clip_pos = vec4<f32>(vertex * 2.0, 0.0, 1.0);
    output.clip_pos = vec4<f32>(vertex_world, 0.0, 1.0);
    // output.clip_pos.y = vertex.y * 0.5;
    output.color = rgb_from_u32(fragment.color);
    output.color.w = fragment.end.x / 2.0;
    
    return output;
}

fn vs_rectangle(vertex_idx: u32, fragment: RectangleFragment) -> VertexOutput {
    var output: VertexOutput;

    let vertex = vec2<f32>(
        (f32((vertex_idx >> 1u) & 1u) - 0.5) * 2.0,
        (f32(vertex_idx & 1u) - 0.5) * 2.0
    );
    
    output.clip_pos = vec4<f32>(vertex.x * fragment.size.x, vertex.y * fragment.size.y, 0.0, 1.0) + vec4<f32>(fragment.center, 0.0, 0.0);
    output.color = rgb_from_u32(fragment.color);
    output.tex_coords = vertex;

    
    return output;
}

@vertex
fn vs_main(
    // @location(0) component_idx: u32,
    // @location(1) fragment_idx: u32,
    @builtin(vertex_index) vertex_idx: u32,
    @builtin(instance_index) instance_index: u32,
) -> VertexOutput {

    let fragment_idx = vertex_idx / 4;
    let component_idx = instance_index;

    var output: VertexOutput;

    let component = components[component_idx];
    var fragments = component_fragments[component.ty];
    let vertex_idx_in_fragment = vertex_idx % 4;

    if (fragment_idx < fragments.n_circles) {
    
        var fragment = fragments.circles[fragment_idx];
        output = vs_circle(vertex_idx_in_fragment, fragment);
        output.fragment_ty = 0u;
    
    } else if (fragment_idx < fragments.n_circles + fragments.n_lines) {
    
        var fragment = fragments.lines[fragment_idx - fragments.n_circles];
        output = vs_line(vertex_idx_in_fragment, fragment);
        output.fragment_ty = 1u;
    
    } else {
    
        var fragment = fragments.rectangles[fragment_idx - fragments.n_circles - fragments.n_lines];
        output = vs_rectangle(vertex_idx_in_fragment, fragment);
        output.fragment_ty = 2u;
    
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

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {
    var output: FragmentOutput;

    if (in.fragment_ty == 0) {
        output = fs_circle(in);
    } else if (in.fragment_ty == 1) {
        output = fs_line(in);
    } else {
        output = fs_rectangle(in);
    }
    
    return output;
}