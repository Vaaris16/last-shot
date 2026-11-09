use std::time::Duration;

use bevy::prelude::*;

use crate::{
    GameState,
    screens::game::{
        game_plugin::GameSet,
        space_ship::{
            space_ship_controls::{maintain_bullet, shoot_bullet, spaceship_controls},
            spawn_space_ship::{keep_spaceship_in_bounds, spawn_space_spaceship},
        },
    },
};

#[derive(Component)]
pub struct Immune {
    immune_timer: Timer,
    blink_timer: Timer,
}

impl Default for Immune {
    fn default() -> Self {
        Self {
            immune_timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
            blink_timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating),
        }
    }
}

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
                    fade_spaceship_immune,
                )
                    .in_set(GameSet),
            );
    }
}

fn fade_spaceship_immune(
    time: Res<Time>,
    mut space_ship: Single<(&mut Sprite, Entity, &mut Immune), With<Immune>>,
    mut commands: Commands,
) {
    space_ship.2.immune_timer.tick(time.delta());
    space_ship.2.blink_timer.tick(time.delta());

    let alpha = space_ship.0.color.alpha();

    if space_ship.2.blink_timer.just_finished() {
        if alpha > 0.5 {
            space_ship.0.color.set_alpha(0.15);
        } else {
            space_ship.0.color.set_alpha(1.0);
        }
    }
    if space_ship.2.immune_timer.just_finished() {
        commands.entity(space_ship.1).remove::<Immune>();
    }
}
