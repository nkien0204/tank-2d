use crate::components::tank::{Tank, TankGun};
use crate::plugins::game_state::GameState;
use bevy::prelude::*;

const DESPAWN_DISTANCE: f32 = 1000.0;

pub struct DespawnPlugin;
impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_entities, check_tank_destroyed).run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            handle_game_over.run_if(in_state(GameState::GameOver)),
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
    query: Query<Entity, With<Tank>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if query.get_single().is_err() {
        // Tank was destroyed, game over
        next_state.set(GameState::GameOver);
    }
}

fn handle_game_over(mut commands: Commands, query: Query<Entity, With<TankGun>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
