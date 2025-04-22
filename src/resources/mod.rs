use crate::components::Wall;
use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
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

#[derive(Resource)]
pub struct DijkstraMap(pub HashMap<IVec2, i32>);

#[derive(Resource, Default)]
pub struct Solution {
    pub start: IVec2,
    pub end: IVec2,
    pub path: HashMap<IVec2, i32>,
    pub distances: HashMap<IVec2, i32>,
    pub farthest_distance: i32,
}
