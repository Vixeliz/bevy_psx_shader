pub mod camera;
pub mod material;
pub mod shader;

use bevy::{
    asset::{load_internal_asset, load_internal_binary_asset},
    prelude::*,
    render::{
        camera::ScalingMode,
        primitives::Aabb,
        render_resource::{AddressMode, SamplerDescriptor},
        texture::{CompressedImageFormats, ImageSampler, ImageTextureLoader, ImageType},
        view::VisibleEntities,
    },
    sprite::Material2dPlugin,
};

use crate::material::{
    PsxDitherMaterial, PsxMaterial, PSX_DITHER_HANDLE, PSX_DITHER_SHADER_HANDLE,
    PSX_FRAG_SHADER_HANDLE, PSX_VERT_SHADER_HANDLE,
};

pub fn image_load(bytes: &[u8]) -> Image {
    let mut image = Image::from_buffer(
        bytes,
        // include_bytes!("psx-dith.png"),
        ImageType::Extension("png"),
        CompressedImageFormats::NONE,
        true,
    )
    .unwrap();

    let mut image_descriptor = ImageSampler::nearest_descriptor();
    image_descriptor.label = Some("psx_dith_sampler");
    image.sampler_descriptor = ImageSampler::Descriptor(image_descriptor);

    image
}

pub struct PsxPlugin;

impl Plugin for PsxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<PsxMaterial>::default());
        app.add_plugin(Material2dPlugin::<PsxDitherMaterial>::default());
        app.register_type::<Camera>()
            .register_type::<Visibility>()
            .register_type::<ComputedVisibility>()
            .register_type::<OrthographicProjection>()
            .register_type::<VisibleEntities>()
            .register_type::<ScalingMode>()
            .register_type::<Aabb>()
            .add_system(camera::setup_camera.in_base_set(CoreSet::PostUpdate))
            .add_system(camera::scale_render_image);

        load_internal_binary_asset!(app, PSX_DITHER_HANDLE, "psx-dith.png", image_load);

        load_internal_asset!(
            app,
            PSX_FRAG_SHADER_HANDLE,
            "psx-frag.wgsl",
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            PSX_VERT_SHADER_HANDLE,
            "psx-vert.wgsl",
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            PSX_DITHER_SHADER_HANDLE,
            "psx-dith.wgsl",
            Shader::from_wgsl
        );
    }
}
