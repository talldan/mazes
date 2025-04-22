use super::map_utils::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn update_cell_content(
    solution: Res<Solution>,
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
    for (cell, children) in &cell_query {
        let distance = solution.distances.get(&cell.position);
        let is_start = cell.position == solution.start;
        let is_end = cell.position == solution.end;
        let is_on_path = solution.path.contains_key(&cell.position);

        let (mut content_text, mut content_text_color) =
            cell_content_text_query.get_mut(children[1]).unwrap();
        content_text.0 = get_cell_text(is_start, is_end, distance, false);
        content_text_color.0 = get_cell_text_color(is_on_path);

        let mut overlay_background_material =
            cell_overlay_background_query.get_mut(children[2]).unwrap();
        let old_material_id = overlay_background_material.id();
        let overlay_background_color =
            get_cell_background_color(distance, solution.farthest_distance, true);
        overlay_background_material.0 = materials.add(overlay_background_color);
        materials.remove(old_material_id);

        let (mut overlay_text, mut overlay_text_color) =
            cell_overlay_text_query.get_mut(children[3]).unwrap();
        overlay_text.0 = get_cell_text(is_start, is_end, distance, true);
        overlay_text_color.0 = get_cell_text_color(is_on_path);
    }
}
