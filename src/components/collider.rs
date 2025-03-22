use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<(Entity, Name)>,
}
