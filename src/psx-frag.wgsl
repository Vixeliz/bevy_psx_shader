#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

struct PsxMaterial {
    color: vec4<f32>,
    fog_color: vec4<f32>,
    snap_amount: f32,
    fog_distance: vec2<f32>
};
@group(1) @binding(0)
var<uniform> material: PsxMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions


struct FragmentInput {
    @location(0) c_position: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) fog: f32,
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let output_color = material.color * textureSample(base_color_texture, base_color_sampler, in.uv / in.c_position.w);
    return vec4(mix(output_color.rgb, material.fog_color.rgb, in.fog), 1.0);
}