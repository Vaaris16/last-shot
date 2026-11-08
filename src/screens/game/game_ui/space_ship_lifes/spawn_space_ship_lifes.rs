use crate::GameState;
use bevy::prelude::*;

#[derive(Resource)]
pub struct SpaceShipLifes(pub i32);

#[derive(Component)]
pub struct SpaceShipLifeUi;

#[derive(Component)]
pub struct SpaceShipLifesIndicator {
    index: i32,
}

pub fn spaceship_lifes(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: px(20),
                left: px(50),
                column_gap: px(30),
                justify_content: JustifyContent::FlexEnd,
                ..Default::default()
            },
            SpaceShipLifeUi,
        ))
        .with_children(|life| {
            for i in 1..=3 {
                life.spawn((
                    ImageNode::new(asset_server.load("space_ship_lifes.png")),
                    SpaceShipLifesIndicator { index: i },
                    Node {
                        width: px(65),
                        height: px(65),
                        ..Default::default()
                    },
                ));
            }
        });
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
