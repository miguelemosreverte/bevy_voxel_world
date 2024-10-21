// world_wrapper.rs
use crate::WalkingCamera;
use bevy::prelude::*;
use bevy_voxel_world::prelude::*;
use std::sync::{Arc, Mutex};

// Generic wrapper that includes an identifier and the world configuration
#[derive(Resource, Clone)]
pub struct WorldWrapper<C: VoxelWorldConfig + 'static> {
    pub id: String,
    pub config: C,
}

impl<C: VoxelWorldConfig + Default + 'static> Default for WorldWrapper<C> {
    fn default() -> Self {
        Self {
            id: String::new(), // You can set a default ID or leave it empty
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

fn get_camera_default<C: VoxelWorldConfig + 'static>() -> VoxelWorldCamera<WorldWrapper<C>> {
    VoxelWorldCamera::<WorldWrapper<C>>::default()
}

/// Factory function to create a VoxelWorldPlugin with a wrapped configuration and spawn a camera
pub fn create_world_plugin<C: VoxelWorldConfig + Default + 'static>(
    id: &str,
    config: C,
) -> impl Plugin {
    // Define a struct to encapsulate the composite plugin
    struct CompositePlugin<C: VoxelWorldConfig + Default + 'static> {
        wrapped_config: WorldWrapper<C>,
    }

    // Implement the Plugin trait for CompositePlugin
    impl<C: VoxelWorldConfig + Default + 'static> Plugin for CompositePlugin<C> {
        fn build(&self, app: &mut App) {
            // Add a startup system to spawn the camera for this voxel world
            app.add_systems(PreStartup, spawn_voxel_world_camera::<C>);
            // Add the VoxelWorldPlugin with the wrapped configuration
            app.add_plugins(VoxelWorldPlugin::<WorldWrapper<C>>::with_config(
                self.wrapped_config.clone(),
            ));
        }
    }

    // Define the startup system to spawn the camera
    fn spawn_voxel_world_camera<C: VoxelWorldConfig + 'static>(
        mut commands: Commands,
        world_wrapper: Res<WorldWrapper<C>>,
    ) {
        commands.spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 160.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            WalkingCamera::default(),
            VoxelWorldCamera::<WorldWrapper<C>>::default(),
        ));

        println!("Added camera for world {}", world_wrapper.id);
    }

    // Wrap the configuration with an identifier
    let wrapped_config = WorldWrapper {
        id: id.to_string(),
        config,
    };

    // Instantiate and return the CompositePlugin
    CompositePlugin { wrapped_config }
}
