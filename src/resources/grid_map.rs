use bevy::{prelude::*, utils::HashSet};

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Wall {
    from: IVec2,
    to: IVec2,
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
    removed_walls: HashSet<Wall>,
}

impl GridMap {
    pub fn new(columns: i32, rows: i32) -> Self {
        GridMap {
            columns,
            rows,
            removed_walls: HashSet::new(),
        }
    }

    pub fn remove_wall(&mut self, from: IVec2, to: IVec2) {
        self.removed_walls.insert(Wall { from, to });
    }

    pub fn is_wall_removed(&self, wall: &Wall) {
        self.removed_walls.contains(wall);
    }

    pub fn wall_from_cell_pos(&self, cell_pos: IVec2, direction: Direction) -> Option<Wall> {
        if !self.is_cell_pos_in_bounds(cell_pos) {
            return None;
        }

        match direction {
            Direction::Up => Some(Wall {
                from: cell_pos,
                to: IVec2 {
                    x: cell_pos.x + 1,
                    y: cell_pos.y,
                },
            }),
            Direction::Right => Some(Wall {
                from: IVec2 {
                    x: cell_pos.x + 1,
                    y: cell_pos.y,
                },
                to: cell_pos + 1,
            }),
            Direction::Down => Some(Wall {
                from: IVec2 {
                    x: cell_pos.x,
                    y: cell_pos.y + 1,
                },
                to: cell_pos + 1,
            }),
            Direction::Left => Some(Wall {
                from: cell_pos,
                to: IVec2 {
                    x: cell_pos.x,
                    y: cell_pos.y + 1,
                },
            }),
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
