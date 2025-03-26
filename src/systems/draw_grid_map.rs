use crate::resources::GridMap;
use bevy::prelude::*;

const POINT_SIZE: f32 = 4.0;
const PADDING_PX: f32 = 150.0;

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
        x: window_width - PADDING_PX,
        y: window_height - PADDING_PX,
    };

    let grid_size = grid_map.fit_to_available_space(available_space);

    println!("grid_size: {}, {}", grid_size.x, grid_size.y);

    let start_pos = -grid_size / 2.0;
    println!("start_pos: {}, {}", start_pos.x, start_pos.y);

    for y in 0..(grid_map.size.y as i32) {
        for x in 0..(grid_map.size.x as i32) {
            let mesh = meshes.add(Circle::new(POINT_SIZE));
            let color = Color::srgb(1.0, 1.0, 1.0);
            let material = materials.add(color);

            let grid_index = Vec2 {
                x: x as f32,
                y: y as f32,
            };

            let gap = grid_size / (grid_map.size - 1.0);
            let pos = start_pos + (grid_index * gap);

            commands.spawn((
                Mesh2d(mesh),
                MeshMaterial2d(material),
                Transform::from_xyz(pos.x, pos.y, 0.0),
            ));
        }
    }
}
