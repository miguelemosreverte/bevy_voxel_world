use bevy::{
    app::AppExit,
    input::{keyboard::KeyCode, mouse::MouseMotion, ButtonInput},
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    utils::HashMap,
    window::CursorGrabMode,
};

use bevy_voxel_world::prelude::*;
use noise::{HybridMulti, NoiseFn, Perlin};

#[derive(Resource, Clone)]
struct MainWorld {
    scale: f64,        // Horizontal scale
    height_scale: f64, // Vertical/height scale
}

impl Default for MainWorld {
    fn default() -> Self {
        Self {
            scale: 2.0,        // Default horizontal scale
            height_scale: 0.5, // Default height scale
        }
    }
}

impl VoxelWorldConfig for MainWorld {
    fn spawning_distance(&self) -> u32 {
        (25.0 * self.scale) as u32 // Adjust spawning distance based on scale
    }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate {
        let scale = self.scale;
        let height_scale = self.height_scale; // Capture both scales
        Box::new(move |_chunk_pos| get_voxel_fn(scale, height_scale))
    }

    /// Strategy for despawning chunks
    fn chunk_despawn_strategy(&self) -> ChunkDespawnStrategy {
        ChunkDespawnStrategy::FarAway
    }

    /// Strategy for spawning chunks
    /// This is only used if the despawn strategy is `FarAway`
    fn chunk_spawn_strategy(&self) -> ChunkSpawnStrategy {
        ChunkSpawnStrategy::CloseAndInView
    }
}

fn get_voxel_fn(
    scale: f64,
    height_scale: f64,
) -> Box<dyn FnMut(IVec3) -> WorldVoxel + Send + Sync> {
    let mut noise = HybridMulti::<Perlin>::new(1234);
    noise.octaves = 5;
    noise.frequency = 1.1;
    noise.lacunarity = 2.8;
    noise.persistence = 0.4;

    let mut cache = HashMap::<(i32, i32), f64>::new();
    let mut canopy_positions = HashMap::<(i32, i32), i32>::new(); // Track positions for canopies

    Box::new(move |pos: IVec3| {
        if pos.y < 1 {
            return WorldVoxel::Solid(3); // Sea level voxel
        }

        let [x, y, z] = pos.as_dvec3().to_array();
        let scaled_x = x / (1000.0 / scale);
        let scaled_z = z / (1000.0 / scale);
        let y_i32 = y as i32; // Cast y to i32 for comparison

        let ground_height = match cache.get(&(pos.x, pos.z)) {
            Some(sample) => *sample,
            None => {
                let sample = noise.get([scaled_x, scaled_z]) * 50.0 * height_scale;
                cache.insert((pos.x, pos.z), sample);
                sample
            }
        };

        // Step 1: Check for canopy positions around the tree trunk
        let canopy_offsets = vec![
            (0, 0),   // Directly above the trunk
            (1, 0),   // To the east
            (-1, 0),  // To the west
            (0, 1),   // To the north
            (0, -1),  // To the south
            (1, 1),   // North-east
            (-1, 1),  // North-west
            (1, -1),  // South-east
            (-1, -1), // South-west
        ];

        for (dx, dz) in canopy_offsets.iter() {
            if let Some(canopy_base) = canopy_positions.get(&(pos.x + dx, pos.z + dz)) {
                if y_i32 >= *canopy_base && y_i32 <= *canopy_base + 3 {
                    return WorldVoxel::Solid(1); // Canopy material (greenery)
                }
            }
        }

        // Step 2: Place tree trunks and record positions for canopy placement
        if y < ground_height {
            WorldVoxel::Solid(0) // Ground material
        } else if y < ground_height + 5.0 && ground_height > 5.0 && y > 5.0 {
            // Ensure trees spawn with at least 5 blocks of distance between each other
            if (pos.x % 5 == 0) && (pos.z % 5 == 0) {
                let tree_height = 5; // Fixed tree height for trunk
                let tree_top_height = ground_height + tree_height as f64;

                if y < tree_top_height {
                    // Record this position as the top of the tree trunk for canopy placement
                    canopy_positions.insert((pos.x, pos.z), y_i32 + 1); // Canopy starts at tree top + 1
                    WorldVoxel::Solid(2) // Tree trunk material
                } else {
                    WorldVoxel::Air
                }
            } else {
                WorldVoxel::Air
            }
        } else {
            WorldVoxel::Air
        }
    })
}

#[derive(Component)]
struct WalkingCamera {
    speed: f32,
    sensitivity: f32,
    gravity: f32,
    jump_force: f32,
    is_grounded: bool,
    velocity: Vec3,
}

impl Default for WalkingCamera {
    fn default() -> Self {
        Self {
            speed: 5.0,
            sensitivity: 0.002,
            gravity: -9.8,
            jump_force: 5.0,
            is_grounded: false,
            velocity: Vec3::ZERO,
        }
    }
}

#[derive(Component)]
struct FlyCamera {
    speed: f32,
    sensitivity: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VoxelWorldPlugin::with_config(MainWorld::default()))
        .add_systems(Startup, (setup, grab_mouse))
        //.add_systems(Update, fly_camera)
        .add_systems(Update, (walking_camera, exit_on_esc))
        .run();
}

fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    // Set the sky color to a brighter blue
    *clear_color = ClearColor(Color::rgb(0.5, 0.8, 1.0)); // Light blue sky

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 160.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        WalkingCamera::default(),
        VoxelWorldCamera::<MainWorld>::default(),
    ));

    // Sun
    let cascade_shadow_config = CascadeShadowConfigBuilder { ..default() }.build();
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

    // Ambient light, same color as sun
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.98, 0.95, 0.82),
        brightness: 100.0,
    });
}

fn walking_camera(
    time: Res<Time>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut WalkingCamera), With<Camera>>,
    mut voxel_world: VoxelWorld<MainWorld>,
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

fn fly_camera(
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

fn grab_mouse(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

fn exit_on_esc(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::default());
    }
}
