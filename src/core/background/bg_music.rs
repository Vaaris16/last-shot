use bevy::prelude::*;

pub fn play_background_music(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        AudioPlayer("background_music.ogg")
        PlaybackSettings::LOOP
    });
}
