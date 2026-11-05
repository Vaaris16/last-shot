use avian2d::{parry::math::Vector, prelude::*};
use bevy::prelude::*;

#[derive(Component, FromTemplate)]
pub struct SpaceShip;

pub fn spawn_space_spaceship(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Sprite {
            image: "space_ship.png",
            custom_size: Vec2::new(70., 100.)
        }
        Transform::from_xyz(0., 0., 0.)
        SpaceShip
        RigidBody::Dynamic
        LinearVelocity(Vec2::ZERO)
        AngularVelocity(0.)
        GravityScale(0.)
        Restitution::ZERO
        LinearDamping(0.)
        Collider::triangle(
            Vector::new(0.0, 50.0),
            Vector::new(-35.0, -50.0),
            Vector::new(35.0, -50.0),
        )
    });
}

pub fn keep_spaceship_in_bounds(
    window: Single<&Window>,
    mut spaceship: Single<&mut Transform, With<SpaceShip>>,
) {
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    spaceship.translation.x = spaceship
        .translation
        .x
        .clamp(-half_width + 35.0, half_width - 35.0);

    spaceship.translation.y = spaceship
        .translation
        .y
        .clamp(-half_height + 50.0, half_height - 50.0);
}
