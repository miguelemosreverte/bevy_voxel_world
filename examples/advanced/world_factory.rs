// world_factory.rs

use bevy::prelude::*;
use bevy_voxel_world::prelude::*;
use std::any::Any;
use std::sync::{Arc, Mutex};

// Import your world configurations
use crate::world::{HighDetailWorld, LowDetailWorld};

// Wrapper structs
#[derive(Resource, Clone)]
pub struct HighDetailWorldWrapper(pub HighDetailWorld);

#[derive(Resource, Clone)]
pub struct LowDetailWorldWrapper(pub LowDetailWorld);

// Implement Default for wrapper structs
impl Default for HighDetailWorldWrapper {
    fn default() -> Self {
        HighDetailWorldWrapper(HighDetailWorld::default())
    }
}

impl Default for LowDetailWorldWrapper {
    fn default() -> Self {
        LowDetailWorldWrapper(LowDetailWorld::default())
    }
}

// Custom plugins
pub struct HighDetailWorldPlugin;
pub struct LowDetailWorldPlugin;

impl Plugin for HighDetailWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VoxelWorldPlugin::<HighDetailWorldWrapper>)
    }
}

impl Plugin for LowDetailWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VoxelWorldPlugin::<LowDetailWorldWrapper>);
    }
}

// Implement VoxelWorldConfig for wrapper structs
impl VoxelWorldConfig for HighDetailWorldWrapper {
    fn spawning_min_distance(&self) -> u32 {
        self.0.spawning_min_distance()
    }

    fn spawning_max_distance(&self) -> u32 {
        self.0.spawning_max_distance()
    }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate {
        self.0.voxel_lookup_delegate()
    }

    fn chunk_despawn_strategy(&self) -> ChunkDespawnStrategy {
        self.0.chunk_despawn_strategy()
    }

    fn chunk_spawn_strategy(&self) -> ChunkSpawnStrategy {
        self.0.chunk_spawn_strategy()
    }

    fn debug_draw_chunks(&self) -> bool {
        self.0.debug_draw_chunks()
    }
}

impl VoxelWorldConfig for LowDetailWorldWrapper {
    fn spawning_min_distance(&self) -> u32 {
        self.0.spawning_min_distance()
    }

    fn spawning_max_distance(&self) -> u32 {
        self.0.spawning_max_distance()
    }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate {
        self.0.voxel_lookup_delegate()
    }

    fn chunk_despawn_strategy(&self) -> ChunkDespawnStrategy {
        self.0.chunk_despawn_strategy()
    }

    fn chunk_spawn_strategy(&self) -> ChunkSpawnStrategy {
        self.0.chunk_spawn_strategy()
    }

    fn debug_draw_chunks(&self) -> bool {
        self.0.debug_draw_chunks()
    }
}

// Trait to allow downcasting
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Factory function to create world plugins
pub fn create_world_plugin(world_type: &str, config: Box<dyn AsAny>) -> Box<dyn Plugin> {
    match world_type {
        "high_detail" => {
            let high_detail_world = config.as_any().downcast_ref::<HighDetailWorld>().unwrap();
            Box::new(VoxelWorldPlugin::<HighDetailWorldWrapper>::with_config(
                HighDetailWorldWrapper(high_detail_world.clone()),
            ))
        }
        "low_detail" => {
            let low_detail_world = config.as_any().downcast_ref::<LowDetailWorld>().unwrap();
            Box::new(VoxelWorldPlugin::<LowDetailWorldWrapper>::with_config(
                LowDetailWorldWrapper(low_detail_world.clone()),
            ))
        }
        _ => panic!("Unknown world type"),
    }
}
