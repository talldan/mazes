use bevy::prelude::*;

mod components;
mod maze_builders;
mod resources;
mod systems;
mod utils;

pub use crate::resources::*;
pub use crate::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GridMap::new(15, 15))
        .add_systems(Startup, (setup_camera, draw_grid_map))
        .run();
}
