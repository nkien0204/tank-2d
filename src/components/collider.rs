use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
}

#[derive(Component, Debug)]
pub struct Explosion;
