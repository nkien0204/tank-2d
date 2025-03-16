use super::asset_loader::ImageAssets;
use super::movement::{Acceleration, MovingObjectBundle};
use crate::components::tank::{Opponent, Velocity};
use bevy::prelude::*;
use rand::Rng;
use std::ops::Range;

const SPAWN_RANGE_X: Range<f32> = -500.0..500.0;
const SPAWN_RANGE_Y: Range<f32> = -500.0..500.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;

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
        .add_systems(Update, spawn_opponent);
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

    let translation = Vec3::new(
        rng.random_range(SPAWN_RANGE_X.clone()),
        rng.random_range(SPAWN_RANGE_Y.clone()),
        0.0,
    );
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

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity { value: velocity },
            acceleration: Acceleration {
                value: acceleration,
            },
            transform: Transform {
                translation,
                scale: super::DEFAULT_SCALE,
                ..default()
            },
            model: Sprite {
                image: image_assets.opponent.clone(),
                ..default()
            },
        },
        Opponent,
    ));
}
