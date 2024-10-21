use bevy::prelude::*;
use bevy::{
    app::AppExit,
    input::{keyboard::KeyCode, mouse::MouseMotion, ButtonInput},
    pbr::CascadeShadowConfigBuilder,
    utils::HashMap,
    window::CursorGrabMode,
};
use bevy_voxel_world::prelude::*;

use crate::camera;
use crate::voxel::get_voxel_fn;
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
    pub height_minus: f64,
    pub from: u32,
    pub to: u32,
}

impl HighDetailWorld {
    pub const fn new() -> Self {
        Self {
            scale: 1.0,
            height_scale: 1.0,
            height_minus: 0.0,
            from: 0,
            to: 5,
        }
    }
}

impl Default for HighDetailWorld {
    fn default() -> Self {
        Self {
            scale: 1.0,
            height_scale: 1.0,
            height_minus: 0.0,
            from: 0,
            to: 5,
        }
    }
}

impl VoxelWorldConfig for HighDetailWorld {
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
        Box::new(move |chunk_pos| {
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

// LowDetailWorld implementation
#[derive(Resource, Clone)]
pub struct LowDetailWorld {
    pub scale: f64,
    pub height_scale: f64,
    pub height_minus: f64,
    pub from: u32,
    pub to: u32,
}

impl Default for LowDetailWorld {
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

impl VoxelWorldConfig for LowDetailWorld {
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
        Box::new(move |chunk_pos| {
            let mut voxel_fn = get_voxel_fn(scale, height_scale, height_minus);
            Box::new(move |pos| voxel_fn(pos, 0))
        })
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

// Define a struct to wrap the VoxelWorldPlugin
pub struct WrappedVoxelWorldPlugin<C: VoxelWorldConfig + 'static> {
    id: String,
    config: C,
}

impl<C: VoxelWorldConfig + Default + 'static> Plugin for WrappedVoxelWorldPlugin<C> {
    fn build(&self, app: &mut App) {
        // Use the wrapped VoxelWorldPlugin to build the plugin within Bevy
        let wrapped_config = WorldWrapper {
            id: self.id.clone(),
            config: self.config.clone(),
        };
        VoxelWorldPlugin::<WorldWrapper<C>>::with_config(wrapped_config).build(app);
    }

    // Override the is_unique method to allow multiple instances
    fn is_unique(&self) -> bool {
        true
    }

    // Optional: You can override the name method to distinguish between instances
    fn name(&self) -> &str {
        &self.id
    }
}

pub fn create_world_plugin<C: VoxelWorldConfig + Default + 'static>(
    id: &str,
    config: C,
) -> impl Plugin {
    WrappedVoxelWorldPlugin {
        id: id.to_string(),
        config,
    }
}

// Define chunk stats text identifiers
#[derive(Component)]
enum ChunkStatsText {
    HighDetail,
    LowDetail,
}
