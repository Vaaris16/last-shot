use crate::{GameState, screens::game::game_ui::space_ship_lifes};
use bevy::prelude::*;

#[derive(Resource)]
pub struct SpaceShipLifes(pub i32);

#[derive(Component, FromTemplate)]
pub struct SpaceShipLifeUi;

#[derive(Component, FromTemplate)]
pub struct SpaceShipLifesIndicator {
    index: i32,
}

pub fn spaceship_lifes() -> impl Scene {
    bsn! {
        SpaceShipLifeUi
        Node {
            position_type: PositionType::Absolute,
            top: px(20),
            left: px(50),
            column_gap: px(30),
            justify_content: JustifyContent::FlexEnd,
        }

        Children [
                ImageNode {
                    image: "space_ship_lifes.png"
                }
                SpaceShipLifesIndicator { index: 1 }
                Node {
                    width: px(65),
                    height: px(65),
                }
            --
                ImageNode {
                    image: "space_ship_lifes.png"
                }
                SpaceShipLifesIndicator { index: 2 }
                Node {
                    width: px(65),
                    height: px(65),
                }
            --
                ImageNode {
                    image: "space_ship_lifes.png"
                }
                SpaceShipLifesIndicator { index: 3 }
                Node {
                    width: px(65),
                    height: px(65),
                }
        ]
    }
}

pub fn decrement_lifes(
    life_indicator: Query<(Entity, &SpaceShipLifesIndicator)>,
    lifes: &mut SpaceShipLifes,
    game_state: &mut NextState<GameState>,
    commands: &mut Commands,
) {
    lifes.0 -= 1;

    if lifes.0 == 0 {
        game_state.set(GameState::Retry);
    }

    for (indicator_entity, indicator) in &life_indicator {
        if indicator.index > lifes.0 {
            commands.entity(indicator_entity).despawn();
        }
    }
}

pub fn enter_retry_screen(
    mut next_state: ResMut<NextState<GameState>>,
    space_ship_lifes: Res<SpaceShipLifes>,
) {
    if space_ship_lifes.0 == 0 {
        next_state.set(GameState::Retry);
    }
}
