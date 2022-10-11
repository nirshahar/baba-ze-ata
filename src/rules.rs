use bevy::prelude::{Component, Input, KeyCode, Plugin, Query, Res, With};

use crate::cell::{Cell, CellPos};

pub struct RulesPlugin;

impl Plugin for RulesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(RuleIsYou::apply_rule);
    }
}

#[derive(Component)]
pub struct RuleIsYou;
impl RuleIsYou {
    fn apply_rule(keyboard: Res<Input<KeyCode>>, mut targets: Query<&mut Cell, With<RuleIsYou>>) {
        let direction_pressed = if keyboard.just_pressed(KeyCode::A) {
            Some((-1, 0))
        } else if keyboard.just_pressed(KeyCode::W) {
            Some((0, 1))
        } else if keyboard.just_pressed(KeyCode::D) {
            Some((1, 0))
        } else if keyboard.just_pressed(KeyCode::S) {
            Some((0, -1))
        } else {
            None
        };

        if let Some((x_off, y_off)) = direction_pressed {
            for mut entity in targets.iter_mut() {
                let entity = entity.as_mut();
                entity.pos.x += x_off;
                entity.pos.y += y_off;

                println!("entity: {:?}", entity);
            }
        }
    }
}
