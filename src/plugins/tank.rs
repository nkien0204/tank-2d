use super::asset_loader::ImageAssets;
use super::movement::{Acceleration, MovingObjectBundle};
use crate::components::{
    collider::Collider,
    tank::{Tank, TankShell, Velocity},
};
use bevy::prelude::*;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const STARTING_VELOCITY: Vec3 = Vec3::ZERO;

const TANK_SPEED: f32 = 150.0;
const TANK_ROTATION_SPEED: f32 = 2.0;
const TANK_RADIUS: f32 = 10.0;

const SHELL_SPEED: f32 = 500.0;
const SHELL_FORWARD_SPAWN_SCALAR: f32 = 20.0;
const SHELL_RADIUS: f32 = 10.0;

pub struct TankPlugin;
impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_tank)
            .add_systems(Update, (handle_movement, handle_tank_shell));
    }
}

fn spawn_tank(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity {
                value: STARTING_VELOCITY,
            },
            collider: Collider {
                radius: TANK_RADIUS,
                colliding_entities: Vec::new(),
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
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };
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

fn handle_tank_shell(
    mut commands: Commands,
    query: Query<&Transform, With<Tank>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    image_assets: Res<ImageAssets>,
) {
    let Ok(transform) = query.get_single() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::KeyJ) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity {
                    value: transform.up() * SHELL_SPEED,
                },
                collider: Collider {
                    radius: SHELL_RADIUS,
                    colliding_entities: Vec::new(),
                },
                acceleration: Acceleration { value: Vec3::ZERO },
                transform: Transform {
                    translation: transform.translation
                        + transform.up() * SHELL_FORWARD_SPAWN_SCALAR,
                    scale: super::DEFAULT_SCALE,
                    rotation: transform.rotation,
                    ..default()
                },
                model: Sprite {
                    image: image_assets.shell.clone(),
                    ..default()
                },
            },
            TankShell,
        ));
    }
}
