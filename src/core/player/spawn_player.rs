use bevy::{gltf::gltf::scene::Transform, prelude::*};

pub fn spawn_player(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Camera2d
    });
}
