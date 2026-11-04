use bevy::prelude::*;

use crate::GAME_BACKGROUND_COLOR;

pub fn set_bg_color(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = GAME_BACKGROUND_COLOR;
}
