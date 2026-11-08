use bevy::prelude::*;

use crate::{
    GameState,
    screens::game::{
        collision::collision_plugin::CollisionPlugin,
        enemies::enemies_plugin::EnemiesPlugin,
        game_ui::game_ui_plugin::GameUiPlugin,
        space_ship::{space_ship_plugin::SpaceShipPlugin, spawn_space_ship::spawn_space_spaceship},
        walls::walls_plugin::WallsPlugin,
    },
};

pub struct GamePlugin;

#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct GameSet;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SpaceShipPlugin,
            GameUiPlugin,
            WallsPlugin,
            EnemiesPlugin,
            CollisionPlugin,
        ));
    }
}
