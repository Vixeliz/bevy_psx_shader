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
    let in_clip = mesh_position_local_to_clip(mesh.model, vertex.position);
    let snap_scale = 20.0;
    var position = vec4(
        in_clip.x  / in_clip.w,
        in_clip.y  / in_clip.w,
        in_clip.z  / in_clip.w,
        in_clip.w
    );
    position = vec4(
        floor(in_clip.x * snap_scale) / snap_scale,
        floor(in_clip.y * snap_scale) / snap_scale,
        in_clip.z,
        in_clip.w
    );

    out.clip_position = position;
    out.c_position = position;
    out.uv = vertex.uv * position.w;

    return out;
}