pub mod camera;
pub mod material;
pub mod shader;

use bevy::{
    asset::load_internal_asset,
    prelude::*,
    render::{camera::ScalingMode, primitives::Aabb, view::VisibleEntities},
};

use crate::material::{PsxMaterial, PSX_FRAG_SHADER_HANDLE, PSX_VERT_SHADER_HANDLE};

pub struct PsxPlugin;

impl Plugin for PsxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<PsxMaterial>::default());
        app.register_type::<Camera>()
            .register_type::<Visibility>()
            .register_type::<ComputedVisibility>()
            .register_type::<OrthographicProjection>()
            .register_type::<VisibleEntities>()
            .register_type::<ScalingMode>()
            .register_type::<Aabb>()
            .add_system(camera::setup_camera.in_base_set(CoreSet::PostUpdate))
            .add_system(camera::scale_render_image);

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
    }
}
