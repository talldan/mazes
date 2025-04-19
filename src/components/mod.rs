use crate::maze_builders::MazeBuilderType;
use bevy::prelude::*;

#[derive(Component)]
pub enum HudAction {
    ToggleOverlay,
    RandomiseSeed,
    ChangeMazeType(MazeBuilderType),
}

#[derive(Component)]
pub struct ToggleOverlayText;

#[derive(Component, Eq, PartialEq, Copy, Clone)]
pub enum WallOrientation {
    Horizontal,
    Vertical,
}

#[derive(Component, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Wall {
    pub from: IVec2,
    pub to: IVec2,
}

#[derive(Component, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Cell {
    pub position: IVec2,
}
