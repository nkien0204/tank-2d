use super::collision_detection::CollisionEvent;
use super::map::{OutOfBoundTypes, check_out_of_bounds};
use crate::components::tank::Tank;
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
    mut collision_event_reader: EventReader<CollisionEvent>,
    query: Query<Entity, With<Tank>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in collision_event_reader.read() {
        if query.get_single().is_err() {
            // Tank was destroyed, game over
            next_state.set(GameState::GameOver);
        }
    }
}
