use bevy::prelude::*;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum WallOrientation {
    Horizontal,
    Vertical,
}

#[derive(Hash, Eq, PartialEq)]
pub struct Wall {
    pub from: IVec2,
    pub to: IVec2,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Resource)]
pub struct GridMap {
    columns: i32,
    rows: i32,
}

impl GridMap {
    pub fn new(columns: i32, rows: i32) -> Self {
        GridMap { columns, rows }
    }

    pub fn wall_from_cell_pos(&self, cell_pos: IVec2, direction: Dir2) -> Option<Wall> {
        if !self.is_cell_pos_in_bounds(cell_pos) {
            return None;
        }

        match direction {
            Dir2::NORTH => Some(Wall {
                from: IVec2 {
                    x: cell_pos.x,
                    y: cell_pos.y + 1,
                },
                to: cell_pos + 1,
            }),
            Dir2::SOUTH => Some(Wall {
                from: cell_pos,
                to: IVec2 {
                    x: cell_pos.x + 1,
                    y: cell_pos.y,
                },
            }),
            Dir2::EAST => Some(Wall {
                from: IVec2 {
                    x: cell_pos.x + 1,
                    y: cell_pos.y,
                },
                to: cell_pos + 1,
            }),
            Dir2::WEST => Some(Wall {
                from: cell_pos,
                to: IVec2 {
                    x: cell_pos.x,
                    y: cell_pos.y + 1,
                },
            }),
            _ => None,
        }
    }

    pub fn wall_from_cell_index(&self, index: i32, direction: Dir2) -> Option<Wall> {
        let cell_pos = self.index_to_cell_pos(index);
        match cell_pos {
            Some(cell_pos) => self.wall_from_cell_pos(cell_pos, direction),
            None => None,
        }
    }

    pub fn neighbour_from_cell_index(&self, index: i32, direction: Dir2) -> Option<i32> {
        let cell_pos = self.index_to_cell_pos(index);
        match cell_pos {
            Some(cell_pos) => self.cell_pos_to_index(cell_pos + direction.as_ivec2()),
            None => None,
        }
    }

    pub fn neighbour_from_cell_pos(&self, cell_pos: IVec2, direction: Dir2) -> Option<IVec2> {
        let neighbour_pos = cell_pos + direction.as_ivec2();
        match self.is_cell_pos_in_bounds(neighbour_pos) {
            true => Some(neighbour_pos),
            false => None,
        }
    }

    pub fn is_cell_pos_in_bounds(&self, cell_pos: IVec2) -> bool {
        is_pos_in_bounds(cell_pos, self.columns, self.rows)
    }

    pub fn is_cell_index_in_bounds(&self, index: i32) -> bool {
        is_index_in_bounds(index, self.columns, self.rows)
    }

    pub fn cell_pos_to_index(&self, cell_pos: IVec2) -> Option<i32> {
        pos_to_index(cell_pos, self.columns, self.rows)
    }

    pub fn index_to_cell_pos(&self, index: i32) -> Option<IVec2> {
        index_to_pos(index, self.columns, self.rows)
    }

    pub fn is_point_pos_in_bounds(&self, point_pos: IVec2) -> bool {
        is_pos_in_bounds(point_pos, self.columns + 1, self.rows + 1)
    }

    pub fn is_point_index_in_bounds(&self, index: i32) -> bool {
        is_index_in_bounds(index, self.columns + 1, self.rows + 1)
    }

    pub fn point_pos_to_index(&self, point_pos: IVec2) -> Option<i32> {
        pos_to_index(point_pos, self.columns + 1, self.rows + 1)
    }

    pub fn index_to_point_pos(&self, index: i32) -> Option<IVec2> {
        index_to_pos(index, self.columns + 1, self.rows + 1)
    }

    pub fn is_wall_pos_in_bounds(&self, wall: Wall) -> bool {
        let cols = self.columns + 1;
        let rows = self.rows + 1;
        is_pos_in_bounds(wall.from, cols, rows) && is_pos_in_bounds(wall.to, cols, rows)
    }

    pub fn is_wall_index_in_bounds(&self, index: i32, orientation: WallOrientation) -> bool {
        if orientation == WallOrientation::Horizontal {
            is_index_in_bounds(index, self.columns, self.rows + 1)
        } else {
            is_index_in_bounds(index, self.columns + 1, self.rows)
        }
    }

