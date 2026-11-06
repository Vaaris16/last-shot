use bevy::prelude::*;

use crate::{
    GameState,
    screens::game::game_ui::{
        remaining_bullets::remaining_bullets_text::{
            RemainingBullets, RemainingBulletsTimer, increment_remaining_bullets,
            spawn_remaining_bullets, update_remaining_bullets_text,
        },
        score::spawn_score_text::{Score, spawn_score, update_score},
        space_ship_lifes::spawn_space_ship_lifes::{SpaceShipLifes, spaceship_lifes},
    },
};

pub struct GameUiPlugin;

#[derive(SystemSet, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct GameUiSet;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, GameUiSet.run_if(in_state(GameState::Game)));
        app.add_systems(OnEnter(GameState::Game), spawn_score)
            .insert_resource(Score(0))
            .add_systems(Update, update_score.in_set(GameUiSet));

        app.add_systems(OnEnter(GameState::Game), spawn_remaining_bullets)
            .add_systems(
                Update,
                (increment_remaining_bullets, update_remaining_bullets_text).in_set(GameUiSet),
            )
            .insert_resource(RemainingBulletsTimer::default())
            .insert_resource(RemainingBullets(1));

        app.insert_resource(SpaceShipLifes(3))
            .add_systems(OnEnter(GameState::Game), spaceship_lifes);
    }
}
