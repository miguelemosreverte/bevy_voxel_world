use bevy::prelude::*;
use bevy_voxel_world::prelude::*;
use noise::{HybridMulti, NoiseFn, Perlin};
use std::collections::HashMap;

pub fn get_voxel_fn(
    scale: f64,
    height_scale: f64,
    height_minus: f64
) -> Box<dyn FnMut(IVec3, u8) -> WorldVoxel + Send + Sync> {
    let mut noise = HybridMulti::<Perlin>::new(1234);
    noise.octaves = 5;
    noise.frequency = 1.1;
    noise.lacunarity = 2.8;
    noise.persistence = 0.4;

    let mut cache = HashMap::<(i32, i32), f64>::new();
    let mut canopy_positions = HashMap::<(i32, i32), i32>::new(); // Track positions for canopies

    Box::new(move |pos: IVec3, lod_level: u8| {
        if pos.y < 1 {
            return WorldVoxel::Solid(3); // Sea level voxel
        }

        let [x, y, z] = pos.as_dvec3().to_array();
        let scaled_x = x / (1000.0 / scale);
        let scaled_z = z / (1000.0 / scale);
        let y_i32 = y as i32; // Cast y to i32 for comparison

        let ground_height = match cache.get(&(pos.x, pos.z)) {
            Some(sample) => *sample,
            None => {
                let sample = noise.get([scaled_x, scaled_z]) * 50.0 * height_scale - height_minus;
                cache.insert((pos.x, pos.z), sample);
                sample
            }
        };

        // Step 1: Check for canopy positions around the tree trunk
        let canopy_offsets = vec![
            (0, 0),   // Directly above the trunk
            (1, 0),   // To the east
            (-1, 0),  // To the west
            (0, 1),   // To the north
            (0, -1),  // To the south
            (1, 1),   // North-east
            (-1, 1),  // North-west
            (1, -1),  // South-east
            (-1, -1), // South-west
        ];

        for (dx, dz) in canopy_offsets.iter() {
            if let Some(canopy_base) = canopy_positions.get(&(pos.x + dx, pos.z + dz)) {
                if y_i32 >= *canopy_base && y_i32 <= *canopy_base + 3 {
                    return WorldVoxel::Solid(1); // Canopy material (greenery)
                }
            }
        }

        // Step 2: Place tree trunks and record positions for canopy placement
        if y < ground_height {
            WorldVoxel::Solid(0) // Ground material
        } else if y < ground_height + 5.0 && ground_height > 5.0 && y > 5.0 {
            // Ensure trees spawn with at least 5 blocks of distance between each other
            if (pos.x % 5 == 0) && (pos.z % 5 == 0) {
                let tree_height = 5; // Fixed tree height for trunk
                let tree_top_height = ground_height + tree_height as f64;

                if y < tree_top_height {
                    // Record this position as the top of the tree trunk for canopy placement
                    canopy_positions.insert((pos.x, pos.z), y_i32 + 1); // Canopy starts at tree top + 1
                    WorldVoxel::Solid(2) // Tree trunk material
                } else {
                    WorldVoxel::Air
                }
            } else {
                WorldVoxel::Air
            }
        } else {
            WorldVoxel::Air
        }
    })
}
