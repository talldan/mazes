use super::get_direction_from_coinflip;
use crate::resources::{GridMap, Wall};
use bevy::{prelude::*, utils::HashSet};
use fastrand;

pub fn carve_aldous_broder_into_grid_map(grid_map: &GridMap) -> HashSet<Wall> {
    let mut removed_walls = HashSet::new();
    let mut visited = HashSet::new();

    let cell_count = grid_map.get_cell_count() as usize;
    let random_start = fastrand::usize(0..cell_count);
    let mut current_pos = grid_map.index_to_cell_pos((random_start as i32)).unwrap();
    visited.insert(current_pos);

    while visited.len() < cell_count {
        let neighbours = vec![
            (
                grid_map.neighbour_from_cell_pos(current_pos, Dir2::NORTH),
                grid_map.inner_wall_from_cell_pos(current_pos, Dir2::NORTH),
            ),
            (
                grid_map.neighbour_from_cell_pos(current_pos, Dir2::EAST),
                grid_map.inner_wall_from_cell_pos(current_pos, Dir2::EAST),
            ),
            (
                grid_map.neighbour_from_cell_pos(current_pos, Dir2::SOUTH),
                grid_map.inner_wall_from_cell_pos(current_pos, Dir2::SOUTH),
            ),
            (
                grid_map.neighbour_from_cell_pos(current_pos, Dir2::WEST),
                grid_map.inner_wall_from_cell_pos(current_pos, Dir2::WEST),
            ),
        ];

        let possible_neighbours: Vec<(Option<IVec2>, Option<Wall>)> = neighbours
            .into_iter()
            .filter(|(neighbour, wall)| neighbour.is_some() && wall.is_some())
            .collect();

        let num_possible_neighbours = possible_neighbours.len();
        let random_neighbour_index = fastrand::usize(0..(num_possible_neighbours));
        let neighbour = possible_neighbours.iter().nth(random_neighbour_index);

        if let Some((Some(neighbour_cell), Some(neighbour_wall))) = neighbour {
            if !visited.contains(neighbour_cell) {
                removed_walls.insert(*neighbour_wall);
            }
            current_pos = *neighbour_cell;
            visited.insert(current_pos);
        } else {
            // Something went wrong, a neighbour couldn't be found.
            break;
        }
    }

    removed_walls
}