    pub fn wall_pos_to_index(&self, wall: Wall) -> Option<i32> {
        if wall.from.x == wall.to.x {
            // Horizontal.
            pos_to_index(wall.from, self.columns, self.rows + 1)
        } else {
            // Vertical.
            pos_to_index(wall.from, self.columns + 1, self.rows)
        }
    }

    pub fn index_to_wall_pos(&self, index: i32, orientation: WallOrientation) -> Option<Wall> {
        let from = if orientation == WallOrientation::Horizontal {
            index_to_pos(index, self.columns, self.rows + 1)
        } else {
            index_to_pos(index, self.columns + 1, self.rows)
        };

        // We know that if `from` is valid, then we can also calculate a valid `to` pos
        // due to the way `index_to_pos` works, so defer calculating `to` until we've
        // checked there is `Some` `from` value.
        if let Some(from) = from {
            let to = if orientation == WallOrientation::Horizontal {
                IVec2 {
                    x: from.x + 1,
                    y: from.y,
                }
            } else {
                IVec2 {
                    x: from.x,
                    y: from.y + 1,
                }
            };

            return Some(Wall { from, to });
        } else {
            return None;
        }
    }

    pub fn get_scale_from_available_space(&self, available_space: Vec2) -> f32 {
        let width_scale = available_space.x / (self.columns as f32);
        let height_scale = available_space.y / (self.rows as f32);

        if width_scale > height_scale {
            height_scale
        } else {
            width_scale
        }
    }

    pub fn get_centered_grid_pos(&self, scale: f32) -> Vec2 {
        let base_vec = Vec2 {
            x: self.columns as f32,
            y: self.rows as f32,
        };

        -(base_vec * scale) / 2.0
    }

    pub fn get_cell_count(&self) -> i32 {
        self.columns * self.rows
    }

    pub fn get_point_count(&self) -> i32 {
        (self.columns + 1) * (self.rows + 1)
    }

    pub fn get_wall_count(&self) -> i32 {
        (self.columns * (self.rows + 1)) + (self.rows * (self.columns + 1))
    }

    pub fn iter_cells(&self) -> GridMapCellIterator {
        GridMapCellIterator {
            grid_map: self,
            index: 0,
        }
    }

    pub fn iter_points(&self) -> GridMapPointIterator {
        GridMapPointIterator {
            grid_map: self,
            index: 0,
        }
    }

    pub fn iter_walls(&self, orientation: WallOrientation) -> GridMapWallIterator {
        GridMapWallIterator {
            grid_map: self,
            index: 0,
            orientation,
        }
    }
}

pub struct GridMapCellIterator<'a> {
    grid_map: &'a GridMap,
    index: usize,
}

impl<'a> Iterator for GridMapCellIterator<'a> {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid_map.index_to_cell_pos(self.index as i32);
        self.index += 1;
        result
    }
}

pub struct GridMapPointIterator<'a> {
    grid_map: &'a GridMap,
    index: usize,
}

impl<'a> Iterator for GridMapPointIterator<'a> {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid_map.index_to_point_pos(self.index as i32);
        self.index += 1;
        result
    }
}

pub struct GridMapWallIterator<'a> {
    grid_map: &'a GridMap,
    orientation: WallOrientation,
    index: usize,
}

impl<'a> Iterator for GridMapWallIterator<'a> {
    type Item = Wall;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self
            .grid_map
            .index_to_wall_pos(self.index as i32, self.orientation);
        self.index += 1;
        result
    }
}

fn is_pos_in_bounds(pos: IVec2, columns: i32, rows: i32) -> bool {
    pos.x < columns && pos.y < rows
}

fn pos_to_index(pos: IVec2, columns: i32, rows: i32) -> Option<i32> {
    if !is_pos_in_bounds(pos, columns, rows) {
        return None;
    }

    Some((pos.y * columns) + pos.x)
}

fn is_index_in_bounds(index: i32, columns: i32, rows: i32) -> bool {
    index < (columns * rows)
}

fn index_to_pos(index: i32, columns: i32, rows: i32) -> Option<IVec2> {
    if !is_index_in_bounds(index, columns, rows) {
        return None;
    }

    Some(IVec2 {
        x: index % columns,
        y: index / columns,
    })
}
