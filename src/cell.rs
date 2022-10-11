use bevy::prelude::Component;

#[derive(Debug)]
pub enum CellType {
    Baba,
    Rock,
    Tree,
    Wall,
    Grass,
    Water,
}

#[derive(Debug)]
pub struct CellPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
pub struct Cell {
    pub cell_type: CellType,
    pub pos: CellPos,
}

impl Cell {
    pub fn new(cell_type: CellType, x: i32, y: i32) -> Self {
        Self {
            cell_type,
            pos: CellPos { x, y },
        }
    }
}
