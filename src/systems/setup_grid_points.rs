use crate::resources::MazeMap;
use bevy::prelude::*;

const POINT_SIZE: f32 = 2.0;
const GRID_SPACING: f32 = 100.0;

pub fn setup_grid_points(
    mut commands: Commands,
    maze_map: Res<MazeMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let maze_map = maze_map.into_inner();
    if let MazeMap::Grid(width, height) = maze_map {
        // let num_points = width * height;

        for y in 0..*height {
            for x in 0..*width {
                let mesh = meshes.add(Circle::new(POINT_SIZE));
                let color = Color::rgb(1.0, 1.0, 1.0);
                let material = materials.add(color);

                commands.spawn((
                    Mesh2d(mesh),
                    MeshMaterial2d(material),
                    Transform::from_xyz((x as f32) * GRID_SPACING, (y as f32) * GRID_SPACING, 0.0),
                ));
            }
        }
    }
}
