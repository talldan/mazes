use crate::components::*;
use crate::maze_builders::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn update_walls(
    grid_map: Res<GridMap>,
    rng_seed: Res<RngSeed>,
    mut walls_query: Query<(&Wall, &mut Visibility)>,
) {
    let removed_walls = carve_aldous_broder_into_grid_map(&grid_map, rng_seed.0);
    for (wall, mut visibility) in &mut walls_query {
        *visibility = if removed_walls.contains(wall) {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
}
