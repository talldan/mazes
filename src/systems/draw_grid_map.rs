use crate::maze_builders::*;
use crate::resources::*;
use crate::utils::*;
use bevy::color::palettes::css::*;
use bevy::prelude::*;

const POINT_SIZE: f32 = 4.0;
const WALL_SIZE: f32 = 2.0;
const PADDING_PX: f32 = 75.0;
const COLOR: Color = Color::srgb(0.2, 0.2, 0.2);

pub fn draw_grid_map(
    mut commands: Commands,
    grid_map: Res<GridMap>,
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

    let show_dijkstra_overlay = false;
    let scale = grid_map.get_scale_from_available_space(available_space);
    let start_pos = grid_map.get_centered_grid_pos(scale);
    let point_shape = meshes.add(Circle::new(POINT_SIZE));
    let rectangle_shape = meshes.add(Rectangle::new(1.0, 1.0));
    let material = materials.add(COLOR);
    let removed_walls = carve_wilson_into_grid_map(&grid_map);
    let start = grid_map.get_north_east_cell_pos();
    let distances = dijkstra(start, &grid_map, &removed_walls);
    let (to, _) = get_most_distant(&distances);
    let distances = dijkstra(to, &grid_map, &removed_walls);
    let (from, farthest_distance) = get_most_distant(&distances);
    let path = get_path(from, to, &grid_map, &removed_walls);

    let cell_batch: Vec<(Mesh2d, MeshMaterial2d<ColorMaterial>, Transform)> = grid_map
        .iter_cells()
        .map(|cell| {
            let cell_coords = start_pos + (cell.as_vec2() * scale);
            let distance = distances.get(&cell);
            let color = if !show_dijkstra_overlay {
                Color::srgba(0.8, 0.8, 0.8, 1.0)
            } else if let Some(distance) = distance {
                let max = farthest_distance as f32;
                let dist = *distance as f32;
                let intensity = (max - dist) / max;
                let dark = 1.0 * intensity;
                let bright = 0.5 + (0.5 * intensity);
                Color::srgb(dark, bright, dark)
            } else {
                Color::srgba(1.0, 1.0, 1.0, 0.0)
            };
            let background_material = materials.add(color);
            (
                Mesh2d(rectangle_shape.clone()),
                MeshMaterial2d(background_material),
                Transform::from_scale(Vec3 {
                    x: scale,
                    y: scale,
                    z: 1.0,
                })
                .with_translation(Vec3 {
                    // The rectangle will be centered over the 'from' position,
                    // so we need to move it by another half of its length.
                    x: cell_coords.x + (scale * 0.5),
                    y: cell_coords.y + (scale * 0.5),
                    z: -0.1,
                }),
            )
        })
        .collect();

    let point_batch: Vec<(Mesh2d, MeshMaterial2d<ColorMaterial>, Transform)> = grid_map
        .iter_points()
        .map(|point_pos| {
            let cell_coords = start_pos + (point_pos.as_vec2() * scale);

            (
                Mesh2d(point_shape.clone()),
                MeshMaterial2d(material.clone()),
                Transform::from_xyz(cell_coords.x, cell_coords.y, 0.0),
            )
        })
        .collect();

    let horizontal_wall_batch: Vec<(Mesh2d, MeshMaterial2d<ColorMaterial>, Transform)> = grid_map
        .iter_walls(WallOrientation::Horizontal)
        .filter(|wall| !removed_walls.contains(wall))
        .map(|wall| {
            let from = start_pos + (wall.from.as_vec2() * scale);

            (
                Mesh2d(rectangle_shape.clone()),
                MeshMaterial2d(material.clone()),
                Transform::from_scale(Vec3 {
                    x: scale,
                    y: WALL_SIZE,
                    z: 1.0,
                })
                .with_translation(Vec3 {
                    // The rectangle will be centered over the 'from' position,
                    // so we need to move it by another half of its length.
                    x: from.x + (scale * 0.5),
                    y: from.y,
                    z: 0.0,
                }),
            )
        })
        .collect();

    let vertical_wall_batch: Vec<(Mesh2d, MeshMaterial2d<ColorMaterial>, Transform)> = grid_map
        .iter_walls(WallOrientation::Vertical)
        .filter(|wall| !removed_walls.contains(wall))
        .map(|wall| {
            let from = start_pos + (wall.from.as_vec2() * scale);

            (
                Mesh2d(rectangle_shape.clone()),
                MeshMaterial2d(material.clone()),
                Transform::from_scale(Vec3 {
                    x: WALL_SIZE,
                    y: scale,
                    z: 1.0,
                })
                .with_translation(Vec3 {
                    // The rectangle will be centered over the 'from' position,
                    // so we need to move it by another half of its length.
                    x: from.x,
                    y: from.y + (scale * 0.5),
                    z: 0.0,
                }),
            )
        })
        .collect();

    let cell_content_batch: Vec<(Text2d, TextColor, Transform)> = grid_map
        .iter_cells()
        .map(|cell| {
            let distance = distances.get(&cell);
            let text = if !show_dijkstra_overlay {
                if cell == from {
                    Text2d::new(format!("GO"))
                } else if cell == to {
                    Text2d::new(format!("END"))
                } else {
                    Text2d::new(format!(""))
                }
            } else if let Some(distance) = distance {
                Text2d::new(format!("{distance}"))
            } else {
                Text2d::new("")
            };
            let cell_coords = start_pos + (cell.as_vec2() * scale);
            let is_on_path = path.contains_key(&cell);
            let color = if is_on_path {
                Color::Srgba(RED)
            } else {
                Color::Srgba(BLACK)
            };

            (
                text,
                TextColor(color),
                Transform::from_scale(Vec3 {
                    x: 0.02 * scale,
                    y: 0.02 * scale,
                    z: 1.0,
                })
                .with_translation(Vec3 {
                    x: cell_coords.x + (scale * 0.5),
                    y: cell_coords.y + (scale * 0.5),
                    z: 0.0,
                }),
            )
        })
        .collect();

    commands.spawn_batch(cell_batch);
    commands.spawn_batch(point_batch);
    commands.spawn_batch(horizontal_wall_batch);
    commands.spawn_batch(vertical_wall_batch);
    commands.spawn_batch(cell_content_batch);
}
