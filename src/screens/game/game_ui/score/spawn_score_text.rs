use bevy::{prelude::*, text::FontSourceTemplate};

use crate::core::game_fonts::game_fonts_resource::GameFonts;

#[derive(Component, FromTemplate)]
pub struct ScoreText;

#[derive(Resource)]
pub struct Score(pub i64);

pub fn spawn_score() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
        }
        Children [
            @score_text()
        ]
    }
}

fn score_text() -> impl Scene {
    let score_text_font = GameFonts::OxaniumMedium.font_path();

    bsn! {
        Text("Score Score")
        TextFont {
            font_size: px(64),
            font: FontSourceTemplate::Handle(score_text_font),
        }
        TextColor(Color::WHITE)
        ScoreText
    }
}

pub fn update_score(mut score_text: Single<&mut Text, With<ScoreText>>, mut score: ResMut<Score>) {
    score.0 += 1;
    let text = format!("Score: {}", score.0.to_string());
    score_text.0 = text;
}
