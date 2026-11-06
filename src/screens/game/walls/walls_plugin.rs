use avian2d::prelude::*;
use bevy::prelude::*;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_walls);
    }
}

fn spawn_walls(mut commands: Commands, window: Single<&Window>) {
    let width = window.width();
    let height = window.height();
    let thickness = 25.;

    commands.spawn_scene(bsn! {
        RigidBody::Static

        Collider::rectangle(width, thickness)

        Transform::from_xyz(0., height / 2., 0.)
    });

    commands.spawn_scene(bsn! {
        RigidBody::Static

        Collider::rectangle(width, thickness)

        Transform::from_xyz(0., -height / 2., 0.)
    });

    commands.spawn_scene(bsn! {
        RigidBody::Static

        Collider::rectangle(thickness, height)

        Transform::from_xyz(-width / 2., 0., 0.)
    });

    commands.spawn_scene(bsn! {
        RigidBody::Static

        Collider::rectangle(thickness, height)

        Transform::from_xyz(width / 2., 0., 0.)
    });
}
