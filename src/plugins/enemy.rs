use super::asset_loader::ImageAssets;
use super::game_state::{GameState, PluginName, PluginReadyEvent};
use super::movement::{Acceleration, MovingObjectBundle};
use super::{ENEMIES_TAG_NAME, SHELL_FORWARD_SPAWN_SCALAR, SHELL_RADIUS, SHELL_SPEED};
use crate::components::tank::{
    EnemyDirectionChangeState, EnemyDirectionChangeTimer, TankObjectType,
};
use crate::components::{
    collider::Collider,
    tank::{Enemy, EnemyShell, Velocity},
};
use bevy::prelude::*;
use rand::Rng;
use std::ops::Range;

const SPAWN_RANGE_X: Range<f32> = -500.0..500.0;
const SPAWN_RANGE_Y: Range<f32> = -500.0..500.0;
const SPAWN_TIME_SECONDS: f32 = 0.2;
const FIRE_SHELL_TIME_SECONDS: f32 = 2.0;
const OPPONENT_RADIUS: f32 = 50.0;

const MAX_ENEMIES: usize = 5;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    pub timer: Timer,
}

#[derive(Resource, Debug)]
pub struct FireShellTimer {
    pub timer: Timer,
}

#[derive(Resource, Debug)]
pub struct StopSpawnEnemy {
    pub stop: bool,
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .insert_resource(StopSpawnEnemy { stop: false })
        .insert_resource(FireShellTimer {
            timer: Timer::from_seconds(FIRE_SHELL_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(
            Update,
            (
                spawn_enemy,
                (
                    handle_shell,
                    update_enemy_direction_state,
                    update_enemy_direction,
                )
                    .run_if(in_state(GameState::InGame)),
            ),
        );
    }
}

fn spawn_enemy(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    query: Query<&Enemy>,
    time: Res<Time>,
    image_assets: Res<ImageAssets>,
    mut stop_spawn_enemy: ResMut<StopSpawnEnemy>,
    mut plugin_state_event: EventWriter<PluginReadyEvent>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    if stop_spawn_enemy.stop {
        return;
    }

    if query.iter().count() >= MAX_ENEMIES {
        stop_spawn_enemy.stop = true;
        plugin_state_event.send(PluginReadyEvent {
            plugin_name: PluginName::Enemy,
        });
        return;
    }

    let mut rng = rand::rng();

    let mut random_unit_vector = || {
        Vec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            0.0,
        )
        .normalize_or_zero()
    };
    let velocity = random_unit_vector() * super::VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * super::ACCELERATION_SCALAR;

    let translation = Vec3::new(
        rng.random_range(SPAWN_RANGE_X.clone()),
        rng.random_range(SPAWN_RANGE_Y.clone()),
        0.0,
    );
    let mut transform = Transform {
        translation,
        scale: Vec3::new(
            super::DEFAULT_SCALE,
            super::DEFAULT_SCALE * -1.0,
            super::DEFAULT_SCALE,
        ),
        ..default()
    };

    let rotate_angle = calculate_angle(Vec3::new(0.0, 1.0, 0.0), velocity);
    if velocity.x < 0.0 {
        transform.rotate_z(rotate_angle);
    } else {
        transform.rotate_z(-rotate_angle);
    }

    let random_moving_timer: f32 = rng.random_range(2.0..3.0);
    let random_rotating_timer: f32 = rng.random_range(0.5..1.5);
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity { value: velocity },
            acceleration: Acceleration {
                value: acceleration,
            },
            collider: Collider {
                radius: OPPONENT_RADIUS * super::DEFAULT_SCALE,
            },
            transform,
            model: Sprite {
                image: image_assets.enemy.clone(),
                ..default()
            },
        },
        EnemyDirectionChangeTimer {
            moving_timer: Timer::from_seconds(random_moving_timer, TimerMode::Repeating),
            rotating_timer: Timer::from_seconds(random_rotating_timer, TimerMode::Repeating),
        },
        EnemyDirectionChangeState::Moving,
        Enemy,
        Name::new(ENEMIES_TAG_NAME),
        TankObjectType::Tank,
    ));
}

fn handle_shell(
    mut commands: Commands,
    query: Query<&Transform, With<Enemy>>,
    image_assets: Res<ImageAssets>,
    mut fire_shell_timer: ResMut<FireShellTimer>,
    time: Res<Time>,
) {
    fire_shell_timer.timer.tick(time.delta());
    if !fire_shell_timer.timer.just_finished() {
        return;
    }

    for transform in query.iter() {
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
            EnemyShell,
            Name::new(ENEMIES_TAG_NAME),
            TankObjectType::Shell,
        ));
    }
}

fn calculate_angle(u: Vec3, v: Vec3) -> f32 {
    let dot_product = u.dot(v);
    let magnitude_u = u.length();
    let magnitude_v = v.length();

    if magnitude_u == 0.0 || magnitude_v == 0.0 {
        return 0.0;
    }

    let cos_theta = (dot_product / (magnitude_u * magnitude_v)).clamp(-1.0, 1.0);
    cos_theta.acos() // This returns the angle in radians
}

fn update_enemy_direction(
    mut query: Query<(&mut Transform, &mut Velocity, &EnemyDirectionChangeState), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity, direction_state) in query.iter_mut() {
        match *direction_state {
            EnemyDirectionChangeState::Rotating => {
                let rotation_angle = 2.0 * time.delta_secs();
                transform.rotate_z(rotation_angle);

                velocity.value = Vec3::new(0.0, 0.0, 0.0);
            }
            EnemyDirectionChangeState::Moving => {
                let rotation = transform.rotation;
                velocity.value = rotation.mul_vec3(Vec3::new(0.0, super::VELOCITY_SCALAR, 0.0));
            }
        }
    }
}

fn update_enemy_direction_state(
    mut query: Query<
        (
            &mut EnemyDirectionChangeTimer,
            &mut EnemyDirectionChangeState,
        ),
        With<Enemy>,
    >,
    time: Res<Time>,
) {
    for (mut direction_change_timer, mut direction_state) in query.iter_mut() {
        match *direction_state {
            EnemyDirectionChangeState::Rotating => {
                direction_change_timer.rotating_timer.tick(time.delta());
                if !direction_change_timer.rotating_timer.just_finished() {
                    continue;
                }
                *direction_state = EnemyDirectionChangeState::Moving;
                direction_change_timer.rotating_timer.reset();
            }
            EnemyDirectionChangeState::Moving => {
                direction_change_timer.moving_timer.tick(time.delta());
                if !direction_change_timer.moving_timer.just_finished() {
                    continue;
                }
                *direction_state = EnemyDirectionChangeState::Rotating;
                direction_change_timer.moving_timer.reset();
            }
        }
    }
}
