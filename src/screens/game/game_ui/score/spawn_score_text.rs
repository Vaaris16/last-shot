use std::time::Duration;

use bevy::{log::tracing_subscriber::fmt::format, prelude::*, text::FontSourceTemplate};

use crate::core::game_fonts::game_fonts_resource::GameFonts;

#[derive(Component, FromTemplate)]
pub struct ScoreText;

#[derive(Resource)]
pub struct Score(pub i64);

#[derive(Resource)]
pub struct ScoreTimer(Timer);

impl Default for ScoreTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::new(1, 0), TimerMode::Repeating))
    }
}

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
        Text("Score: 0")
        TextFont {
            font_size: px(64),
            font: FontSourceTemplate::Handle(score_text_font),
        }
        TextColor(Color::WHITE)
        ScoreText
    }
}

pub fn update_score_bonus(score_text: &mut Text, score: &mut Score) {
    score.0 += 5;
    let text = format!("Score: {}", score.0.to_string());
    score_text.0 = text;
}

pub fn update_score(
    time: Res<Time>,
    mut score_text: Single<&mut Text, With<ScoreText>>,
    mut score: ResMut<Score>,
    mut score_timer: ResMut<ScoreTimer>,
) {
    score_timer.0.tick(time.delta());

    if score_timer.0.just_finished() {
        score.0 += 1;
        let text = format!("Score: {}", score.0.to_string());
        score_text.0 = text
    }
}
