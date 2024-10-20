use bevy::prelude::*;
use bevy::{
    app::AppExit,
    input::{keyboard::KeyCode, mouse::MouseMotion, ButtonInput},
    pbr::CascadeShadowConfigBuilder,
    utils::HashMap,
    window::CursorGrabMode,
};
use bevy_voxel_world::prelude::*;

mod voxel;

use voxel::get_voxel_fn;

// WorldWrapper struct
#[derive(Resource, Clone)]
pub struct WorldWrapper<C: VoxelWorldConfig + 'static> {
    pub id: String,
    pub config: C,
}

impl<C: VoxelWorldConfig + Default + 'static> Default for WorldWrapper<C> {
    fn default() -> Self {
        Self {
            id: String::new(),
            config: C::default(),
        }
    }
}

impl<C: VoxelWorldConfig + 'static> VoxelWorldConfig for WorldWrapper<C> {
    fn spawning_min_distance(&self) -> u32 {
        self.config.spawning_min_distance()
    }
    fn spawning_max_distance(&self) -> u32 {
        self.config.spawning_max_distance()
    }
    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate {
        self.config.voxel_lookup_delegate()
    }
    fn chunk_despawn_strategy(&self) -> ChunkDespawnStrategy {
        self.config.chunk_despawn_strategy()
    }
    fn chunk_spawn_strategy(&self) -> ChunkSpawnStrategy {
        self.config.chunk_spawn_strategy()
    }
    fn debug_draw_chunks(&self) -> bool {
        self.config.debug_draw_chunks()
    }
}

// HighDetailWorld implementation
#[derive(Resource, Clone)]
pub struct HighDetailWorld {
    pub scale: f64,
    pub height_scale: f64,
}

impl HighDetailWorld {
    pub const fn new() -> Self {
        Self {
            scale: 1.0,
            height_scale: 1.0,
        }
    }
}

impl Default for HighDetailWorld {
    fn default() -> Self {
        Self {
            scale: 1.0,
            height_scale: 1.0,
        }
    }
}

impl VoxelWorldConfig for HighDetailWorld {
    fn spawning_min_distance(&self) -> u32 {
        0
    }
    fn spawning_max_distance(&self) -> u32 {
        4
    }
    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate {
        let scale = self.scale as f32;
        let height_scale = self.height_scale as f32;
        Box::new(move |pos| Box::new(move |pos| get_voxel_fn(1.0, 1.0, 0.0)(pos, 0)))
    }
    fn chunk_despawn_strategy(&self) -> ChunkDespawnStrategy {
        ChunkDespawnStrategy::Distance(7)
    }
    fn chunk_spawn_strategy(&self) -> ChunkSpawnStrategy {
        ChunkSpawnStrategy::Distance(5)
    }
    fn debug_draw_chunks(&self) -> bool {
        false
    }
}

// LowDetailWorld implementation
#[derive(Resource, Clone)]
pub struct LowDetailWorld {
    pub scale: f64,
    pub height_scale: f64,
    pub height_minus: f64,
}

impl Default for LowDetailWorld {
    fn default() -> Self {
        Self {
            scale: 1.0,
            height_scale: 1.0,
            height_minus: 0.0,
        }
    }
}

impl VoxelWorldConfig for LowDetailWorld {
    fn spawning_min_distance(&self) -> u32 {
        1
    }
    fn spawning_max_distance(&self) -> u32 {
        3
    }
    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate {
        let scale = self.scale as f32;
        let height_scale = self.height_scale as f32;
        Box::new(move |pos| Box::new(move |pos| get_voxel_fn(1.0, 1.0, 0.0)(pos, 0)))
    }
    fn chunk_despawn_strategy(&self) -> ChunkDespawnStrategy {
        ChunkDespawnStrategy::Distance(5)
    }
    fn chunk_spawn_strategy(&self) -> ChunkSpawnStrategy {
        ChunkSpawnStrategy::Distance(3)
    }
    fn debug_draw_chunks(&self) -> bool {
        false
    }
}

// Helper function for noise generation
fn noise_2d(x: f32, y: f32) -> f32 {
    (x.sin() * y.cos() + y.sin() * x.cos()).sin() * 0.5
}

// WalkingCamera component
#[derive(Component)]
pub struct WalkingCamera {
    pub speed: f32,
    pub sensitivity: f32,
}

impl Default for WalkingCamera {
    fn default() -> Self {
        Self {
            speed: 12.0,
            sensitivity: 0.00012,
        }
    }
}

// Plugin creation helper
fn create_world_plugin<C: VoxelWorldConfig + Default + 'static>(
    id: &str,
    config: C,
) -> impl Plugin {
    let wrapped_config = WorldWrapper {
        id: id.to_string(),
        config,
    };
    VoxelWorldPlugin::<WorldWrapper<C>>::with_config(wrapped_config)
}

// Define chunk stats text identifiers
#[derive(Component)]
enum ChunkStatsText {
    HighDetail,
    LowDetail,
}

fn main() {
    // Create the plugins
    let high_detail_world =
        VoxelWorldPlugin::<HighDetailWorld>::with_config(HighDetailWorld::new());
    let low_detail_world = create_world_plugin("low_detail_1", LowDetailWorld::default());

    // Create a vector of boxed plugins
    let PLUGINS = vec![low_detail_world];
    fn setup(mut commands: Commands) {
        // Set background color
        commands.insert_resource(ClearColor(Color::rgb(0.5, 0.8, 1.0)));
        let camera_entity = commands
            .spawn((
                Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 160.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                },
                WalkingCamera::default(),
            ))
            .id();
        // Spawn Camera for High-Detail and Low-Detail Worlds
        commands
            .entity(camera_entity)
            .insert(VoxelWorldCamera::<HighDetailWorld>::default())
            .insert(VoxelWorldCamera::<WorldWrapper<LowDetailWorld>>::default());

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

    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, grab_mouse))
        .add_systems(Update, (walking_camera, exit_on_esc));

    app.add_plugins(high_detail_world);

    for plugin in PLUGINS.into_iter() {
        //app.add_plugins(plugin);
    }

    app.run();
}

// Camera control systems
fn grab_mouse(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.cursor.visible = false;
}

fn walking_camera(
    time: Res<Time>,
    mut camera_query: Query<(&WalkingCamera, &mut Transform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    for (camera, mut transform) in camera_query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for key in keyboard.get_pressed() {
            match key {
                KeyCode::KeyW => velocity += forward,
                KeyCode::KeyS => velocity -= forward,
                KeyCode::KeyA => velocity -= right,
                KeyCode::KeyD => velocity += right,
                KeyCode::Space => velocity += Vec3::Y,
                KeyCode::ShiftLeft => velocity -= Vec3::Y,
                _ => (),
            }
        }

        velocity = velocity.normalize_or_zero();
        transform.translation += velocity * time.delta_seconds() * camera.speed;

        let mut mouse_delta = Vec2::ZERO;
        for event in mouse_motion.read() {
            mouse_delta += event.delta;
        }
        if mouse_delta != Vec2::ZERO {
            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            yaw -= mouse_delta.x * camera.sensitivity;
            pitch -= mouse_delta.y * camera.sensitivity;
            pitch = pitch.clamp(-1.54, 1.54);
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
        }
    }
}

fn exit_on_esc(keyboard: Res<ButtonInput<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}
