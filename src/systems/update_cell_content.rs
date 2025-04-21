use super::map_utils::*;
use crate::components::*;
use crate::resources::*;
use crate::utils::*;
use bevy::prelude::*;

pub fn update_cell_content(
    removed_walls: Res<RemovedWalls>,
    grid_map: Res<GridMap>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    cell_query: Query<(&Cell, &Children)>,
    mut cell_content_text_query: Query<
        (&mut Text2d, &mut TextColor),
        (With<CellContentText>, Without<CellOverlayText>),
    >,
    mut cell_overlay_background_query: Query<
        &mut MeshMaterial2d<ColorMaterial>,
        With<CellOverlayBackground>,
    >,
    mut cell_overlay_text_query: Query<
        (&mut Text2d, &mut TextColor),
        (With<CellOverlayText>, Without<CellContentText>),
    >,
) {
    let start = grid_map.get_north_east_cell_pos();
    let distances = dijkstra(start, &grid_map, &removed_walls.0);
    let (to, _) = get_most_distant(&distances);
    let distances = dijkstra(to, &grid_map, &removed_walls.0);
    let (from, farthest_distance) = get_most_distant(&distances);
    let distances = dijkstra(from, &grid_map, &removed_walls.0);
    let path = get_path(from, to, &grid_map, &removed_walls.0);

    for (cell, children) in &cell_query {
        let distance = distances.get(&cell.position);
        let is_start = cell.position == from;
        let is_end = cell.position == to;
        let is_on_path = path.contains_key(&cell.position);

        let (mut content_text, mut content_text_color) =
            cell_content_text_query.get_mut(children[1]).unwrap();
        content_text.0 = get_cell_text(is_start, is_end, distance, false);
        content_text_color.0 = get_cell_text_color(is_on_path);

        let mut overlay_background_material =
            cell_overlay_background_query.get_mut(children[2]).unwrap();
        let old_material_id = overlay_background_material.id();
        let overlay_background_color = get_cell_background_color(distance, farthest_distance, true);
        overlay_background_material.0 = materials.add(overlay_background_color);
        materials.remove(old_material_id);

        let (mut overlay_text, mut overlay_text_color) =
            cell_overlay_text_query.get_mut(children[3]).unwrap();
        overlay_text.0 = get_cell_text(is_start, is_end, distance, true);
        overlay_text_color.0 = get_cell_text_color(is_on_path);
    }
}
