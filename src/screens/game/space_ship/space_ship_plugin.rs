use bevy::prelude::*;

use crate::{
    GameState,
    screens::game::space_ship::{
        space_ship_controls::move_spaceship, spawn_space_ship::spawn_space_spaceship,
    },
};

pub struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_space_spaceship)
            .add_systems(Update, move_spaceship);
    }
}
