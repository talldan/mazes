use bevy::prelude::*;

mod components;
mod events;
mod maze_builders;
mod resources;
mod systems;
mod utils;

pub use crate::events::*;
pub use crate::resources::*;
pub use crate::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GridMap::new(15, 15))
        .insert_resource(RngSeed(0))
        .insert_resource(OverlayState(false))
        .add_systems(Startup, (setup_camera, setup_hud, setup_grid_map))
        .add_systems(Update, (button_state_system, hud_action))
        .add_systems(
            Update,
            (update_walls, update_overlay)
                .run_if(resource_changed::<RngSeed>.and(not(resource_added::<RngSeed>))),
        )
        .add_systems(
            Update,
            update_overlay
                .run_if(resource_changed::<OverlayState>.and(not(resource_added::<OverlayState>))),
        )
        .run();
}
