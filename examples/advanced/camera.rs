use crate::VoxelWorld;
use bevy::{
    app::AppExit,
    input::{keyboard::KeyCode, mouse::MouseMotion, ButtonInput},
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    window::CursorGrabMode,
};
use bevy_voxel_world::prelude::WorldVoxel;

#[derive(Component)]
pub struct WalkingCamera {
    pub speed: f32,
    pub sensitivity: f32,
    pub gravity: f32,
    pub jump_force: f32,
    pub is_grounded: bool,
    pub velocity: Vec3,
}

impl Default for WalkingCamera {
    fn default() -> Self {
        Self {
            speed: 50.0,
            sensitivity: 0.002,
            gravity: -9.8,
            jump_force: 15.0,
            is_grounded: false,
            velocity: Vec3::ZERO,
        }
    }
}

pub fn walking_camera<HighDetailWorld: bevy_voxel_world::prelude::VoxelWorldConfig>(
    time: Res<Time>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut WalkingCamera), With<Camera>>,
    voxel_world: VoxelWorld<HighDetailWorld>,
) {
    let (mut transform, mut camera) = query.single_mut();
    // Handle mouse look
    for ev in mouse_motion_events.read() {
        let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
        yaw -= ev.delta.x * camera.sensitivity;
        pitch -= ev.delta.y * camera.sensitivity;
        pitch = pitch.clamp(-1.54, 1.54); // Prevent camera from flipping
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    }
    // Handle keyboard input
    let mut input = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        input += transform.forward().as_vec3();
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        input -= transform.forward().as_vec3();
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        input -= transform.right().as_vec3();
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        input += transform.right().as_vec3();
    }
    // Remove vertical component for horizontal movement
    input.y = 0.0;
    input = input.normalize_or_zero();
    // Apply horizontal movement
    camera.velocity.x = input.x * camera.speed;
    camera.velocity.z = input.z * camera.speed;
    // Apply gravity
    if !camera.is_grounded {
        camera.velocity.y += camera.gravity * time.delta_seconds();
    }
    // Handle jumping
    if keyboard_input.pressed(KeyCode::Space) && camera.is_grounded {
        camera.velocity.y = camera.jump_force;
        camera.is_grounded = false;
    }
    // Move the camera
    let mut new_position = transform.translation + camera.velocity * time.delta_seconds();
    // Collision detection
    let feet_position = new_position - Vec3::new(0.0, 1.0, 0.0); // Assuming the camera is 2 units tall
    let head_position = new_position + Vec3::new(0.0, 1.0, 0.0);
    // Check for vertical collisions
    if matches!(
        voxel_world.get_voxel(feet_position.as_ivec3()),
        WorldVoxel::Solid(_)
    ) {
        new_position.y = feet_position.y.ceil() + 1.0; // Place the camera just above the ground
        camera.velocity.y = 0.0;
        camera.is_grounded = true;
    } else if matches!(
        voxel_world.get_voxel(head_position.as_ivec3()),
        WorldVoxel::Solid(_)
    ) {
        new_position.y = head_position.y.floor() - 1.0; // Place the camera just below the ceiling
        camera.velocity.y = 0.0;
    } else {
        camera.is_grounded = false;
    }
    // Horizontal collision
    let horizontal_movement =
        Vec3::new(camera.velocity.x, 0.0, camera.velocity.z) * time.delta_seconds();
    let check_positions = [
        new_position + Vec3::new(0.3, 0.0, 0.3),
        new_position + Vec3::new(0.3, 0.0, -0.3),
        new_position + Vec3::new(-0.3, 0.0, 0.3),
        new_position + Vec3::new(-0.3, 0.0, -0.3),
    ];
    for pos in check_positions.iter() {
        if matches!(voxel_world.get_voxel(pos.as_ivec3()), WorldVoxel::Solid(_)) {
            // If there's a collision, don't apply horizontal movement
            new_position -= horizontal_movement;
            break;
        }
    }
    transform.translation = new_position;
}

#[derive(Component)]
pub struct FlyCamera {
    speed: f32,
    sensitivity: f32,
}

impl Default for FlyCamera {
    fn default() -> Self {
        Self {
            speed: 50.0,
            sensitivity: 0.002,
        }
    }
}

pub fn fly_camera(
    time: Res<Time>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &FlyCamera), With<Camera>>,
) {
    let (mut transform, camera) = query.single_mut();

    // Handle mouse look
    for ev in mouse_motion_events.read() {
        let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
        yaw -= ev.delta.x * camera.sensitivity;
        pitch -= ev.delta.y * camera.sensitivity;
        pitch = pitch.clamp(-1.54, 1.54); // Prevent camera from flipping
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    }

    // Handle keyboard input
    let mut velocity = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        velocity += transform.forward().as_vec3();
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        velocity -= transform.forward().as_vec3();
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        velocity -= transform.right().as_vec3();
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        velocity += transform.right().as_vec3();
    }
    if keyboard_input.pressed(KeyCode::Space) {
        velocity += Vec3::Y;
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        velocity -= Vec3::Y;
    }

    transform.translation += velocity * camera.speed * time.delta_seconds();
}

pub fn grab_mouse(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

pub fn exit_on_esc(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::default());
    }
}
