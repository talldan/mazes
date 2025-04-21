use bevy::{prelude::*, utils::HashSet};

mod components;
mod maze_builders;
mod resources;
mod systems;
mod utils;

pub use crate::resources::*;
pub use crate::systems::*;

fn main() {
    let rng_seed_changed = resource_changed::<RngSeed>.and(not(resource_added::<RngSeed>));
    let maze_builder_changed =
        resource_changed::<MazeBuilderType>.and(not(resource_added::<MazeBuilderType>));
    let grid_map_changed = resource_changed::<GridMap>.and(not(resource_added::<GridMap>));

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GridMap::new(15, 15))
        .insert_resource(RngSeed(0))
        .insert_resource(MazeBuilderType::AldousBroder)
        .insert_resource(OverlayState(false))
        .insert_resource(RemovedWalls(HashSet::new()))
        .add_systems(Startup, (setup_camera, setup_hud, setup_grid_map))
        .add_systems(Update, (update_button_state, hud_action))
        .add_systems(
            Update,
            update_removed_walls.run_if(
                rng_seed_changed
                    .or(maze_builder_changed)
                    .or(grid_map_changed),
            ),
        )
        .add_systems(
            Update,
            (update_cell_content, update_wall_visibility)
                .run_if(resource_changed::<RemovedWalls>.and(not(resource_added::<RemovedWalls>))),
        )
        .add_systems(
            Update,
            update_overlay_visibility
                .run_if(resource_changed::<OverlayState>.and(not(resource_added::<OverlayState>))),
        )
        .run();
}
