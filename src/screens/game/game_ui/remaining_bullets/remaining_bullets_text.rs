use bevy::{prelude::*, state::commands};

use crate::screens::game::game_ui::remaining_bullets::remaining_bullets_text;

#[derive(Resource)]
pub struct RemainingBullets(pub i32);

#[derive(Resource)]
pub struct RemainingBulletsTimer(Timer);

#[derive(Component, FromTemplate)]
pub struct RemainingBulletsText;

impl Default for RemainingBulletsTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(15.0, TimerMode::Repeating))
    }
}

pub fn spawn_remaining_bullets() -> impl Scene {
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            right: px(25),
            column_gap: px(5)
        }
        Children [
            @bullet_image()
            --
            @remaining_bullets_text()
        ]
    }
}

fn remaining_bullets_text() -> impl Scene {
    bsn! {
        Text("0")
        TextFont {
            font_size: px(32),
        }
        TextColor(Color::WHITE)
        RemainingBulletsText
    }
}

fn bullet_image() -> impl Scene {
    bsn! {
        Node {
            width: px(40),
            height: px(40),
        }
        ImageNode {
            image: "bullet.png",
        }
    }
}

pub fn increment_remaining_bullets(
    time: Res<Time>,
    mut timer: ResMut<RemainingBulletsTimer>,
    mut remaining_bullets: ResMut<RemainingBullets>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        remaining_bullets.0 += 1;
    }
}

pub fn decrement_remaining_bullets(mut remaining_bullets: ResMut<RemainingBullets>) {
    if remaining_bullets.0 > 0 {
        remaining_bullets.0 -= 1;
    }
}

pub fn update_remaining_bullets_text(
    mut remaining_bullets_text: Single<&mut Text, With<RemainingBulletsText>>,
    remaining_bullets: Res<RemainingBullets>,
) {
    let remaining_bullets_string = format!("{}", remaining_bullets.0);
    remaining_bullets_text.0 = remaining_bullets_string;
}
