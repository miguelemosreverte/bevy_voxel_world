use bevy::prelude::*;
use bevy_voxel_world::prelude::*;
use noise::{HybridMulti, NoiseFn, Perlin};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

const CHUNK_SIZE: i32 = 16;

#[derive(Resource, Clone)]
pub struct MainWorld {
    pub scale: f64,
    pub height_scale: f64,
    pub camera_position: Arc<Mutex<Vec3>>,
}

impl Default for MainWorld {
    fn default() -> Self {
        Self {
            scale: 2.0,
            height_scale: 0.5,
            camera_position: Arc::new(Mutex::new(Vec3::ZERO)),
        }
    }
}

impl VoxelWorldConfig for MainWorld {
    fn spawning_distance(&self) -> u32 {
        (25.0 * self.scale) as u32
    }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate {
        let scale = self.scale;
        let height_scale = self.height_scale;

        Box::new(move |chunk_pos| {
            let mut voxel_fn = crate::voxel::get_voxel_fn(scale, height_scale);
            Box::new(move |local_pos: IVec3| {
                let lod_level = 1;
                voxel_fn(local_pos, lod_level)
            })
        })
    }

    fn chunk_despawn_strategy(&self) -> ChunkDespawnStrategy {
        ChunkDespawnStrategy::FarAway
    }

    fn chunk_spawn_strategy(&self) -> ChunkSpawnStrategy {
        ChunkSpawnStrategy::CloseAndInView
    }
}
