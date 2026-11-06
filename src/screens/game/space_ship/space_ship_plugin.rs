use bevy::prelude::*;

use crate::{
    GameState,
    screens::game::space_ship::{
        space_ship_controls::{maintain_bullet, shoot_bullet, spaceship_controls},
        spawn_space_ship::{keep_spaceship_in_bounds, spawn_space_spaceship},
    },
};

pub struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_space_spaceship)
            .add_systems(
                Update,
                (
                    maintain_bullet,
                    spaceship_controls,
                    keep_spaceship_in_bounds,
                    shoot_bullet,
                ),
            );
    }
}
