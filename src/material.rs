use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    utils::HashMap,
};

pub const PSX_FRAG_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 310591614790536);
pub const PSX_VERT_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 120592519790135);

impl Material for PsxMaterial {
    fn fragment_shader() -> ShaderRef {
        PSX_FRAG_SHADER_HANDLE.typed().into()
        // "shaders/custom_material.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        PSX_VERT_SHADER_HANDLE.typed().into()
        // "shaders/custom_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct PsxMaterial {
    #[uniform(0)]
    pub color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct MaterialMap(pub HashMap<Handle<StandardMaterial>, Handle<PsxMaterial>>);

pub fn fill_material_map(
    mut events: EventReader<AssetEvent<bevy::gltf::Gltf>>,
    loader: Res<AssetServer>,
    models: Res<Assets<bevy::gltf::Gltf>>,
    mut new_mats: ResMut<Assets<PsxMaterial>>,
    mats: Res<Assets<StandardMaterial>>,
    mut map: ResMut<MaterialMap>,
) {
    for event in events.iter() {
        if let AssetEvent::Created { handle } = event {
            let Some(model) = models.get(handle) else {continue;};
            let Some(path) = loader.get_handle_path(&model.scenes[0]) else {continue;};

            info!("Setting psx shader on {:?}", path.path());

            for (_, mat_handle) in model.named_materials.iter() {
                if let Some(old_material) = mats.get(mat_handle) {
                    let new_mat = new_mats.add(PsxMaterial {
                        color: old_material.base_color,
                        color_texture: old_material.base_color_texture.clone(),
                        alpha_mode: old_material.alpha_mode,
                    });

                    map.insert(mat_handle.clone(), new_mat);
                }
            }
        }
    }
}

pub fn replace_materials(
    mut commands: Commands,
    query: Query<(Entity, &Handle<StandardMaterial>)>,
    map: Res<MaterialMap>,
) {
    for (entity, old_handle) in query.iter() {
        let Some(new_handle) = map.get(old_handle) else {continue;};
        let mut entity = commands.entity(entity);
        entity.remove::<Handle<StandardMaterial>>();
        entity.insert(new_handle.clone());
    }
}
