use bevy::prelude::*;

use crate::{
    GameState,
    screens::game::{
        game_plugin::GameSet,
        game_ui::{
            remaining_bullets::remaining_bullets_text::{
                RemainingBullets, RemainingBulletsTimer, increment_remaining_bullets,
                spawn_remaining_bullets, update_remaining_bullets_text,
            },
            score::spawn_score_text::{Score, spawn_score, update_score},
            space_ship_lifes::spawn_space_ship_lifes::{
                SpaceShipLifes, enter_retry_screen, spaceship_lifes,
            },
        },
    },
};

pub struct GameUiPlugin;

#[derive(Component, FromTemplate)]
pub struct GameUi;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), game_ui)
            .insert_resource(Score(0))
            .add_systems(
                Update,
                (
                    increment_remaining_bullets,
                    update_remaining_bullets_text,
                    enter_retry_screen,
                )
                    .in_set(GameSet),
            )
            .insert_resource(RemainingBulletsTimer::default())
            .insert_resource(RemainingBullets(1))
            .insert_resource(SpaceShipLifes(3));
    }
}

fn game_ui(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Node {
            width: percent(100),
            justify_content: JustifyContent::SpaceBetween,
        }
        GameUi
        Children [
            @spawn_remaining_bullets()
            --
            @spawn_score()
            --
            @spaceship_lifes()
        ]
    });
}
