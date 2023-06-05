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
    let position_world = mesh_position_local_to_world(mesh.model, vertex.position);
    let snap_scale = 30.0;
    var position = vec4(
        floor(position_world.x * snap_scale) / snap_scale,
        floor(position_world.y * snap_scale) / snap_scale,
        floor(position_world.z * snap_scale) / snap_scale,
        vertex.position.w
    );

    let in_clip = mesh_position_local_to_clip(mesh.model, position);
    
    out.clip_position = in_clip;
    out.c_position = in_clip;
    out.uv = vertex.uv * in_clip.w;

    return out;
}