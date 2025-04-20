use crate::maze_builders::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn update_removed_walls(
    grid_map: Res<GridMap>,
    rng_seed: Res<RngSeed>,
    maze_builder_type: Res<MazeBuilderType>,
    mut removed_walls: ResMut<RemovedWalls>,
) {
    removed_walls.0 = match *maze_builder_type {
        MazeBuilderType::AldousBroder => carve_aldous_broder_into_grid_map(&grid_map, rng_seed.0),
        MazeBuilderType::Wilson => carve_wilson_into_grid_map(&grid_map, rng_seed.0),
        MazeBuilderType::BinaryTree => carve_binary_tree_into_grid_map(&grid_map, rng_seed.0),
        MazeBuilderType::Sidewinder => carve_sidewinder_into_grid_map(&grid_map, rng_seed.0),
    };
}
