use bevy::{prelude::*, window::CursorGrabMode};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_psx::{camera::PsxCamera, material::PsxMaterial, PsxPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PsxPlugin)
        .add_plugin(FlyCameraPlugin)
        .insert_resource(Msaa::Off)
        .add_startup_system(setup)
        .add_systems((rotate, grab_mouse))
        .run();
}

fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

/// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PsxMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((PsxCamera::default(), FlyCamera::default()));
    // cube
    let transform =
        Transform::from_scale(Vec3::splat(2.0)).with_translation(Vec3::new(0.0, -5.0, -10.0));
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(PsxMaterial {
                color: Color::rgb(1.0, 1.0, 1.0),
                color_texture: Some(asset_server.load("crate.png")),
                alpha_mode: AlphaMode::Opaque,
            }),
            transform,
            ..default()
        },
        // Rotates,
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
