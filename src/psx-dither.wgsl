#import bevy_sprite::mesh2d_view_bindings
#import bevy_sprite::mesh2d_bindings

struct PsxDitherMaterial {
    dither_amount: f32,
    banding_enabled: u32
};

@group(1) @binding(0)
var<uniform> material: PsxDitherMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;
@group(1) @binding(3)
var dither_color_texture: texture_2d<f32>;
@group(1) @binding(4)
var dither_color_sampler: sampler;

struct FragmentInput {
    @location(0) c_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let base_col = textureSample(base_color_texture, base_color_sampler, in.uv);
    let dith_size = vec2<f32>(textureDimensions(dither_color_texture));
    let buf_size = vec2<f32>(textureDimensions(base_color_texture));
    let dith = textureSample(dither_color_texture, dither_color_sampler, in.uv * (buf_size / dith_size)).rgb - 0.5;
    var final_col = vec3(0.0, 0.0, 0.0);
    if material.banding_enabled > 0u {
        final_col = round(base_col.rgb * material.dither_amount + dith * (1.0)) / material.dither_amount;
    } else {
        final_col = round(base_col.rgb * material.dither_amount + dith * (0.0)) / material.dither_amount;
    }
    return vec4(final_col, 1.0);
}