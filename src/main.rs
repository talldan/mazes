use bevy::{prelude::*, utils::HashSet};

mod components;
mod maze_builders;
mod resources;
mod systems;
mod utils;

pub use crate::resources::*;
pub use crate::systems::*;

pub fn resource_updated<T>(res: Option<Res<T>>) -> bool
where
    T: Resource,
{
    match res {
        Some(res) => !res.is_added() && res.is_changed(),
        None => false,
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GridMap::new(15, 15))
        .insert_resource(RngSeed(0))
        .insert_resource(MazeBuilderType::AldousBroder)
        .insert_resource(OverlayState(false))
        .insert_resource(RemovedWalls(HashSet::new()))
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_hud,
                update_removed_walls.before(setup_grid_map),
                setup_grid_map,
            ),
        )
        .add_systems(
            Update,
            (
                update_button_state,
                handle_hud_action,
                update_removed_walls.run_if(
                    resource_updated::<RngSeed>
                        .or(resource_updated::<MazeBuilderType>)
                        .or(resource_updated::<GridMap>),
                ),
                (update_cell_content, update_wall_visibility)
                    .run_if(resource_updated::<RemovedWalls>),
                update_overlay_visibility.run_if(resource_updated::<OverlayState>),
            ),
        )
        .run();
}
