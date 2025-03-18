use bevy::prelude::*;

pub mod asset_loader;
pub mod camera;
pub mod collision_detection;
pub mod debug;
pub mod despawn;
pub mod game_state;
pub mod movement;
pub mod opponent;
pub mod tank;

const DEFAULT_SCALE: Vec3 = Vec3::new(0.2, 0.2, 1.0);
const VELOCITY_SCALAR: f32 = 100.0;
const ACCELERATION_SCALAR: f32 = 1.0;
