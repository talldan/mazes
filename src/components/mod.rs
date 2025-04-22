use crate::resources::MazeBuilderType;
use bevy::prelude::*;

#[derive(Component)]
pub enum HudAction {
    ToggleOverlay,
    RandomiseSeed,
    ChangeMazeType(MazeBuilderType),
    None,
}

#[derive(Component, Eq, PartialEq, Copy, Clone)]
pub enum ButtonVariant {
    Normal,
    // bool = is_toggle_active
    Toggle(bool),
    // bool = is_dropdown_open
    DropdownOpener(bool),
    // bool = is_option_active
    Radio(bool),
}

#[derive(Component, Eq, PartialEq, Copy, Clone)]
pub struct Dropdown;

#[derive(Component, Eq, PartialEq, Copy, Clone)]
pub struct DropdownMenu;

#[derive(Component, Eq, PartialEq, Copy, Clone)]
pub struct RadioGroup;

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
pub struct CellContentText;
#[derive(Component)]
pub struct CellContentBackground;

#[derive(Component)]
pub struct CellOverlayText;
#[derive(Component)]
pub struct CellOverlayBackground;

#[derive(Component)]
pub struct OverlayVisibility(pub bool);
