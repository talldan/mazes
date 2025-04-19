use super::map_utils::*;
use crate::components::*;
use crate::maze_builders::*;
use crate::resources::*;
use crate::utils::*;
use bevy::prelude::*;

pub fn update_overlay(
    grid_map: Res<GridMap>,
    rng_seed: Res<RngSeed>,
    overlay_state: Res<OverlayState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cells_query: Query<(&Cell, &mut MeshMaterial2d<ColorMaterial>, &Children)>,
    mut cell_text_query: Query<(&mut Text2d, &mut TextColor)>,
) {
    let removed_walls = carve_aldous_broder_into_grid_map(&grid_map, rng_seed.0);
    let start = grid_map.get_north_east_cell_pos();
    let distances = dijkstra(start, &grid_map, &removed_walls);
    let (to, _) = get_most_distant(&distances);
    let distances = dijkstra(to, &grid_map, &removed_walls);
    let (from, farthest_distance) = get_most_distant(&distances);
    let path = get_path(from, to, &grid_map, &removed_walls);
    let has_overlay = overlay_state.0;

    for (cell, mut material, children) in &mut cells_query {
        // Update the cell's background.
        let distance = distances.get(&cell.position);
        let background_color = get_cell_background_color(distance, farthest_distance, has_overlay);
        let background_material = materials.add(background_color);
        let old_material_id = material.id();
        *material = MeshMaterial2d(background_material);
        materials.remove(old_material_id);

        // With the child cell text...
        let (mut text, mut text_color) = cell_text_query.get_mut(children[0]).unwrap();

        // Update the text.
        let is_start = cell.position == from;
        let is_end = cell.position == to;
        let new_text = get_cell_text(is_start, is_end, distance, has_overlay);
        **text = new_text;

        // Update the color.
        let is_on_path = path.contains_key(&cell.position);
        let new_color = get_cell_text_color(is_on_path);
        **text_color = new_color;
    }
}
