use crate::maze_builders::MazeBuilderType;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct UpdateGridDimensions {
    pub size: Vec2,
}

#[derive(Event, Debug)]
pub struct UpdateMazeBuilderType {
    pub size: MazeBuilderType,
}

#[derive(Event, Debug)]
pub struct ShowDijkstraOverlay {
    pub is_shown: bool,
}

#[derive(Event, Debug)]
pub struct UpdateSeed {
    pub seed: usize,
}
