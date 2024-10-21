use bevy::prelude::*;
use bevy::{
    app::AppExit,
    input::{keyboard::KeyCode, ButtonInput},
    pbr::CascadeShadowConfigBuilder,
    window::CursorGrabMode,
};
use bevy_voxel_world::prelude::*;
use serde::Deserialize;

mod camera;
mod voxel;
use camera::*;
use voxel::get_voxel_fn;

#[derive(Resource, Clone, Deserialize, Default)]
struct VoxelWorldConfiguration {
    scale: f32,
    height_scale: f32,
    height_minus: f32,
    from: u32,
    to: u32,
}

macro_rules! impl_voxel_world_config {
    ($type:ident) => {
        #[derive(Resource, Clone, Default)]
        struct $type(VoxelWorldConfiguration);

        impl VoxelWorldConfig for $type {
            fn spawning_min_distance(&self) -> u32 {
                self.0.from
            }
            fn spawning_distance(&self) -> u32 {
                self.0.from
            }
            fn spawning_max_distance(&self) -> u32 {
                self.0.to
            }
            fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate {
                let scale = self.0.scale as f64;
                let height_scale = self.0.height_scale as f64;
                let height_minus = self.0.height_minus as f64;
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
    };
}

impl_voxel_world_config!(HighDetailWorld);
impl_voxel_world_config!(LowDetailWorld1);
impl_voxel_world_config!(LowDetailWorld2);
impl_voxel_world_config!(LowDetailWorld3);
impl_voxel_world_config!(LowDetailWorld4);

#[derive(Deserialize)]
struct WorldConfigurations {
    high_detail: VoxelWorldConfiguration,
    low_detail: Vec<VoxelWorldConfiguration>,
}

fn load_configurations() -> WorldConfigurations {
    let config_str =
        std::fs::read_to_string("voxel_configs.yaml").expect("Failed to read config file");
    serde_yaml::from_str(&config_str).expect("Failed to parse YAML")
}

fn main() {
    let configs = load_configurations();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_plugins(VoxelWorldPlugin::<HighDetailWorld>::with_config(
        HighDetailWorld(configs.high_detail),
    ));

    app.add_plugins(VoxelWorldPlugin::<LowDetailWorld1>::with_config(
        LowDetailWorld1(configs.low_detail[0].clone()),
    ));
    app.add_plugins(VoxelWorldPlugin::<LowDetailWorld2>::with_config(
        LowDetailWorld2(configs.low_detail[1].clone()),
    ));
    app.add_plugins(VoxelWorldPlugin::<LowDetailWorld3>::with_config(
        LowDetailWorld3(configs.low_detail[2].clone()),
    ));
    app.add_plugins(VoxelWorldPlugin::<LowDetailWorld4>::with_config(
        LowDetailWorld4(configs.low_detail[3].clone()),
    ));

    app.add_systems(Startup, (setup, grab_mouse))
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
