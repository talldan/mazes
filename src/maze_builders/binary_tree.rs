use super::get_direction_from_coinflip;
use crate::resources::{GridMap, Wall};
use bevy::utils::HashSet;
use fastrand::Rng;

pub fn carve_binary_tree_into_grid_map(grid_map: &GridMap, rng_seed: u64) -> HashSet<Wall> {
    let mut rng = Rng::with_seed(rng_seed);
    let mut removed_walls = HashSet::new();

    grid_map.iter_cells().for_each(|cell_pos| {
        let coinflip = rng.bool();
        let mut direction = get_direction_from_coinflip(coinflip);
        let mut wall = grid_map.inner_wall_from_cell_pos(cell_pos, direction);

        // Switch direction if there's no neighbour in that direction.
        // This avoids carving through the outer walls.
        if wall.is_none() {
            direction = get_direction_from_coinflip(!coinflip);
            wall = grid_map.inner_wall_from_cell_pos(cell_pos, direction);
        }

        // Check the neighbor again, we might be in the far corner, and so should
        // be careful again not to carve through an outer wall.
        if let Some(wall) = wall {
            removed_walls.insert(wall);
        }
    });

    removed_walls
}
