// world.rs
use bevy::prelude::*;
use bevy_voxel_world::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Resource, Clone)]
pub struct HighDetailWorld {
    pub scale: f64,
    pub height_scale: f64,
    pub camera_position: Arc<Mutex<Vec3>>,
}

impl Default for HighDetailWorld {
    fn default() -> Self {
        Self {
            scale: 1.0, // Larger scale for higher detail
            height_scale: 0.5,
            camera_position: Arc::new(Mutex::new(Vec3::ZERO)),
        }
    }
}

impl VoxelWorldConfig for HighDetailWorld {
    /// Minimum distance in chunks to start spawning high-detail chunks
    fn spawning_min_distance(&self) -> u32 {
        0
    }

    /// Maximum distance in chunks to spawn high-detail chunks around the camera
    fn spawning_max_distance(&self) -> u32 {
        (11.0) as u32
    }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate {
        let scale = self.scale;
        let height_scale = self.height_scale;

        Box::new(move |chunk_pos| {
            let mut voxel_fn = crate::voxel::get_voxel_fn(scale, height_scale);
            Box::new(move |local_pos: IVec3| {
                // Adjust block density based on scale
                let scaled_pos = local_pos / scale as i32;
                voxel_fn(scaled_pos, scale as u8)
            })
        })
    }

    fn chunk_despawn_strategy(&self) -> ChunkDespawnStrategy {
        ChunkDespawnStrategy::FarAway
    }

    fn chunk_spawn_strategy(&self) -> ChunkSpawnStrategy {
        ChunkSpawnStrategy::CloseAndInView
    }

    fn debug_draw_chunks(&self) -> bool {
        true // Enable debug visualization for high-detail chunks
    }
}

// world.rs (continued)
#[derive(Resource, Clone)]
pub struct LowDetailWorld {
    pub scale: f64,
    pub height_scale: f64,
    pub camera_position: Arc<Mutex<Vec3>>,
}

impl Default for LowDetailWorld {
    fn default() -> Self {
        Self {
            scale: 2.0, // Smaller scale for lower detail
            height_scale: 0.1,
            camera_position: Arc::new(Mutex::new(Vec3::ZERO)),
        }
    }
}

impl VoxelWorldConfig for LowDetailWorld {
    /// Minimum distance in chunks to start spawning low-detail chunks
    fn spawning_min_distance(&self) -> u32 {
        (11.0) as u32 // Start beyond high-detail range
    }

    /// Maximum distance in chunks to spawn low-detail chunks around the camera
    fn spawning_max_distance(&self) -> u32 {
        (50.0) as u32
    }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate {
        let scale = self.scale;
        let height_scale = self.height_scale;

        Box::new(move |chunk_pos| {
            let mut voxel_fn = crate::voxel::get_voxel_fn(scale, height_scale);
            Box::new(move |local_pos: IVec3| {
                // Adjust block density based on scale
                let scaled_pos = local_pos / scale as i32;
                voxel_fn(scaled_pos, scale as u8)
            })
        })
    }

    fn chunk_despawn_strategy(&self) -> ChunkDespawnStrategy {
        ChunkDespawnStrategy::FarAway
    }

    fn chunk_spawn_strategy(&self) -> ChunkSpawnStrategy {
        ChunkSpawnStrategy::CloseAndInView
    }

    fn debug_draw_chunks(&self) -> bool {
        false // Disable debug visualization for low-detail chunks
    }
}
