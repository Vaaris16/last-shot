use bevy::{math::VectorSpace, prelude::*};

use crate::screens::game::space_ship::spawn_space_ship::SpaceShip;

const SPEED: f32 = 300.;

pub fn move_spaceship(
    keyboard: Res<ButtonInput<KeyCode>>,
    window: Single<&Window>,
    mut space_ship: Single<(&mut Transform, &Sprite), With<SpaceShip>>,
    time: Res<Time>,
) {
    let x_bound = window.width() / 2.;
    let y_bound = window.height() / 2.;

    let movement = SPEED * time.delta_secs();

    let space_ship_size = space_ship.1.custom_size.unwrap_or(Vec2::ZERO);

    if keyboard.pressed(KeyCode::ArrowUp) {
        space_ship.0.translation.y =
            (space_ship.0.translation.y + movement).min(y_bound - space_ship_size.y / 2.);
    }

    if keyboard.pressed(KeyCode::ArrowDown) {
        space_ship.0.translation.y =
            (space_ship.0.translation.y - movement).max(-y_bound + space_ship_size.y / 2.);
    }

    if keyboard.pressed(KeyCode::ArrowRight) {
        space_ship.0.translation.x =
            (space_ship.0.translation.x + movement).min(x_bound - space_ship_size.x / 2.);
    }

    if keyboard.pressed(KeyCode::ArrowLeft) {
        space_ship.0.translation.x =
            (space_ship.0.translation.x - movement).max(-x_bound + space_ship_size.x / 2.);
    }
}
