use bevy::prelude::*;

use crate::{
    GameState,
    screens::game::game_ui::score::spawn_score_text::{Score, spawn_score, update_score},
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
    }
}
