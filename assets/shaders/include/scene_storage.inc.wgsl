struct Component {
    model: mat3x3<f32>,
    id: u32,
    ty: u32,
}
struct Wire {
    id: u32,
    start: vec2<f32>,
    end: vec2<f32>,
    prev_dir: vec2<f32>,
    next_dir: vec2<f32>
}

@group($bg) @binding(0)
var<storage, read> components: array<Component>;
@group($bg) @binding(1)
var<storage, read> wires: array<Wire>;