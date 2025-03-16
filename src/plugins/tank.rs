use super::asset_loader::ImageAssets;
use super::movement::{Acceleration, MovingObjectBundle};
use crate::components::tank::{Tank, Velocity};
use bevy::prelude::*;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const STARTING_VELOCITY: Vec3 = Vec3::ZERO;
const TANK_SPEED: f32 = 150.0;
const TANK_ROTATION_SPEED: f32 = 2.0;

pub struct TankPlugin;
impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_tank)
            .add_systems(Update, handle_movement);
    }
}

fn spawn_tank(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity {
                value: STARTING_VELOCITY,
            },
            acceleration: Acceleration { value: Vec3::ZERO },
            transform: Transform {
                translation: STARTING_TRANSLATION,
                scale: super::DEFAULT_SCALE,
                ..default()
            },
            model: Sprite {
                image: image_assets.tank.clone(),
                ..default()
            },
        },
        Tank,
    ));
}

fn handle_movement(
    mut query: Query<(&mut Transform, &mut Velocity), With<Tank>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = TANK_ROTATION_SPEED * time.delta_secs();
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        rotation = -TANK_ROTATION_SPEED * time.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        movement = -TANK_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyW) {
        movement = TANK_SPEED;
    }

    transform.rotate_z(rotation);

    velocity.value = transform.up() * movement;
}
