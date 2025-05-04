use super::map::{OutOfBoundTypes, check_out_of_bounds};
use crate::components::tank::TankObjectType;
use crate::components::tank::{Enemy, Tank};
use crate::plugins::game_state::GameState;
use bevy::prelude::*;

pub struct DespawnPlugin;
impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_entities, check_tank_destroyed).run_if(in_state(GameState::InGame)),
        );
    }
}

fn despawn_entities(
    mut commands: Commands,
    sprites: Res<Assets<Image>>,
    query: Query<(Entity, &GlobalTransform, &Sprite)>,
) {
    for (entity, transform, sprite) in query.iter() {
        let Some(image) = sprites.get(sprite.image.id()) else {
            continue;
        };
        let translation = transform.translation();
        match check_out_of_bounds(translation, image.size(), 0.0) {
            OutOfBoundTypes::None => {}
            _ => {
                commands.entity(entity).despawn_recursive();
            }
        };
    }
}

fn check_tank_destroyed(
    mut commands: Commands,
    query: Query<(Entity, &TankObjectType), With<Tank>>,
    enemies_query: Query<Entity, With<Enemy>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut noti_text = "";
    if enemies_query.iter().count() == 0 {
        next_state.set(GameState::GameVictory);
        noti_text = "Victory!";
    }
    if query.iter().count() == 0 {
        next_state.set(GameState::GameOver);
        noti_text = "Game Over!";
    }

    if noti_text != "" {
        commands.spawn((
            Text2d::new(noti_text),
            TextColor::WHITE,
            TextFont {
                font_size: 60.0,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
        ));
    }
}
