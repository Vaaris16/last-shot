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
    });
}
