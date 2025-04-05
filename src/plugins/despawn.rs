use super::collision_detection::CollisionEvent;
use crate::components::tank::Tank;
use crate::plugins::game_state::GameState;
use bevy::prelude::*;

const DESPAWN_DISTANCE: f32 = 1000.0;

pub struct DespawnPlugin;
impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_entities, check_tank_destroyed).run_if(in_state(GameState::InGame)),
        );
    }
}

fn despawn_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);
        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
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
