use avian2d::{PhysicsPlugins, debug_render::PhysicsDebugPlugin};
use bevy::{prelude::*, window::WindowMode};

use crate::{core::core_plugin::CorePlugin, screens::screens_plugin::ScreensPlugin};

mod core;
mod screens;

pub const GAME_BACKGROUND_COLOR: Color = Color::BLACK;
pub const SECONDARY_TEXT_COLOR: Color = Color::hsl(359., 0.01, 0.32);

#[derive(States, Debug, Clone, Hash, Default, PartialEq, Eq)]
pub enum GameState {
    SplashScreen,
    #[default]
    Game,
    Retry,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            CorePlugin,
            ScreensPlugin,
            PhysicsPlugins::default(),
        ))
        .init_state::<GameState>()
        .run();
}
