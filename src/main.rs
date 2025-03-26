use bevy::prelude::*;

mod components;
mod resources;
mod systems;

pub use crate::resources::*;
pub use crate::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GridMap::new(Vec2 { x: 5.0, y: 5.0 }))
        .add_systems(Startup, (setup_camera, draw_grid_map))
        .run();
}
