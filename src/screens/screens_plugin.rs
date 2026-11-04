use bevy::prelude::*;

use crate::screens::splash_screen::splash_screen_plugin::SplashScreenPlugin;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SplashScreenPlugin);
    }
}
