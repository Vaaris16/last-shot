use crate::screens::game::space_ship::spawn_space_ship::SpaceShip;
use avian2d::prelude::*;
use bevy::{math::VectorSpace, prelude::*};

const SPEED: f32 = 300.;

pub fn spaceship_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    space_ship: Single<(&mut LinearVelocity, &mut AngularVelocity), With<SpaceShip>>,
) {
    let (mut velocity, mut angular_velocity) = space_ship.into_inner();

    velocity.x = 0.;
    velocity.y = 0.;
    angular_velocity.0 = 0.;

    if keyboard.pressed(KeyCode::ArrowUp) {
        velocity.y = 300.;
    }

    if keyboard.pressed(KeyCode::ArrowDown) {
        velocity.y = -300.;
    }

    if keyboard.pressed(KeyCode::ArrowRight) {
        velocity.x = 300.;
    }

    if keyboard.pressed(KeyCode::ArrowLeft) {
        velocity.x = -300.;
    }

    if keyboard.pressed(KeyCode::KeyA) {
        angular_velocity.0 = 3.;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        angular_velocity.0 = -3.;
    }
}
