use bevy::prelude::*;
use bevy_voxel_world::prelude::*;
use serde::Deserialize;
use crate::voxel::get_voxel_fn;

#[derive(Resource, Clone, Deserialize, Default)]
pub struct VoxelWorldConfiguration {
    pub scale: f32,
    pub height_scale: f32,
    pub height_minus: f32,
    pub from: u32,
    pub to: u32,
}

macro_rules! impl_voxel_world_config {
    ($type:ident) => {
        #[derive(Resource, Clone, Default)]
        pub struct $type(pub VoxelWorldConfiguration);

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
pub struct WorldConfigurations {
    pub high_detail: VoxelWorldConfiguration,
    pub low_detail: Vec<VoxelWorldConfiguration>,
}

pub fn load_configurations() -> WorldConfigurations {
    let config_str =
        std::fs::read_to_string("voxel_configs.yaml").expect("Failed to read config file");
    serde_yaml::from_str(&config_str).expect("Failed to parse YAML")
}