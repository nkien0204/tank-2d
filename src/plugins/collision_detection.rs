use crate::components::collider::Collider;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct CollisionDetectionPlugin;
impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_collision);
    }
}

fn detect_collision(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
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
