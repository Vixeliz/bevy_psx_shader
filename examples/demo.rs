use bevy_psx::{material::PsxMaterial, PsxPlugin};

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins, // .set(
                            //     AssetPlugin {
                            //     // Hot reloading the shader works correctly
                            //     watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                            //     ..default()
                            // }
                            // )
        )
        .add_plugin(PsxPlugin)
        // .add_plugin(PsxPlugin)
        // .init_resource::<PsxConfig>()
        .add_startup_system(setup)
        .add_system(rotate)
        .run();
}

/// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PsxMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0))
            .looking_at(Vec3::default(), Vec3::Y),
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::WHITE),
            ..default()
        },
        ..default()
    },));

    // cube
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(PsxMaterial {
                color: Color::rgb(1.0, 1.0, 1.0),
                color_texture: Some(asset_server.load("crate.png")),
                alpha_mode: AlphaMode::Opaque,
            }),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        Rotates,
    ));
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });
}

#[derive(Component)]
struct Rotates;

/// Rotates any entity around the x and y axis
fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in &mut query {
        transform.rotate_x(0.55 * time.delta_seconds());
        transform.rotate_z(0.15 * time.delta_seconds());
    }
}
