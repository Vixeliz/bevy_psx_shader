#import bevy_sprite::mesh2d_view_bindings
#import bevy_sprite::mesh2d_bindings

struct PsxMaterial {
    // color: vec4<f32>,
    dither_amount: f32,
};
@group(1) @binding(0)
var<uniform> material: PsxMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;
@group(1) @binding(3)
var dither_color_texture: texture_2d<f32>;
@group(1) @binding(4)
var dither_color_sampler: sampler;

// NOTE: Bindings must come before functions that use them!
#import bevy_sprite::mesh2d_functions


struct FragmentInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec2<f32>,
    // #ifdef COLORED
    @location(2) uv: vec2<f32>
    // #endif
};

fn channel_error(col: f32, col_min: f32, col_max: f32) -> f32
{
    let range = abs(col_min - col_max);
    let aRange = abs(col - col_min);
    return aRange /range;
}

fn dithered_channel(error: f32, dither_blockUV: vec2<f32>, dither_steps: f32) -> f32
{
    let error_new = floor(error * dither_steps) / dither_steps;
    var ditherUV = vec2(error_new, 0.0);
    ditherUV.x += dither_blockUV.x;
    ditherUV.y = dither_blockUV.y;
    return textureSample(dither_color_texture, dither_color_sampler, ditherUV).x;
}

// float4 mix(float4 a, float4 b, float amt)
// {
//     return ((1.0 - amt) * a) + (b * amt);
// }

    /// YUV/RGB color space calculations

fn rgb_to_yuv(rgba: vec4<f32>) -> vec4<f32> {
    var yuva = vec4(0.0, 0.0, 0.0, 0.0);
    yuva.r = rgba.r * 0.2126 + 0.7152 * rgba.g + 0.0722 * rgba.b;
    yuva.g = (rgba.b - yuva.r) / 1.8556;
    yuva.b = (rgba.r - yuva.r) / 1.5748;
    yuva.a = rgba.a;
    
    // Adjust to work on GPU
    yuva.g += 0.5;
    yuva.b += 0.5;
    
    return yuva;
}

fn yuv_to_rgb(yuva: vec4<f32>) -> vec4<f32> {
    var new_yuva = yuva;
    new_yuva.g -= 0.5;
    new_yuva.b -= 0.5;
    return vec4(
        new_yuva.r * 1.0 + new_yuva.g * 0.0 + new_yuva.b * 1.5748,
        new_yuva.r * 1.0 + new_yuva.g * -0.187324 + new_yuva.b * -0.468124,
        new_yuva.r * 1.0 + new_yuva.g * 1.8556 + new_yuva.b * 0.0,
        new_yuva.a);
}


@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let col = textureSample(base_color_texture, base_color_sampler, in.uv);
    var yuv = rgb_to_yuv(col);

    // Clamp the YUV color to specified color depth (default: 32, 5 bits per channel, as per playstation hardware)
    var col1 = floor(yuv * material.dither_amount) / material.dither_amount;
    var col2 = ceil(yuv * material.dither_amount) / material.dither_amount;

    let main_texel_size = 1.0 / vec2<f32>(textureDimensions(base_color_texture));
    let dither_texel_size = 1.0 / vec2<f32>(textureDimensions(dither_color_texture));
    
    // Calculate dither texture UV based on the input texture
    var ditherSize = dither_texel_size.y;
    var ditherSteps = dither_texel_size.x/ditherSize;

    var ditherBlockUV = in.uv;
    ditherBlockUV.x %= (ditherSize / main_texel_size.x);
    ditherBlockUV.x /= (ditherSize / main_texel_size.x);
    ditherBlockUV.y %= (ditherSize / main_texel_size.y);
    ditherBlockUV.y /= (ditherSize / main_texel_size.y);
    ditherBlockUV.x /= ditherSteps;

    // Dither each channel individually
    yuv.x = mix(col1.x, col2.x, dithered_channel(channel_error(yuv.x, col1.x, col2.x), ditherBlockUV, ditherSteps));
    yuv.y = mix(col1.y, col2.y, dithered_channel(channel_error(yuv.y, col1.y, col2.y), ditherBlockUV, ditherSteps));
    yuv.z = mix(col1.z, col2.z, dithered_channel(channel_error(yuv.z, col1.z, col2.z), ditherBlockUV, ditherSteps));

    return yuv_to_rgb(yuv);
}