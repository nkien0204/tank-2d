use crate::components::tank::TankObjectType;
use crate::components::{collider::Collider, tank::Velocity};
use bevy::prelude::*;

use super::game_state::GameState;
use super::map::{OutOfBoundTypes, check_out_of_bounds};

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub collider: Collider,
    pub acceleration: Acceleration,
    pub transform: Transform,
    pub model: Sprite,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_velocity, update_position).run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_secs();
    }
}

fn update_position(
    sprites: Res<Assets<Image>>,
    mut query: Query<(
        &Velocity,
        &mut Transform,
        &GlobalTransform,
        &Sprite,
        &TankObjectType,
    )>,
    time: Res<Time>,
) {
    for (velocity, mut transform, global_transform, sprite, object_type) in query.iter_mut() {
        let mut padding: f32 = 0.0;
        match object_type {
            TankObjectType::Tank => padding = 4.0,
            _ => {}
        }
        let Some(image) = sprites.get(sprite.image.id()) else {
            continue;
        };
        let translation = global_transform.translation();
        match check_out_of_bounds(translation, image.size(), padding) {
            OutOfBoundTypes::LeftVertical(v) => transform.translation.x = v,
            OutOfBoundTypes::RightVertical(v) => transform.translation.x = v,
            OutOfBoundTypes::LowHorizontal(v) => transform.translation.y = v,
            OutOfBoundTypes::HighHorizontal(v) => transform.translation.y = v,
            OutOfBoundTypes::None => {
                transform.translation += velocity.value * time.delta_secs();
            }
        }
    }
}
