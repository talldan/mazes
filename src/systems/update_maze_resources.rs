use crate::maze_builders::*;
use crate::resources::*;
use crate::utils::*;
use bevy::prelude::*;

pub fn update_maze_resources(
    grid_map: Res<GridMap>,
    rng_seed: Res<RngSeed>,
    maze_builder_type: Res<MazeBuilderType>,
    mut removed_walls: ResMut<RemovedWalls>,
    mut solution: ResMut<Solution>,
) {
    removed_walls.0 = match *maze_builder_type {
        MazeBuilderType::AldousBroder => carve_aldous_broder_into_grid_map(&grid_map, rng_seed.0),
        MazeBuilderType::Wilson => carve_wilson_into_grid_map(&grid_map, rng_seed.0),
        MazeBuilderType::BinaryTree => carve_binary_tree_into_grid_map(&grid_map, rng_seed.0),
        MazeBuilderType::Sidewinder => carve_sidewinder_into_grid_map(&grid_map, rng_seed.0),
    };

    // The following process determines a start cell and end cell that are
    // the furthest apart on the map and finds the quickest path between those points.

    // First pick a temporary start position in the corner of the map.
    let start = grid_map.get_north_east_cell_pos();

    // Get the distances from that start and find furthest cell, which will be the actual end.
    let distances = dijkstra(start, &grid_map, &removed_walls.0);
    let (end, _) = get_most_distant(&distances);

    // Repeat this process from the end to get the actual start.
    let distances = dijkstra(end, &grid_map, &removed_walls.0);
    let (start, farthest_distance) = get_most_distant(&distances);

    // Finally get the distances from the new start.
    let distances = dijkstra(start, &grid_map, &removed_walls.0);

    // Store the path from the start to the end.
    let path = get_path(start, end, &distances, &grid_map, &removed_walls.0);

    *solution = Solution {
        start,
        end,
        path,
        distances,
        farthest_distance,
    };
}
