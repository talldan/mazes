use crate::maze_builders::MazeBuilderType;
use bevy::prelude::*;

#[derive(Component)]
pub enum HudAction {
    ToggleOverlay,
    RandomiseSeed,
    ChangeMazeType(MazeBuilderType),
}

#[derive(Component, Clone, PartialEq)]
pub enum ButtonVariant {
    Normal(String),
    Toggle(
        // initial state,
        bool,
        // inactive text
        String,
        // active text
        String,
    ),
}

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

#[derive(Component)]
pub struct OverlayVisibility(pub bool);
