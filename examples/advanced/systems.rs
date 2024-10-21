use crate::camera::FlyCamera;
use crate::config::*;
use bevy::prelude::*;
use bevy::{
    app::AppExit,
    input::{keyboard::KeyCode, ButtonInput},
    pbr::CascadeShadowConfigBuilder,
    window::CursorGrabMode,
};
use bevy_voxel_world::prelude::*;

pub fn setup(mut commands: Commands) {
    let camera_entity = commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 30.0, 0.0)
                    .looking_at(Vec3::new(185.0, 0.0, 0.0), Vec3::Y),
                ..default()
            },
            FlyCamera::default(),
        ))
        .id();

    commands
        .entity(camera_entity)
        .insert(VoxelWorldCamera::<HighDetailWorld>::default())
        .insert(VoxelWorldCamera::<LowDetailWorld1>::default())
        .insert(VoxelWorldCamera::<LowDetailWorld2>::default())
        .insert(VoxelWorldCamera::<LowDetailWorld3>::default())
        .insert(VoxelWorldCamera::<LowDetailWorld4>::default());

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

pub fn grab_mouse(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.cursor.visible = false;
}

pub fn exit_on_esc(keyboard: Res<ButtonInput<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}
