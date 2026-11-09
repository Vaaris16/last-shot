use std::time::Duration;

use crate::screens::game::{
    game_ui::remaining_bullets::remaining_bullets_text::{
        RemainingBullets, decrement_remaining_bullets,
    },
    space_ship::spawn_space_ship::SpaceShip,
};
use avian2d::prelude::*;
use bevy::{math::VectorSpace, prelude::*};

pub fn spaceship_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    space_ship: Single<(&mut LinearVelocity, &mut AngularVelocity), With<SpaceShip>>,
) {
    let (mut velocity, mut angular_velocity) = space_ship.into_inner();

    velocity.x = 0.;
    velocity.y = 0.;
    angular_velocity.0 = 0.;

    if keyboard.pressed(KeyCode::KeyW) {
        velocity.y = 300.;
    }

    if keyboard.pressed(KeyCode::KeyS) {
        velocity.y = -300.;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        velocity.x = 300.;
    }

    if keyboard.pressed(KeyCode::KeyA) {
        velocity.x = -300.;
    }

    if keyboard.pressed(KeyCode::ArrowLeft) {
        angular_velocity.0 = 3.;
    }

    if keyboard.pressed(KeyCode::ArrowRight) {
        angular_velocity.0 = -3.;
    }
}

#[derive(Component)]
pub struct Bullet {
    timer: Timer,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::new(6, 0), TimerMode::Once),
        }
    }
}

pub fn shoot_bullet(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    spaceship: Single<&Transform, With<SpaceShip>>,
    asset_server: Res<AssetServer>,
    remaining_bullets: ResMut<RemainingBullets>,
) {
    if keyboard.just_pressed(KeyCode::Space) && remaining_bullets.0 != 0 {
        let spaceship = *spaceship;

        let direction = spaceship.rotation * Vec3::Y;
        let bullet_position = spaceship.translation + direction * 30.0;

        commands.spawn((
            Sprite {
                image: asset_server.load("bullet.png"),
                custom_size: Some(Vec2::new(40., 40.)),
                ..Default::default()
            },
            Transform {
                translation: bullet_position,
                rotation: spaceship.rotation,
                ..default()
            },
            RigidBody::Dynamic,
            Collider::circle(5.),
            LinearVelocity(direction.truncate() * 700.),
            GravityScale(0.),
            Bullet::default(),
            Restitution::new(1.5),
        ));

        decrement_remaining_bullets(remaining_bullets);
    }
}

pub fn maintain_bullet(
    mut commands: Commands,
    time: Res<Time>,
    bullets: Query<(&mut Bullet, Entity), With<Bullet>>,
) {
    for (mut bullet, bullet_entity) in bullets {
        bullet.timer.tick(time.delta());
        if bullet.timer.just_finished() {
            commands.entity(bullet_entity).despawn();
        }
    }
}
