use crate::resources::MazeMap;
use bevy::prelude::*;

const POINT_SIZE: f32 = 5.0;
const PADDING_PCT: f32 = 0.15;

pub fn setup_grid_points(
    mut commands: Commands,
    maze_map: Res<MazeMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&mut Window>,
) {
    let maze_map = maze_map.into_inner();
    let window = windows.single();

    if let MazeMap::Grid(grid_width, grid_height) = maze_map {
        let window_width = window.resolution.width();
        let window_height = window.resolution.height();
        let (window_shortest, points_on_shortest) = if window_width > window_height {
            (window_height, grid_height)
        } else {
            (window_width, grid_width)
        };

        let padding = PADDING_PCT * window_shortest;
        let padding_total = padding * 2.0;
        let grid_size = window_shortest - padding_total;
        let spacing = grid_size / (*points_on_shortest - 1) as f32;
        let start_pos = -grid_size / 2.0;

        for y in 0..*grid_height {
            for x in 0..*grid_width {
                let mesh = meshes.add(Circle::new(POINT_SIZE));
                let color = Color::srgb(1.0, 1.0, 1.0);
                let material = materials.add(color);

                let x_pos = start_pos + ((x as f32) * spacing);
                let y_pos = start_pos + ((y as f32) * spacing);

                commands.spawn((
                    Mesh2d(mesh),
                    MeshMaterial2d(material),
                    Transform::from_xyz(x_pos, y_pos, 0.0),
                ));
            }
        }
    }
}
