#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

// @group(1) @binding(0)
// var<uniform> material: CustomMaterial;

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec4<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) c_position: vec4<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let grid = vec2(480.0, 360.0) * 0.5;
    let in_clip = mesh_position_local_to_clip(mesh.model, vertex.position);
    var snapped = in_clip;
    snapped = vec4(
        in_clip.x / in_clip.w,
        in_clip.y / in_clip.w,
        in_clip.z / in_clip.w,
        snapped.w
    );
    snapped = vec4(
        (floor(grid * snapped.xy) / grid).x,
        (floor(grid * snapped.xy) / grid).y,
        snapped.z,
        snapped.w
    );
    snapped *= vertex.position.w;
    
    out.clip_position = snapped;
    out.c_position = snapped;
    out.uv = vertex.uv * snapped.w;

    return out;
}