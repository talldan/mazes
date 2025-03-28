use crate::resources::{GridMap, Wall};
use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

fn get_traversable_neighbours(
    from: IVec2,
    grid_map: &GridMap,
    removed_walls: &HashSet<Wall>,
) -> Vec<IVec2> {
    let neighbours = vec![
        (
            Dir2::NORTH,
            grid_map.neighbour_from_cell_pos(from, Dir2::NORTH),
        ),
        (
            Dir2::EAST,
            grid_map.neighbour_from_cell_pos(from, Dir2::EAST),
        ),
        (
            Dir2::SOUTH,
            grid_map.neighbour_from_cell_pos(from, Dir2::SOUTH),
        ),
        (
            Dir2::WEST,
            grid_map.neighbour_from_cell_pos(from, Dir2::WEST),
        ),
    ];

    return neighbours
        .into_iter()
        .filter(|(dir, _)| {
            let wall = grid_map.inner_wall_from_cell_pos(from, *dir);
            if let Some(wall) = wall {
                return removed_walls.contains(&wall);
            }
            return false;
        })
        .filter_map(|(_, neighbour)| neighbour)
        .collect();
}

pub fn dijkstra(
    from: IVec2,
    grid_map: &GridMap,
    removed_walls: &HashSet<Wall>,
) -> HashMap<IVec2, i32> {
    let mut distances: HashMap<IVec2, i32> = HashMap::new();
    let mut distance = 0;
    distances.insert(from, distance);
    let mut frontiers = get_traversable_neighbours(from, grid_map, removed_walls);

    while frontiers.len() > 0 {
        let mut next_frontiers: Vec<IVec2> = vec![];
        distance = distance + 1;
        frontiers.iter().for_each(|frontier| {
            if !distances.contains_key(frontier) {
                distances.insert(*frontier, distance);
                let mut neighbours = get_traversable_neighbours(*frontier, grid_map, removed_walls);
                next_frontiers.append(&mut neighbours);
            }
        });
        frontiers = next_frontiers;
    }

    return distances;
}

pub fn get_path(
    from: IVec2,
    to: IVec2,
    grid_map: &GridMap,
    removed_walls: &HashSet<Wall>,
) -> HashMap<IVec2, i32> {
    let distances = dijkstra(from, grid_map, removed_walls);
    let mut current = to;
    let mut breadcrumbs: HashMap<IVec2, i32> = HashMap::new();
    breadcrumbs.insert(current, 0);

    while current != from {
        let current_distance = distances.get(&current);
        if current_distance.is_none() {
            // Couldn't find a path.
            break;
        }

        let neighbours = get_traversable_neighbours(current, grid_map, removed_walls);
        let next = neighbours.iter().find(|neighbour| {
            let neighbour_distance = distances.get(*neighbour);

            if let (Some(neighbour_distance), Some(current_distance)) =
                (neighbour_distance, current_distance)
            {
                return neighbour_distance < current_distance;
            } else {
                return false;
            }
        });

        if let Some(next) = next {
            current = *next;
            breadcrumbs.insert(current, *current_distance.unwrap());
        } else {
            // Couldn't find a path.
            // This shouldn't happen though, should've hit the earlier break.
            break;
        }
    }

    return breadcrumbs;
}
