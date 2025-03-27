use crate::resources::GridMap;
use bevy::prelude::*;

const POINT_SIZE: f32 = 4.0;
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

    grid_map.iter_points().for_each(|cell_pos| {
        let cell_coords = start_pos + (cell_pos.as_vec2() * scale);

        commands.spawn((
            Mesh2d(point_shape.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_xyz(cell_coords.x, cell_coords.y, 0.0),
        ));
    });
}
