use super::asset_loader::ImageAssets;
use super::game_state::GameState;
use super::movement::{Acceleration, MovingObjectBundle};
use crate::components::tank::GunBundle;
use crate::components::{
    collider::Collider,
    tank::{Opponent, OpponentGun, Velocity},
};
use bevy::prelude::*;
use rand::Rng;
use std::ops::Range;

const SPAWN_RANGE_X: Range<f32> = -500.0..500.0;
const SPAWN_RANGE_Y: Range<f32> = -500.0..500.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;
const OPPONENT_RADIUS: f32 = 10.0;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    pub timer: Timer,
}

pub struct OpponentPlugin;
impl Plugin for OpponentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(Update, spawn_opponent.run_if(in_state(GameState::InGame)));
    }
}

fn spawn_opponent(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    image_assets: Res<ImageAssets>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
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
        scale: super::DEFAULT_SCALE,
        ..default()
    };

    let rotate_angle = calculate_angle(Vec3::new(0.0, 1.0, 0.0), velocity);
    if velocity.x < 0.0 {
        transform.rotate_z(rotate_angle);
    } else {
        transform.rotate_z(-rotate_angle);
    }

    let mut gun_transform = transform.clone();
    gun_transform.translation += Vec3::new(0.0, 2.0, 0.1);

    commands
        .spawn((
            MovingObjectBundle {
                velocity: Velocity { value: velocity },
                acceleration: Acceleration {
                    value: acceleration,
                },
                collider: Collider {
                    radius: OPPONENT_RADIUS,
                    colliding_entities: Vec::new(),
                },
                transform,
                model: Sprite {
                    image: image_assets.opponent.clone(),
                    ..default()
                },
            },
            Opponent,
        ))
        .with_child((
            GunBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 10.0, 1.0), // set z-index to 1.0
                    ..default()
                },
                model: Sprite {
                    image: image_assets.opponent_gun.clone(),
                    ..default()
                },
            },
            OpponentGun,
        ));
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
