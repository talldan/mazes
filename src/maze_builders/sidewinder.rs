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

        // Check whether we're on the northern most row, and if so, flip direction.
        let neighbour = grid_map.neighbour_from_cell_pos(cell_pos, direction);
        if neighbour.is_none() {
            direction = get_direction_from_coinflip(!coinflip);
        }

        // If we're traveling east, continue carving in that direction building up a
        // 'run' of cells until we hit the boundary, at which point we switch direction
        // to north.
        if direction == Dir2::EAST {
            let neighbour = grid_map.neighbour_from_cell_pos(cell_pos, direction);

            // Switch direction if there's no neighbour in that direction.
            // This avoids carving through the outer walls.
            if neighbour.is_some() {
                let wall = grid_map.wall_from_cell_pos(cell_pos, direction);
                if let Some(wall) = wall {
                    removed_walls.insert(wall);
                }
            }
        }

        // When travelling north, pick a random cell to carve north from out of
        // the stored 'run' cells.
        if direction == Dir2::NORTH {
            let run_index = fastrand::usize(0..run.len());
            let run_cell = run[run_index];

            let neighbour = grid_map.neighbour_from_cell_pos(run_cell, direction);
            if neighbour.is_some() {
                let wall = grid_map.wall_from_cell_pos(run_cell, direction);
                if let Some(wall) = wall {
                    removed_walls.insert(wall);
                }
            }
            run.clear();
        }
    });

    removed_walls
}
