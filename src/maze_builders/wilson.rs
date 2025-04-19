use crate::components::Wall;
use crate::resources::GridMap;
use bevy::{prelude::*, utils::hashbrown::HashSet};
use fastrand::Rng;

fn get_random_cell(grid_map: &GridMap, rng: &mut Rng) -> IVec2 {
    let cell_count = grid_map.get_cell_count() as usize;
    let random_start = rng.usize(0..cell_count);
    return grid_map.index_to_cell_pos(random_start as i32).unwrap();
}

fn get_random_unvisited_cell(grid_map: &GridMap, visited: &HashSet<IVec2>, rng: &mut Rng) -> IVec2 {
    let mut cell = get_random_cell(grid_map, rng);

    while !visited.contains(&cell) {
        cell = get_random_cell(grid_map, rng);
    }

    return cell;
}

pub fn carve_wilson_into_grid_map(grid_map: &GridMap, rng_seed: u64) -> HashSet<Wall> {
    let mut rng = Rng::with_seed(rng_seed);
    let mut removed_walls = HashSet::new();
    let mut visited = HashSet::new();
    // The first cell is automatically marked as visited so that other 'paths'
    // the algorithm makes have somewhere to terminate.
    visited.insert(IVec2 { x: 0, y: 0 });

    let mut run: Vec<(IVec2, Wall)> = vec![];
    let cell_count = grid_map.get_cell_count() as usize;
    let mut current_pos = get_random_unvisited_cell(grid_map, &visited, &mut rng);

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
        let random_neighbour_index = rng.usize(0..(num_possible_neighbours));
        let neighbour = possible_neighbours.into_iter().nth(random_neighbour_index);

        if let Some((Some(neighbour_cell), Some(neighbour_wall))) = neighbour {
            // Check for possible loops in the current run, where we've returned to a
            // cell that was already part of the run.
            let loop_result = run
                .iter()
                .enumerate()
                .find(|(_, (run_cell, _))| *run_cell == neighbour_cell);

            // If a loop is found undo it, return to where the loop started, resetting the run
            // to that point.
            if let Some((loop_start_index, (loop_start_cell, _))) = loop_result {
                current_pos = *loop_start_cell;
                run.truncate(loop_start_index + 1);
                continue;
            }

            // Complete this run and carve walls.
            if visited.contains(&neighbour_cell) {
                run.iter().for_each(|(cell, wall)| {
                    visited.insert(*cell);
                    removed_walls.insert(*wall);
                });
                run.clear();
                current_pos = get_random_unvisited_cell(grid_map, &visited, &mut rng);
                continue;
            }

            // If those other things didn't happen, continue the run.
            run.push((neighbour_cell, neighbour_wall));
        } else {
            // Something went wrong, a neighbour couldn't be found.
            break;
        }
    }

    removed_walls
}
