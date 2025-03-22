use crate::components::{
    collider::Collider,
    tank::{Opponent, Tank, TankShell},
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

fn detect_collision(mut query: Query<(Entity, &GlobalTransform, &mut Collider, &Name)>) {
    let mut colliding_entity_map: HashMap<Entity, Vec<(Entity, Name)>> = HashMap::new();

    for (entity_a, transform_a, collider_a, _) in query.iter() {
        for (entity_b, transform_b, collider_b, name_b) in query.iter() {
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
                    .push((entity_b, name_b.clone()));
            }
        }
    }

    for (entity, _, mut collider, _name) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(colliding_entities) = colliding_entity_map.get(&entity) {
            collider
                .colliding_entities
                .extend(colliding_entities.clone());
        }
    }
}

fn handle_collisions<T: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Name, &Collider), With<T>>,
) {
    for (entity, name, collider) in query.iter() {
        for colliding_entity in collider.colliding_entities.iter() {
            if query.get(colliding_entity.0).is_ok() {
                continue;
            }
            if name.as_str() == colliding_entity.1.as_str() {
                continue;
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}
