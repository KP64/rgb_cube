#![warn(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery
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
    window::{Window, WindowResolution},
};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const WINDOW_DIMENSIONS: (f32, f32) = (1280., 720.);
const WINDOW_WIDTH: f32 = WINDOW_DIMENSIONS.0;
const WINDOW_HEIGHT: f32 = WINDOW_DIMENSIONS.1;

fn main() {
    App::new()
        .init_resource::<RotatorSpeed>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                        title: "RGB Cube".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(LogPlugin {
                    level: log::Level::DEBUG,
                    filter: "debug,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
                }),
        )
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .add_startup_system(setup_environment)
        .add_startup_system(setup_rgb_cube)
        .add_system(rotate_rgb_cube)
        .run();
}

fn rotate_rgb_cube(
    mut query: Query<&mut Transform, With<Rotator>>,
    time: Res<Time>,
    rot_speed: Res<RotatorSpeed>,
) {
    for mut transform in &mut query {
        transform.rotate_y(rot_speed.0 * time.delta_seconds());
    }
}

fn setup_environment(mut commands: Commands) {
    const LIGHT_POSITION: f32 = 6.;
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-0.5, -0.5, 15.5).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCam)
        .insert(Name::new("Camera"));

    // * Going through every combination
    for x in 0..2 {
        for y in 0..2 {
            for z in 0..2 {
                let x = if x % 2 == 0 { 1. } else { -1. };
                let y = if y % 2 == 0 { 1. } else { -1. };
                let z = if z % 2 == 0 { 1. } else { -1. };
                commands.spawn(PointLightBundle {
                    transform: Transform::from_translation(Vec3::new(x, y, z) * LIGHT_POSITION),
                    ..Default::default()
                });
            }
        }
    }
}

#[derive(Component)]
struct Rotator;

#[derive(Resource)]
struct RotatorSpeed(f32);

impl Default for RotatorSpeed {
    fn default() -> Self {
        Self(1.)
    }
}

fn setup_rgb_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const GRID_COUNT: f32 = WINDOW_WIDTH / 256.;
    const I_GRID_COUNT: i32 = GRID_COUNT as i32;

    let gegenkathete = 4.;
    let hypotenuse = (2. * 4_f32.powi(2)).sqrt();

    let alpha = (gegenkathete / hypotenuse).asin();

    let mat = Mat3::from_angle(90. - alpha);
    let res = mat * Vec3::splat(4.);
    let transform = Transform::from_translation(res);

    commands
        .spawn((
            PbrBundle {
                visibility: Visibility::Hidden,
                // TODO: Fix this to be mathematically accurate and not based on assumptions
                transform: transform
                    .with_rotation(Quat::from_rotation_x(-1.) * Quat::from_rotation_y(-0.8)),
                ..Default::default()
            },
            Rotator,
        ))
        .with_children(|parent| {
            for x in -I_GRID_COUNT..I_GRID_COUNT {
                for y in -I_GRID_COUNT..I_GRID_COUNT {
                    for z in -I_GRID_COUNT..I_GRID_COUNT {
                        let x = x as f32;
                        let y = y as f32;
                        let z = z as f32;

                        let r = x / GRID_COUNT;
                        let g = y / GRID_COUNT;
                        let b = z / GRID_COUNT;
                        parent.spawn(PbrBundle {
                            mesh: meshes.add(
                                Mesh::try_from(shape::Icosphere {
                                    radius: 0.25,
                                    ..Default::default()
                                })
                                .unwrap(),
                            ),
                            visibility: Visibility::Visible,
                            material: materials.add(Color::rgb(r, g, b).into()),
                            transform: Transform::from_xyz(x, y, z),
                            ..Default::default()
                        });
                    }
                }
            }
        });
}
