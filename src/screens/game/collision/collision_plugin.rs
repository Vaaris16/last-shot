use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    GameState,
    screens::game::{
        enemies::enemies_plugin::Enemy,
        game_plugin::GameSet,
        game_ui::{
            score::spawn_score_text::{Score, ScoreText, update_score},
            space_ship_lifes::spawn_space_ship_lifes::{
                SpaceShipLifes, SpaceShipLifesIndicator, decrement_lifes,
            },
        },
        space_ship::{space_ship_controls::Bullet, spawn_space_ship::SpaceShip},
    },
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_collision_spaceship_enemies,
                check_collision_bullet_enemies,
            )
                .in_set(GameSet),
        );
    }
}

pub fn check_collision_spaceship_enemies(
    spaceship: Query<Entity, With<SpaceShip>>,
    enemies: Query<Entity, With<Enemy>>,
    mut collision_events: MessageReader<CollisionStart>,
    life_indicator: Query<(Entity, &SpaceShipLifesIndicator)>,
    mut lifes: ResMut<SpaceShipLifes>,
    mut game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    for events in collision_events.read() {
        let entity1 = events.collider1;
        let entity2 = events.collider2;

        if (enemies.contains(entity1) && spaceship.contains(entity2))
            || (enemies.contains(entity2) && spaceship.contains(entity1))
        {
            decrement_lifes(life_indicator, &mut lifes, &mut game_state, &mut commands);
        }
    }
}

pub fn check_collision_bullet_enemies(
    bullets: Query<Entity, With<Bullet>>,
    enemies: Query<Entity, With<Enemy>>,
    mut collision_events: MessageReader<CollisionStart>,
    mut commands: Commands,
    mut score_text: Single<&mut Text, With<ScoreText>>,
    mut score: ResMut<Score>,
) {
    for events in collision_events.read() {
        let entity1 = events.collider1;
        let entity2 = events.collider2;

        let (_bullet_entity, enemy_entity) =
            if bullets.contains(entity1) && enemies.contains(entity2) {
                (entity1, entity2)
            } else if bullets.contains(entity2) && enemies.contains(entity1) {
                (entity2, entity1)
            } else {
                continue;
            };

        commands.entity(enemy_entity).despawn();
        update_score(&mut *score_text, &mut score);
    }
}
