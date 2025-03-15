use crate::components::tank::Velocity;
use bevy::prelude::*;

// const STARTING_TRANSLATION: Vec2 = Vec2::new(0.0, 0.0);
const STARTING_VELOCITY: Vec2 = Vec2::new(0.0, 0.0);

#[derive(Bundle)]
struct TankBundle {
    velocity: Velocity,
    model: Sprite,
}

pub struct TankPlugin;
impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tank);
    }
}

fn spawn_tank(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tank_image: Handle<Image> = asset_server.load("tanks/PNG/Hulls_Color_A/Hull_01.png");
    commands.spawn(TankBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        model: Sprite {
            image: tank_image,
            ..default()
        },
    });
}
