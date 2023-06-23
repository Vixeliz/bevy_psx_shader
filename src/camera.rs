use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::{RenderTarget, Viewport},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::{BevyDefault, ImageSampler},
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};

use crate::material::PsxDitherMaterial;

#[derive(Component)]
pub struct PsxCamera {
    pub size: UVec2,
    pub fixed_axis: Option<bool>,
    pub clear_color: Color,
    pub hdr: bool,
    pub dither_amount: f32,
    pub banding_enabled: u32,
    init: bool,
}

impl Default for PsxCamera {
    fn default() -> Self {
        Self {
            size: UVec2::new(256, 224),
            fixed_axis: None,
            clear_color: Color::WHITE,
            init: false,
            hdr: false,
            dither_amount: 48.0,
            banding_enabled: 1,
        }
    }
}

impl PsxCamera {
    pub fn new(
        size: UVec2,
        axis: Option<bool>,
        clear_color: Color,
        hdr: bool,
        dither_amount: f32,
        banding_enabled: u32,
    ) -> Self {
        Self {
            size,
            fixed_axis: axis,
            clear_color,
            init: false,
            hdr,
            dither_amount,
            banding_enabled,
        }
    }

    pub fn from_height(height: u32) -> Self {
        Self {
            size: UVec2::new(0, height),
            fixed_axis: Some(false),
            clear_color: Color::WHITE,
            init: false,
            hdr: false,
            ..default()
        }
    }
    pub fn from_width(width: u32) -> Self {
        Self {
            size: UVec2::new(width, 0),
            fixed_axis: Some(true),
            clear_color: Color::WHITE,
            init: false,
            hdr: false,
            ..default()
        }
    }
    pub fn from_resolution(width: u32, height: u32) -> Self {
        Self {
            size: UVec2::new(width, height),
            fixed_axis: None,
            clear_color: Color::WHITE,
            init: false,
            hdr: false,
            ..default()
        }
    }
}

#[derive(Component)]
pub struct RenderImage;

#[derive(Component)]
pub struct FinalCameraTag;

pub fn setup_camera(
    mut commands: Commands,
    mut camera: Query<(&mut PsxCamera, Entity)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PsxDitherMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    for (mut pixel_camera, entity) in camera.iter_mut() {
        if !pixel_camera.init {
            pixel_camera.init = true;
            let size = Extent3d {
                width: pixel_camera.size.x,
                height: pixel_camera.size.y,
                ..default()
            };

            // This is the texture that will be rendered to.
            let mut image = Image {
                texture_descriptor: TextureDescriptor {
                    label: None,
                    size,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::bevy_default(),
                    mip_level_count: 1,
                    sample_count: 1,
                    usage: TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_DST
                        | TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                },
                sampler_descriptor: ImageSampler::nearest(),
                ..default()
            };

            // fill image.data with zeroes
            image.resize(size);

            let image_handle = images.add(image);

            // The camera we are actually rendering to
            let camera = if pixel_camera.hdr {
                Camera3dBundle {
                    camera: Camera {
                        target: RenderTarget::Image(image_handle.clone()),
                        hdr: true,
                        ..default()
                    },
                    camera_3d: Camera3d {
                        clear_color: ClearColorConfig::Custom(pixel_camera.clear_color),
                        ..default()
                    },
                    ..Default::default()
                }
            } else {
                Camera3dBundle {
                    camera: Camera {
                        target: RenderTarget::Image(image_handle.clone()),
                        ..default()
                    },
                    camera_3d: Camera3d {
                        clear_color: ClearColorConfig::Custom(pixel_camera.clear_color),
                        ..default()
                    },
                    ..Default::default()
                }
            };

            commands
                .entity(entity)
                .insert((UiCameraConfig { show_ui: false }, camera));

            let render_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);
            let ui_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 2) as u8);

            let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
                size.width as f32,
                size.height as f32,
            ))));

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: quad_handle.into(),
                    material: materials.add(PsxDitherMaterial {
                        dither_amount: pixel_camera.dither_amount,
                        banding_enabled: pixel_camera.banding_enabled,
                        color_texture: Some(image_handle),
                        ..Default::default()
                    }),
                    transform: Transform { ..default() },
                    ..default()
                },
                render_layer,
                RenderImage,
            ));

            commands.spawn((
                Camera2dBundle {
                    camera: Camera {
                        viewport: Some(Viewport {
                            physical_size: UVec2 {
                                x: pixel_camera.size.x,
                                y: pixel_camera.size.y,
                            },
                            ..Default::default()
                        }),
                        // renders after the first main camera which has default value: 0.
                        order: 1,
                        ..default()
                    },
                    ..Camera2dBundle::default()
                },
                render_layer,
                FinalCameraTag,
                UiCameraConfig { show_ui: false },
            ));
            commands.spawn((
                Camera2dBundle {
                    camera: Camera {
                        // renders after the camera that draws the texture
                        order: 2,
                        ..default()
                    },
                    camera_2d: Camera2d {
                        clear_color: ClearColorConfig::None,
                    },
                    ..Default::default()
                },
                ui_layer,
            ));
        }
    }
}

pub fn scale_render_image(
    mut texture_query: Query<&mut Transform, With<RenderImage>>,
    mut camera_query: Query<&mut bevy::render::camera::Camera, With<FinalCameraTag>>,
    mut psx_camera_query: Query<&PsxCamera>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut texture_transform) = texture_query.get_single_mut() {
        if let Ok(window) = windows.get_single_mut() {
            if let Ok(mut camera) = camera_query.get_single_mut() {
                if let Ok(psx_camera) = psx_camera_query.get_single_mut() {
                    let (screen_width, screen_height) = (psx_camera.size.x, psx_camera.size.y);
                    let aspect_ratio = screen_width as f32 / screen_height as f32;
                    let window_size: UVec2 = if window.physical_height() > window.physical_width()
                        || window.physical_height() as f32 * aspect_ratio
                            > window.physical_width() as f32
                    {
                        UVec2::new(
                            window.physical_width(),
                            (window.physical_width() as f32 / aspect_ratio).floor() as u32,
                        )
                    } else {
                        UVec2::new(
                            (window.physical_height() as f32 * aspect_ratio).floor() as u32,
                            window.physical_height(),
                        )
                    };

                    let scale_width = window_size.x as f32 / screen_width as f32;
                    let scale_height = window_size.y as f32 / screen_height as f32;
                    let window_position: UVec2 = if window.physical_height()
                        > window.physical_width()
                        || window.physical_height() as f32 * aspect_ratio
                            > window.physical_width() as f32
                    {
                        if let Some(height) =
                            (window.physical_height() / 2).checked_sub(window_size.y / 2)
                        {
                            UVec2::new(0, height)
                        } else {
                            UVec2::ZERO
                        }
                    } else if let Some(width) =
                        (window.physical_width() / 2).checked_sub(window_size.x / 2)
                    {
                        UVec2::new(width, 0)
                    } else {
                        UVec2::ZERO
                    };

                    texture_transform.scale = Vec3::new(scale_width, scale_height, 1.0);

                    camera.viewport = Some(Viewport {
                        physical_size: window_size,
                        physical_position: window_position,
                        ..Default::default()
                    });
                }
            }
        }
    }
}
