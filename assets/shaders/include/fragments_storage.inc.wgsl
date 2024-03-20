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
    ty: u32, // 0 - middle fragment, 1 start fragment, 2 end fragment, 3 single fragment,
    line_cap_ty: u32,
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

@group($bg) @binding(0)
var<storage, read> component_ty_fragments: array<ComponentTyFragments>;
@group($bg) @binding(1)
var<storage, read> circles: array<CircleFragment>;
@group($bg) @binding(2)
var<storage, read> lines: array<LineFragment>;
@group($bg) @binding(3)
var<storage, read> rectangles: array<RectangleFragment>;
@group($bg) @binding(4)
var<storage, read> triangles: array<TriangleFragment>;