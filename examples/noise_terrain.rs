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

#[derive(Resource, Clone, Default)]
struct MainWorld;

impl VoxelWorldConfig for MainWorld {
    fn spawning_distance(&self) -> u32 {
        25
    }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate {
        Box::new(move |_chunk_pos| get_voxel_fn())
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
        .add_plugins(VoxelWorldPlugin::with_config(MainWorld))
        .add_systems(Startup, (setup, grab_mouse))
        .add_systems(Update, (fly_camera, exit_on_esc))
        .run();
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-200.0, 180.0, -200.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FlyCamera {
            speed: 10.0,
            sensitivity: 0.002,
        },
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

fn get_voxel_fn() -> Box<dyn FnMut(IVec3) -> WorldVoxel + Send + Sync> {
    // Set up some noise to use as the terrain height map
    let mut noise = HybridMulti::<Perlin>::new(1234);
    noise.octaves = 5;
    noise.frequency = 1.1;
    noise.lacunarity = 2.8;
    noise.persistence = 0.4;

    // We use this to cache the noise value for each y column so we only need
    // to calculate it once per x/z coordinate
    let mut cache = HashMap::<(i32, i32), f64>::new();

    // Then we return this boxed closure that captures the noise and the cache
    // This will get sent off to a separate thread for meshing by bevy_voxel_world
    Box::new(move |pos: IVec3| {
        // Sea level
        if pos.y < 1 {
            return WorldVoxel::Solid(3);
        }

        let [x, y, z] = pos.as_dvec3().to_array();

        // If y is less than the noise sample, we will set the voxel to solid
        let is_ground = y < match cache.get(&(pos.x, pos.z)) {
            Some(sample) => *sample,
            None => {
                let sample = noise.get([x / 1000.0, z / 1000.0]) * 50.0;
                cache.insert((pos.x, pos.z), sample);
                sample
            }
        };

        if is_ground {
            // Solid voxel of material type 0
            WorldVoxel::Solid(0)
        } else {
            WorldVoxel::Air
        }
    })
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
