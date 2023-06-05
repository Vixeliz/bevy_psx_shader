pub mod material;
pub mod shader;

use bevy::{asset::load_internal_asset, prelude::*};

use crate::material::{
    fill_material_map, replace_materials, MaterialMap, PsxMaterial, PSX_FRAG_SHADER_HANDLE,
    PSX_VERT_SHADER_HANDLE,
};

pub struct PsxPlugin;

impl Plugin for PsxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<PsxMaterial>::default());
        app.insert_resource(MaterialMap::default());
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

        // app.add_systems((fill_material_map, replace_materials));
    }
}
