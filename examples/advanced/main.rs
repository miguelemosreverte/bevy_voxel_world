use bevy::prelude::*;
use bevy::{
    app::AppExit,
    input::{keyboard::KeyCode, ButtonInput},
    pbr::CascadeShadowConfigBuilder,
    window::CursorGrabMode,
};
use bevy_fn_plugin::bevy_plugin;
use bevy_voxel_world::prelude::*;

mod camera;
mod voxel;
use camera::*;
use voxel::get_voxel_fn;

#[derive(Resource, Clone, Copy)]
struct VoxelWorldConfiguration {
    scale: f32,
    height_scale: f32,
    height_minus: f32,
    from: u32,
    to: u32,
}

const HIGH_DETAIL_CONFIG: VoxelWorldConfiguration = VoxelWorldConfiguration {
    scale: 1.0,
    height_scale: 1.0,
    height_minus: 1.0,
    from: 0,
    to: 3,
};

const LOW_DETAIL_CONFIGS: [VoxelWorldConfiguration; 4] = [
    VoxelWorldConfiguration {
        scale: 2.0,
        height_scale: 0.5,
        height_minus: 1.0,
        from: 4,
        to: 6,
    },
    VoxelWorldConfiguration {
        scale: 4.0,
        height_scale: 1.0,
        height_minus: 1.0,
        from: 7,
        to: 10,
    },
    VoxelWorldConfiguration {
        scale: 8.0,
        height_scale: 1.0,
        height_minus: 1.0,
        from: 11,
        to: 15,
    },
    VoxelWorldConfiguration {
        scale: 16.0,
        height_scale: 1.0,
        height_minus: 1.0,
        from: 15,
        to: 20,
    },
];

impl Default for VoxelWorldConfiguration {
    fn default() -> Self {
        Self {
            scale: 1.0,
            height_scale: 1.0,
            height_minus: 0.0,
            from: 6,
            to: 7,
        }
    }
}

impl VoxelWorldConfig for VoxelWorldConfiguration {
    fn spawning_min_distance(&self) -> u32 {
        self.from
    }
    fn spawning_distance(&self) -> u32 {
        self.from
    }
    fn spawning_max_distance(&self) -> u32 {
        self.to
    }
    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate {
        let scale = self.scale as f64;
        let height_scale = self.height_scale as f64;
        let height_minus = self.height_minus as f64;
        Box::new(move |_chunk_pos| {
            let mut voxel_fn = get_voxel_fn(scale, height_scale, height_minus);
            Box::new(move |pos| voxel_fn(pos, 0))
        })
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

fn create_world_plugin(_name: &str, config: VoxelWorldConfiguration) -> impl Plugin {
    VoxelWorldPlugin::with_config(config)
}

// High detail world plugin
#[bevy_plugin]
fn HighDetailWorldPlugin(app: &mut App) {
    app.add_plugins(VoxelWorldPlugin::<VoxelWorldConfiguration>::with_config(
        HIGH_DETAIL_CONFIG,
    ));
}

// Low detail world plugins
#[bevy_plugin]
fn LowDetailWorld1Plugin(app: &mut App) {
    app.add_plugins(create_world_plugin("low_detail_1", LOW_DETAIL_CONFIGS[0]));
}

#[bevy_plugin]
fn LowDetailWorld2Plugin(app: &mut App) {
    app.add_plugins(create_world_plugin("low_detail_2", LOW_DETAIL_CONFIGS[1]));
}

#[bevy_plugin]
fn LowDetailWorld3Plugin(app: &mut App) {
    app.add_plugins(create_world_plugin("low_detail_3", LOW_DETAIL_CONFIGS[2]));
}

#[bevy_plugin]
fn LowDetailWorld4Plugin(app: &mut App) {
    app.add_plugins(create_world_plugin("low_detail_4", LOW_DETAIL_CONFIGS[3]));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HighDetailWorldPlugin)
        .add_plugins(LowDetailWorld1Plugin)
        .add_plugins(LowDetailWorld2Plugin)
        .add_plugins(LowDetailWorld3Plugin)
        .add_plugins(LowDetailWorld4Plugin)
        .add_systems(Startup, (setup, grab_mouse))
        .add_systems(Update, (fly_camera, exit_on_esc))
        .insert_resource(ClearColor(Color::srgb(0.5, 0.8, 1.0)))
        .run();
}

fn setup(mut commands: Commands) {
    let camera_entity = commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 160.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            FlyCamera::default(),
        ))
        .id();

    commands
        .entity(camera_entity)
        .insert(VoxelWorldCamera::<VoxelWorldConfiguration>::default());

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

fn grab_mouse(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.cursor.visible = false;
}

fn exit_on_esc(keyboard: Res<ButtonInput<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}
