use bevy::prelude::*;

use crate::{GameState, screens::game::space_ship::spawn_space_ship::spawn_space_spaceship};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_space_spaceship);
    }
}
