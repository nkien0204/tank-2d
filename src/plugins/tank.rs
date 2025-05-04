use super::asset_loader::ImageAssets;
use super::game_state::{GameState, PluginName, PluginReadyEvent};
use super::movement::{Acceleration, MovingObjectBundle};
use super::{ALLIES_TAG_NAME, SHELL_FORWARD_SPAWN_SCALAR, SHELL_RADIUS, SHELL_SPEED};
use crate::components::{
    collider::Collider,
    tank::{Tank, TankObjectType, TankShell, Velocity},
};
use bevy::prelude::*;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const STARTING_VELOCITY: Vec3 = Vec3::ZERO;

const TANK_SPEED: f32 = 150.0;
const TANK_ROTATION_SPEED: f32 = 2.0;
const TANK_RADIUS: f32 = 50.0;

pub struct TankPlugin;
impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_tank).add_systems(
            Update,
            (handle_movement, handle_tank_shell).run_if(in_state(GameState::InGame)),
        );
    }
}

fn spawn_tank(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut plugin_state_event: EventWriter<PluginReadyEvent>,
) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity {
                value: STARTING_VELOCITY,
            },
            collider: Collider {
                radius: TANK_RADIUS * super::DEFAULT_SCALE,
            },
            acceleration: Acceleration { value: Vec3::ZERO },
            transform: Transform {
                translation: STARTING_TRANSLATION,
                scale: Vec3::new(
                    super::DEFAULT_SCALE,
                    super::DEFAULT_SCALE * -1.0,
                    super::DEFAULT_SCALE,
                ),
                ..default()
            },
            model: Sprite {
                image: image_assets.tank.clone(),
                ..default()
            },
        },
        Tank,
        Name::new(ALLIES_TAG_NAME),
        TankObjectType::Tank,
    ));
    plugin_state_event.send(PluginReadyEvent {
        plugin_name: PluginName::Tank,
    });
}

fn handle_movement(
    mut query: Query<(&mut Transform, &mut Velocity), With<Tank>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
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
                    radius: SHELL_RADIUS * super::DEFAULT_SCALE,
                },
                acceleration: Acceleration { value: Vec3::ZERO },
                transform: Transform {
                    translation: transform.translation
                        + transform.up() * SHELL_FORWARD_SPAWN_SCALAR,
                    scale: Vec3::new(
                        super::DEFAULT_SCALE,
                        super::DEFAULT_SCALE,
                        super::DEFAULT_SCALE,
                    ),
                    rotation: transform.rotation,
                    ..default()
                },
                model: Sprite {
                    image: image_assets.shell.clone(),
                    ..default()
                },
            },
            TankShell,
            Name::new(ALLIES_TAG_NAME),
            TankObjectType::Shell,
        ));
    }
}
