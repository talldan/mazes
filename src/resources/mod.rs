use crate::components::Wall;
use bevy::{prelude::*, utils::HashSet};
mod grid_map;

pub use grid_map::*;

#[derive(Resource)]
pub struct RngSeed(pub u64);

#[derive(Resource)]
pub struct OverlayState(pub bool);

#[derive(Resource, Copy, Clone)]
pub enum MazeBuilderType {
    BinaryTree,
    Sidewinder,
    AldousBroder,
    Wilson,
}

#[derive(Resource)]
pub struct RemovedWalls(pub HashSet<Wall>);
