use bevy::prelude::*;

pub fn spawn_space_spaceship(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Sprite {
            image: "space_ship.png",
        }
        Transform::from_xyz(0., 0., 0.)
    });
}
