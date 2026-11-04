use bevy::{
    prelude::*,
    text::FontSourceTemplate,
    ui_widgets::{Activate, Button},
};

use crate::{GameState, SECONDARY_TEXT_COLOR, core::game_fonts::game_fonts_resource::GameFonts};

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashScreen), spawn_splash_screen_ui)
            .add_systems(OnExit(GameState::SplashScreen), cleanup_splash_screen);
    }
}

#[derive(Component, FromTemplate)]
struct SplashScreenUi;

fn spawn_splash_screen_ui(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
        }
        SplashScreenUi
        Children [
            @splash_title()
            --
            @press_to_start()
        ]
    });
}

fn splash_title() -> impl Scene {
    let splash_title_font = GameFonts::OxaniumBold.font_path();
    bsn! {
        Text("LAST SHOT")
        TextFont {
            font_size: px(150),
            font: FontSourceTemplate::Handle(splash_title_font),
        }
        TextColor(Color::WHITE)
    }
}

fn press_to_start() -> impl Scene {
    let start_game_font = GameFonts::OxaniumMedium.font_path();
    bsn! {
        Button
        Text("PRESS TO START")
        TextFont {
            font_size: px(35),
            font: FontSourceTemplate::Handle(start_game_font),
        }
        TextColor(SECONDARY_TEXT_COLOR)
        on(start_game)
    }
}

fn start_game(_activate: On<Activate>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Game);
}

fn cleanup_splash_screen(
    splash_screen_ui: Single<Entity, With<SplashScreenUi>>,
    mut commands: Commands,
) {
    commands.entity(*splash_screen_ui).despawn();
}
