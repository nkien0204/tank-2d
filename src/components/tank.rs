use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

#[derive(Component, Debug)]
pub struct Tank;

#[derive(Component, Debug)]
pub struct TankShell;

#[derive(Component, Debug)]
pub struct Enemy;

#[derive(Component, Debug)]
pub struct EnemyShell;

#[derive(Bundle)]
pub struct GunBundle {
    pub transform: Transform,
    pub model: Sprite,
}

#[derive(Bundle)]
pub struct TrackBundle {
    pub transform: Transform,
    pub model: Sprite,
}

#[derive(Component, Debug)]
pub enum TankObjectType {
    Tank,
    Shell,
}

#[derive(Component, Debug)]
pub struct EnemyDirectionChangeTimer {
    pub moving_timer: Timer,
    pub rotating_timer: Timer,
}

#[derive(Component, Debug)]
pub enum EnemyDirectionChangeState {
    Rotating,
    Moving,
}
