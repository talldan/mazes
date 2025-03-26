use bevy::prelude::*;

mod components;
mod resources;
mod systems;

pub use crate::resources::MazeMap;
pub use crate::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MazeMap::Grid(5, 5))
        .add_systems(Startup, (setup_camera, setup_grid_points))
        .run();
}
