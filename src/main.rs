#![warn(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery,
)]
#![allow(
    clippy::needless_pass_by_value,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::many_single_char_names,
    clippy::cast_sign_loss
)]

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::{self, LogPlugin},
    prelude::*,
    window::{self, Window, WindowResolution},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const WINDOW_DIMENSIONS: (f32, f32) = (1280., 720.);
/* const WINDOW_DIMENSIONS: (f32, f32) = (960., 540.); */
const WINDOW_WIDTH: f32 = WINDOW_DIMENSIONS.0;
const WINDOW_HEIGHT: f32 = WINDOW_DIMENSIONS.1;

fn main() {
    App::new()
        /* .insert_resource(ClearColor(BACKGROUND_COLOR)) */
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                        title: "Break The Blocks (BTB)".into(),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(LogPlugin {
                    level: log::Level::DEBUG,
                    filter: "debug,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                }),
        )
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_startup_system(setup_rgb_cube)
        .add_system(window::close_on_esc)
        .add_system(rotate_camera)
        .run();
}

fn rotate_camera(query: Query<(&mut Transform, &Name)>) {
    for (transform, name) in query.iter() {
        if name.contains("Route") {
            continue;
        }
    }
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // Camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-0.5, -0.5, 15.5).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(Name::new("Camera"));

    // Camera Tracking route
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: 5.125,
                    ..Default::default()
                })
                .unwrap(),
            ),
            visibility: Visibility::Hidden,
            computed_visibility: ComputedVisibility::HIDDEN,
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        })
        .insert(Name::new("Camera Route"));

    // TODO: Check for better Combinations
    commands.spawn_batch([
        PointLightBundle {
            transform: Transform::from_xyz(-7., -7., -7.),
            ..Default::default()
        },
        PointLightBundle {
            transform: Transform::from_xyz(-7., -7., 7.),
            ..Default::default()
        },
        PointLightBundle {
            transform: Transform::from_xyz(7., 7., -7.),
            ..Default::default()
        },
        PointLightBundle {
            transform: Transform::from_xyz(7., -7., -7.),
            ..Default::default()
        },
    ]);
}

fn setup_rgb_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const I_GRID_COUNT: i32 = (WINDOW_WIDTH / 256.) as i32;
    const F_GRID_COUNT: f32 = I_GRID_COUNT as f32;
    /* let _grid_pad = 0.25; */

    for ix in -I_GRID_COUNT..I_GRID_COUNT {
        for iy in -I_GRID_COUNT..I_GRID_COUNT {
            for iz in -I_GRID_COUNT..I_GRID_COUNT {
                let ix = ix as f32;
                let iy = iy as f32;
                let iz = iz as f32;

                let x = ix;
                let y = iy;
                let z = iz;

                let r = 255. * (ix / F_GRID_COUNT);
                let g = 255. * (iy / F_GRID_COUNT);
                let b = 255. * (iz / F_GRID_COUNT);
                commands.spawn(PbrBundle {
                    mesh: meshes.add(
                        Mesh::try_from(shape::Icosphere {
                            radius: 0.25,
                            ..Default::default()
                        })
                        .unwrap(),
                    ),
                    material: materials.add(Color::rgb_u8(r as u8, g as u8, b as u8).into()),
                    transform: Transform::from_xyz(x, y, z),
                    ..Default::default()
                });
            }
        }
    }
}
