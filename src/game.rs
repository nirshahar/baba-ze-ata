use bevy::prelude::{Entity, Plugin};

use crate::cell::CellType;

pub struct CellGroup {
    pub baba_cells: Vec<Entity>,
    pub rock_cells: Vec<Entity>,
    pub tree_cells: Vec<Entity>,
    pub wall_cells: Vec<Entity>,
    pub grass_cells: Vec<Entity>,
    pub water_cells: Vec<Entity>,
}

impl CellGroup {
    fn new() -> Self {
        Self {
            baba_cells: Vec::new(),
            rock_cells: Vec::new(),
            tree_cells: Vec::new(),
            wall_cells: Vec::new(),
            grass_cells: Vec::new(),
            water_cells: Vec::new(),
        }
    }

    pub fn get(&mut self, cell_type: CellType) -> &mut Vec<Entity> {
        match cell_type {
            CellType::Baba => &mut self.baba_cells,
            CellType::Rock => &mut self.rock_cells,
            CellType::Tree => &mut self.tree_cells,
            CellType::Wall => &mut self.wall_cells,
            CellType::Grass => &mut self.grass_cells,
            CellType::Water => &mut self.water_cells,
        }
    }
}

struct Board<const N: usize> {
    pub board: [[Vec<Entity>; N]; N],
}

impl<const N: usize> Board<N> {
    pub fn new() -> Self {
        Self {
            board: array_init::array_init(|_| array_init::array_init(|_| Vec::new())),
        }
    }
}

pub struct GameResourcesPlugin<const N: usize>;

impl<const N: usize> Plugin for GameResourcesPlugin<N> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(CellGroup::new())
            .insert_resource(Board::<N>::new());
    }
}
