use super::map_utils::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

const POINT_SIZE: f32 = 0.12;
const WALL_SIZE: f32 = 0.08;
const PADDING_PX: f32 = 75.0;
const COLOR: Color = Color::srgb(0.2, 0.2, 0.2);

pub fn setup_grid_map(
    mut commands: Commands,
    grid_map: Res<GridMap>,
    overlay_state: Res<OverlayState>,
    removed_walls: Res<RemovedWalls>,
    solution: Res<Solution>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&mut Window>,
) {
    let window = windows.single();
    let window_width = window.resolution.width();
    let window_height = window.resolution.height();
    let available_space = Vec2 {
        x: window_width - (PADDING_PX * 2.0),
        y: window_height - (PADDING_PX * 2.0),
    };

    let has_overlay = overlay_state.0;
    let scale = grid_map.get_scale_from_available_space(available_space);
    let start_pos = grid_map.get_centered_grid_pos(scale);
    let point_shape = meshes.add(Circle::new(POINT_SIZE));
    let rectangle_shape = meshes.add(Rectangle::new(1.0, 1.0));
    let material = materials.add(COLOR);

    let grid_entity = commands
        .spawn((
            Transform::from_scale(Vec3 {
                x: scale,
                y: scale,
                z: 1.0,
            })
            .with_translation(Vec3 {
                x: start_pos.x,
                y: start_pos.y,
                z: 0.0,
            }),
            InheritedVisibility::VISIBLE,
        ))
        .id();

    grid_map.iter_points().for_each(|point_pos| {
        let point_entity = commands
            .spawn((
                Mesh2d(point_shape.clone()),
                MeshMaterial2d(material.clone()),
                Transform::from_translation(point_pos.as_vec2().extend(0.0)),
            ))
            .id();

        commands.entity(grid_entity).add_child(point_entity);
    });

    grid_map.iter_cells().for_each(|cell_position| {
        let translation = cell_position.as_vec2();
        let distance = solution.distances.get(&cell_position);

        let cell_entity = commands
            .spawn((
                Cell {
                    position: cell_position,
                },
                Transform::from_scale(Vec3::ONE).with_translation(Vec3 {
                    // The rectangle will be centered over the 'from' position,
                    // so we need to move it by another half of its length.
                    x: translation.x + 0.5,
                    y: translation.y + 0.5,
                    z: -0.1,
                }),
                Visibility::Visible,
            ))
            .id();

        let cell_contents_visibility = if has_overlay {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
        let cell_background_color =
            get_cell_background_color(distance, solution.farthest_distance, false);
        let cell_background_material = materials.add(cell_background_color);
        let cell_background_entity = commands
            .spawn((
                CellContentBackground,
                OverlayVisibility(false),
                Mesh2d(rectangle_shape.clone()),
                MeshMaterial2d(cell_background_material),
                Transform::from_scale(Vec3::ONE),
                cell_contents_visibility,
            ))
            .id();
        let is_start = cell_position == solution.start;
        let is_end = cell_position == solution.end;
        let is_on_path = solution.path.contains_key(&cell_position);
        let cell_text_color = get_cell_text_color(is_on_path);
        let cell_text = get_cell_text(is_start, is_end, distance, false);
        let cell_text_entity = commands
            .spawn((
                CellContentText,
                OverlayVisibility(false),
                Text2d(cell_text),
                TextColor(cell_text_color.clone()),
                Transform::from_scale(Vec3 {
                    x: 0.02,
                    y: 0.02,
                    z: 1.0,
                }),
                cell_contents_visibility,
            ))
            .id();

        let overlay_visibility = if has_overlay {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        let overlay_background_color =
            get_cell_background_color(distance, solution.farthest_distance, true);
        let overlay_background_material = materials.add(overlay_background_color);
        let overlay_background_entity = commands
            .spawn((
                CellOverlayBackground,
                OverlayVisibility(true),
                Mesh2d(rectangle_shape.clone()),
                MeshMaterial2d(overlay_background_material),
                Transform::from_scale(Vec3::ONE),
                overlay_visibility,
            ))
            .id();
        let text_color = get_cell_text_color(is_on_path);
        let overlay_text = get_cell_text(is_start, is_end, distance, true);
        let overlay_text_entity = commands
            .spawn((
                CellOverlayText,
                OverlayVisibility(true),
                Text2d(overlay_text),
                TextColor(text_color),
                Transform::from_scale(Vec3 {
                    x: 0.02,
                    y: 0.02,
                    z: 1.0,
                }),
                overlay_visibility,
            ))
            .id();

        commands.entity(cell_entity).add_children(&[
            cell_background_entity,
            cell_text_entity,
            overlay_background_entity,
            overlay_text_entity,
        ]);
        commands.entity(grid_entity).add_child(cell_entity);
    });

    grid_map
        .iter_walls(WallOrientation::Horizontal)
        .for_each(|wall| {
            let from = wall.from.as_vec2();
            let visibility = if removed_walls.0.contains(&wall) {
                Visibility::Hidden
            } else {
                Visibility::Visible
            };

            let wall_entity = commands
                .spawn((
                    wall,
                    WallOrientation::Horizontal,
                    Mesh2d(rectangle_shape.clone()),
                    MeshMaterial2d(material.clone()),
                    visibility,
                    Transform::from_scale(Vec3 {
                        x: 1.0,
                        y: WALL_SIZE,
                        z: 1.0,
                    })
                    .with_translation(Vec3 {
                        // The rectangle will be centered over the 'from' position,
                        // so we need to move it by another half of its length.
                        x: from.x + 0.5,
                        y: from.y,
                        z: 0.0,
                    }),
                ))
                .id();

            commands.entity(grid_entity).add_child(wall_entity);
        });

    grid_map
        .iter_walls(WallOrientation::Vertical)
        .for_each(|wall| {
            let from = wall.from.as_vec2();
            let visibility = if removed_walls.0.contains(&wall) {
                Visibility::Hidden
            } else {
                Visibility::Visible
            };

            let wall_entity = commands
                .spawn((
                    WallOrientation::Vertical,
                    wall,
                    Mesh2d(rectangle_shape.clone()),
                    MeshMaterial2d(material.clone()),
                    visibility,
                    Transform::from_scale(Vec3 {
                        x: WALL_SIZE,
                        y: 1.0,
                        z: 1.0,
                    })
                    .with_translation(Vec3 {
                        // The rectangle will be centered over the 'from' position,
                        // so we need to move it by another half of its length.
                        x: from.x,
                        y: from.y + 0.5,
                        z: 0.0,
                    }),
                ))
                .id();

            commands.entity(grid_entity).add_child(wall_entity);
        });
}
