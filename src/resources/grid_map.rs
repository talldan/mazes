use bevy::prelude::*;

#[derive(Resource)]
pub struct GridMap {
    pub size: Vec2,
}

impl GridMap {
    pub fn new(size: Vec2) -> Self {
        return GridMap { size };
    }

    pub fn fit_to_available_space(&self, available_space: Vec2) -> Vec2 {
        if available_space.x > available_space.y {
            let grid_spacing: f32 = available_space.y / (self.size.y - 1.0);
            return Vec2 {
                x: (self.size.x - 1.0) * grid_spacing,
                y: available_space.y,
            };
        } else {
            let grid_spacing: f32 = available_space.x / (self.size.x - 1.0);
            return Vec2 {
                x: available_space.x,
                y: (self.size.y - 1.0) * grid_spacing,
            };
        };
    }
}
