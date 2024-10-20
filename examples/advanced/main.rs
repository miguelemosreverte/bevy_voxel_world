use bevy::prelude::*;
use bevy_voxel_world::prelude::*;

use bevy::{
    app::AppExit,
    input::{keyboard::KeyCode, mouse::MouseMotion, ButtonInput},
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    utils::HashMap,
    window::CursorGrabMode,
};

mod camera;
mod voxel;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VoxelWorldPlugin::with_config(world::MainWorld::default()))
        .add_systems(Startup, (setup, camera::grab_mouse))
        .add_systems(Update, (camera::walking_camera, camera::exit_on_esc))
        .add_systems(Update, update_camera_position)
        .run();
}

fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    *clear_color = ClearColor(Color::rgb(0.5, 0.8, 1.0));

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 160.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        camera::WalkingCamera::default(),
        VoxelWorldCamera::<world::MainWorld>::default(),
    ));

    let cascade_shadow_config = CascadeShadowConfigBuilder::default().build();
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::srgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::new(-0.15, -0.1, 0.15), Vec3::Y),
        cascade_shadow_config,
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.98, 0.95, 0.82),
        brightness: 100.0,
    });
}

fn update_camera_position(
    camera_query: Query<&Transform, With<Camera>>,
    mut main_world: ResMut<world::MainWorld>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        let mut camera_pos = main_world.camera_position.lock().unwrap();
        *camera_pos = camera_transform.translation;
    }
}
