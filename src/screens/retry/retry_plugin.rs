use bevy::{
    prelude::*,
    text::FontSourceTemplate,
    ui_widgets::{Activate, Button},
};

use crate::{
    GameState, core::game_fonts::game_fonts_resource::GameFonts,
    screens::game::game_ui::score::spawn_score_text::Score,
};

pub struct RetryPlugin;

impl Plugin for RetryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Retry), game_over_ui)
            .add_systems(OnExit(GameState::Retry), cleanup_retry);
    }
}

#[derive(Component, FromTemplate)]
pub struct RetryUi;

pub fn game_over_ui(score: Res<Score>, mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: px(30),
        }
        RetryUi
        Children [
            @game_over_title()
            --
            @game_over_score(score.0)
            --
            @restart_button()
        ]
    });
}

pub fn game_over_title() -> impl Scene {
    let font = GameFonts::OxaniumBold.font_path();

    bsn! {
        Text("GAME OVER")

        TextFont {
            font: FontSourceTemplate::Handle(font),
            font_size: px(100),
        }

        TextColor(Color::WHITE)
    }
}

pub fn game_over_score(score: i64) -> impl Scene {
    let font = GameFonts::OxaniumMedium.font_path();
    bsn! {
        Text({ format!("SCORE: {:06}", score) })

        TextFont {
            font: FontSourceTemplate::Handle(font),
            font_size: px(64),
        }

        TextColor(Color::WHITE)
    }
}

pub fn restart_button() -> impl Scene {
    let font = GameFonts::OxaniumBold.font_path();
    bsn! {
        Button
        Node {
            padding: px(20),
            border: px(2),
            border_radius: BorderRadius::all(px(10)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }

        BorderColor::all(Color::WHITE)
        BackgroundColor(Color::BLACK)

        on(|_activate: On<Activate>, mut next_state: ResMut<NextState<GameState>>| {
            next_state.set(GameState::SplashScreen);
        })

        Children [
            Text("RESTART")
            TextFont {
                font: FontSourceTemplate::Handle(font),
                font_size: px(20)
            }
            TextColor(Color::WHITE)
        ]
    }
}

fn cleanup_retry(retry_screen_ui: Single<Entity, With<RetryUi>>, mut commands: Commands) {
    commands.entity(*retry_screen_ui).despawn();
}
