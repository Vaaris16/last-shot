use crate::core::{
    background::{bg_music::play_background_music, set_bg_color::set_bg_color},
    player::spawn_player::spawn_player,
};
use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (set_bg_color, spawn_player))
            .add_systems(Startup, play_background_music);
    }
}
