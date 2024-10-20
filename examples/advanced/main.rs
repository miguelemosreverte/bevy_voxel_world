// main.rs
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

// Import your voxel and world configurations
mod camera;
mod voxel;
mod world;

use camera::{exit_on_esc, grab_mouse, walking_camera, WalkingCamera};
use world::{HighDetailWorld, LowDetailWorld};

// Define chunk stats text identifiers
#[derive(Component)]
enum ChunkStatsText {
    HighDetail,
    LowDetail,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the High-Detail Voxel World
        .add_plugins(VoxelWorldPlugin::<HighDetailWorld>::with_config(
            HighDetailWorld::default(),
        ))
        // Add the Low-Detail Voxel World
        .add_plugins(VoxelWorldPlugin::<LowDetailWorld>::with_config(
            LowDetailWorld::default(),
        ))
        .add_systems(Startup, (setup, grab_mouse))
        //.add_startup_system(setup_ui)
        .add_systems(
            Update,
            (
                walking_camera,
                exit_on_esc,
                // Update camera positions for both worlds
                //update_camera_position::<HighDetailWorld>,
                //update_camera_position::<LowDetailWorld>,
                // Handle spawning and despawning
                //Internals::<HighDetailWorld>::spawn_chunks
                //    .before(Internals::<LowDetailWorld>::spawn_chunks),
                //Internals::<HighDetailWorld>::retire_chunks,
                //Internals::<LowDetailWorld>::spawn_chunks,
                //Internals::<LowDetailWorld>::retire_chunks,
                // Update UI stats
                //update_chunk_stats::<HighDetailWorld>,
                //update_chunk_stats::<LowDetailWorld>,
                // Debug Visualization
                //Internals::<HighDetailWorld>::draw_chunk_bounding_boxes,
                // Internals::<LowDetailWorld>::draw_spawn_ranges, // Optional
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Set background color
    commands.insert_resource(ClearColor(Color::rgb(0.5, 0.8, 1.0)));

    // Spawn Camera for High-Detail and Low-Detail Worlds
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 160.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        WalkingCamera::default(),
        VoxelWorldCamera::<HighDetailWorld>::default(),
        VoxelWorldCamera::<LowDetailWorld>::default(),
    ));

    // Spawn Directional Light
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

    // Insert Ambient Light
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.98, 0.95, 0.82),
        brightness: 100.0,
    });
}
