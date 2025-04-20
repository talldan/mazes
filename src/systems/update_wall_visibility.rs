use crate::components::*;
use crate::maze_builders::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn update_wall_visibility(
    removed_walls: Res<RemovedWalls>,
    mut walls_query: Query<(&Wall, &mut Visibility)>,
) {
    for (wall, mut visibility) in &mut walls_query {
        *visibility = if removed_walls.0.contains(wall) {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
}
