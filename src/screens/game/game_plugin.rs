use bevy::prelude::*;

use crate::{
    GameState,
    screens::game::{
        game_ui::game_ui_plugin::GameUiPlugin,
        space_ship::{space_ship_plugin::SpaceShipPlugin, spawn_space_ship::spawn_space_spaceship},
    },
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SpaceShipPlugin, GameUiPlugin));
    }
}
