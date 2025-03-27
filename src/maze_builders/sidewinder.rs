use super::get_direction_from_coinflip;
use crate::resources::{GridMap, Wall};
use bevy::{prelude::*, utils::HashSet};
use fastrand;

pub fn carve_sidewinder_into_grid_map(grid_map: &GridMap) -> HashSet<Wall> {
    let mut removed_walls = HashSet::new();
    let mut run = Vec::new();

    grid_map.iter_cells().for_each(|cell_pos| {
        run.push(cell_pos);

        let coinflip = fastrand::bool();
        let mut direction = get_direction_from_coinflip(coinflip);

        let mut wall = grid_map.inner_wall_from_cell_pos(cell_pos, direction);

        // If we've selected a wall at the boundary then flip the direction.
        if wall.is_none() {
            direction = get_direction_from_coinflip(!coinflip);
        }

        // If we're traveling east, continue carving in that direction building up a
        // 'run' of cells until we hit the boundary, at which point we switch direction
        // to north.
        if direction == Dir2::EAST {
            wall = grid_map.inner_wall_from_cell_pos(cell_pos, direction);
        }

        // When travelling north, pick a random cell to carve north from one of
        // the stored 'run' cells.
        if direction == Dir2::NORTH {
            let run_index = fastrand::usize(0..run.len());
            let run_cell = run[run_index];
            wall = grid_map.inner_wall_from_cell_pos(run_cell, direction);
            run.clear();
        }

        // Remove the chosen wall.
        if let Some(wall) = wall {
            removed_walls.insert(wall);
        }
    });

    removed_walls
}
