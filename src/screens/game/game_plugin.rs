use bevy::prelude::*;

use crate::{
    GameState,
    screens::game::{
        collision::collision_plugin::CollisionPlugin,
        enemies::enemies_plugin::{EnemiesPlugin, Enemy},
        game_ui::{
            game_ui_plugin::{GameUi, GameUiPlugin},
            remaining_bullets::remaining_bullets_text::RemainingBullets,
            score::spawn_score_text::Score,
            space_ship_lifes::spawn_space_ship_lifes::SpaceShipLifes,
        },
        space_ship::{
            space_ship_controls::Bullet,
            space_ship_plugin::SpaceShipPlugin,
            spawn_space_ship::SpaceShip,
        },
        walls::walls_plugin::{Wall, WallsPlugin},
    },
};

pub struct GamePlugin;

#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct GameSet;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, GameSet.run_if(in_state(GameState::Game)));
        app.add_plugins((
            SpaceShipPlugin,
            GameUiPlugin,
            WallsPlugin,
            EnemiesPlugin,
            CollisionPlugin,
        ))
        .add_systems(OnExit(GameState::Game), cleanup_game)
        .add_systems(OnEnter(GameState::SplashScreen), reset_game_resources);
    }
}

fn cleanup_game(
    game_ui: Single<Entity, With<GameUi>>,
    enemies: Query<Entity, With<Enemy>>,
    mut commands: Commands,
    space_ship: Single<Entity, With<SpaceShip>>,
    walls: Query<Entity, With<Wall>>,
    bullets: Query<Entity, With<Bullet>>,
) {
    for enemy_entity in enemies {
        commands.entity(enemy_entity).despawn();
    }
    for bullet_entity in bullets {
        commands.entity(bullet_entity).despawn();
    }
    for wall_entity in walls {
        commands.entity(wall_entity).despawn();
    }
    commands.entity(*space_ship).despawn();
    commands.entity(*game_ui).despawn();
}

pub fn reset_game_resources(
    mut score: ResMut<Score>,
    mut remaining_bullets: ResMut<RemainingBullets>,
    mut spaceship_lifes: ResMut<SpaceShipLifes>,
) {
    score.0 = 0;
    remaining_bullets.0 = 1;
    spaceship_lifes.0 = 3;
}
