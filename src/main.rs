use bevy::{
    prelude::{App, Commands, Query},
    DefaultPlugins,
};
use cell::{Cell, CellType};
use game::GameResourcesPlugin;
use rules::{RuleIsYou, RulesPlugin};

pub mod cell;
pub mod game;
pub mod rules;

const BOARD_SIZE: usize = 8;

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Cell::new(CellType::Baba, 0, 0))
        .insert(RuleIsYou);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GameResourcesPlugin::<BOARD_SIZE>)
        .add_plugin(RulesPlugin)
        .add_startup_system(setup)
        .run();
}
