use bevy::prelude::*;
use bevy_voxel_world::prelude::*;

mod camera;
mod voxel;
mod config;
mod systems;

use camera::*;
use config::*;
use systems::*;

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