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
pub struct Opponent;
