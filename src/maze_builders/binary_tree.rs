use super::get_direction_from_coinflip;
use crate::resources::{GridMap, Wall};
use bevy::{prelude::*, utils::HashSet};
use fastrand;

pub fn carve_binary_tree_into_grid_map(grid_map: &GridMap) -> HashSet<Wall> {
    let mut removed_walls = HashSet::new();

    grid_map.iter_cells().for_each(|cell_pos| {
        let coinflip = fastrand::bool();
        let mut direction = get_direction_from_coinflip(coinflip);
        let neighbour = grid_map.neighbour_from_cell_pos(cell_pos, direction);

        // Switch direction if there's no neighbour in that direction.
        // This avoids carving through the outer walls.
        if neighbour.is_none() {
            direction = get_direction_from_coinflip(!coinflip);
        }

        // Check the neighbor again, we might be in the far corner, and so should
        // be careful again not to carve through an outer wall.
        let neighbour = grid_map.neighbour_from_cell_pos(cell_pos, direction);
        if neighbour.is_some() {
            let wall = grid_map.wall_from_cell_pos(cell_pos, direction);
            if let Some(wall) = wall {
                removed_walls.insert(wall);
            }
        }
    });

    removed_walls
}
