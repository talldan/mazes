use crate::{resources::GridMap, WallOrientation};
use bevy::prelude::*;

const POINT_SIZE: f32 = 4.0;
const WALL_SIZE: f32 = 2.0;
const PADDING_PX: f32 = 75.0;
const COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

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

    let scale = grid_map.get_scale_from_available_space(available_space);
    let start_pos = grid_map.get_centered_grid_pos(scale);
    let point_shape = meshes.add(Circle::new(POINT_SIZE));
    let material = materials.add(COLOR);

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

    let wall_shape = meshes.add(Rectangle::new(1.0, 1.0));

    let horizontal_wall_batch: Vec<(Mesh2d, MeshMaterial2d<ColorMaterial>, Transform)> = grid_map
        .iter_walls(WallOrientation::Horizontal)
        .map(|wall| {
            let from = start_pos + (wall.from.as_vec2() * scale);

            (
                Mesh2d(wall_shape.clone()),
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
        .map(|wall| {
            let from = start_pos + (wall.from.as_vec2() * scale);

            (
                Mesh2d(wall_shape.clone()),
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

    commands.spawn_batch(point_batch);
    commands.spawn_batch(horizontal_wall_batch);
    commands.spawn_batch(vertical_wall_batch);
}
