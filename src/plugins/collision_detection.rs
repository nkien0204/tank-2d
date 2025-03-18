use crate::components::{
    collider::Collider,
    tank::{Opponent, OpponentGun, Tank, TankGun, TankShell},
};
use bevy::prelude::*;
use std::collections::HashMap;

use super::game_state::GameState;

pub struct CollisionDetectionPlugin;
impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                detect_collision,
                handle_collisions::<Tank>,
                handle_collisions::<Opponent>,
                handle_collisions::<TankShell>,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn detect_collision(
    mut query: Query<
        (Entity, &GlobalTransform, &mut Collider),
        (Without<TankGun>, Without<OpponentGun>),
    >,
) {
    let mut colliding_entity_map: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a == entity_b {
                continue;
            }

            let distance = transform_a
                .translation()
                .distance(transform_b.translation());
            let collision_distance = collider_a.radius + collider_b.radius;

            if distance < collision_distance {
                colliding_entity_map
                    .entry(entity_a)
                    .or_insert_with(Vec::new)
                    .push(entity_b);
            }
        }
    }

    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(colliding_entities) = colliding_entity_map.get(&entity) {
            collider
                .colliding_entities
                .extend(colliding_entities.iter().copied());
        }
    }
}

fn handle_collisions<T: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in query.iter() {
        for &colliding_entity in collider.colliding_entities.iter() {
            if query.get(colliding_entity).is_ok() {
                continue;
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}
