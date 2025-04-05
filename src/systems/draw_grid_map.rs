use crate::maze_builders::*;
use crate::resources::*;
use crate::utils::*;
use bevy::color::palettes::css::*;
use bevy::prelude::*;

const POINT_SIZE: f32 = 0.12;
const WALL_SIZE: f32 = 0.08;
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

    println!("Scale: {scale}, {available_space}");

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

    grid_map.iter_cells().for_each(|cell| {
        let translation = cell.as_vec2();
        let distance = distances.get(&cell);
        let background_color = if !show_dijkstra_overlay {
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
        let background_material = materials.add(background_color);

        let cell_entity = commands
            .spawn((
                Mesh2d(rectangle_shape.clone()),
                MeshMaterial2d(background_material),
                Transform::from_scale(Vec3::ONE).with_translation(Vec3 {
                    // The rectangle will be centered over the 'from' position,
                    // so we need to move it by another half of its length.
                    x: translation.x + 0.5,
                    y: translation.y + 0.5,
                    z: -0.1,
                }),
            ))
            .id();

        let text = if !show_dijkstra_overlay {
            if cell == from {
                Text2d::new(format!("GO"))
            } else if cell == to {
                Text2d::new(format!("END"))
            } else {
                Text2d::new(format!(""))
            }
        } else {
            if let Some(distance) = distance {
                Text2d::new(format!("{distance}"))
            } else {
                Text2d::new("")
            }
        };
        let is_on_path = path.contains_key(&cell);
        let text_color = if is_on_path {
            Color::Srgba(RED)
        } else {
            Color::Srgba(BLACK)
        };

        let text_entity = commands
            .spawn((
                text,
                TextColor(text_color),
                Transform::from_scale(Vec3 {
                    x: 0.02,
                    y: 0.02,
                    z: 1.0,
                })
                .with_translation(Vec3 {
                    // The rectangle will be centered over the 'from' position,
                    // so we need to move it by another half of its length.
                    x: translation.x + 0.5,
                    y: translation.y + 0.5,
                    z: 0.0,
                }),
            ))
            .id();

        commands.entity(grid_entity).add_child(cell_entity);
        commands.entity(grid_entity).add_child(text_entity);
    });

    grid_map
        .iter_walls(WallOrientation::Horizontal)
        .filter(|wall| !removed_walls.contains(wall))
        .for_each(|wall| {
            let from = wall.from.as_vec2();
            let wall_entity = commands
                .spawn((
                    Mesh2d(rectangle_shape.clone()),
                    MeshMaterial2d(material.clone()),
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
        .filter(|wall| !removed_walls.contains(wall))
        .for_each(|wall| {
            let from = wall.from.as_vec2();
            let wall_entity = commands
                .spawn((
                    Mesh2d(rectangle_shape.clone()),
                    MeshMaterial2d(material.clone()),
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
