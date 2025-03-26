use bevy::prelude::*;

#[derive(Resource)]
pub enum MazeMap {
    Grid(i32, i32),
    Sphere(i32),
}
