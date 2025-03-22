use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

#[derive(Component, Debug)]
pub struct Tank;

#[derive(Component, Debug)]
pub struct TankGun;

#[derive(Component, Debug)]
pub struct TankShell;

#[derive(Component, Debug)]
pub struct Opponent;

#[derive(Component, Debug)]
pub struct OpponentShell;

#[derive(Component, Debug)]
pub struct OpponentGun;

#[derive(Bundle)]
pub struct GunBundle {
    pub transform: Transform,
    pub model: Sprite,
}
