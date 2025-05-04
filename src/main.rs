use bevy::{prelude::*, window::WindowResolution};
use bevy_ecs_tilemap::prelude::*;
use plugins::manual::ManualPlugin;

mod components;
mod plugins;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Tank 2d"),
                        resolution: WindowResolution::new(1288.0, 760.0),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .add_plugins(ManualPlugin)
        .add_plugins(plugins::camera::CameraPlugin)
        .add_plugins(plugins::map::MapPlugin)
        .add_plugins(plugins::asset_loader::AssetLoaderPlugin)
        .add_plugins(plugins::tank::TankPlugin)
        .add_plugins(plugins::enemy::EnemyPlugin)
        .add_plugins(plugins::movement::MovementPlugin)
        .add_plugins(plugins::collision_detection::CollisionDetectionPlugin)
        .add_plugins(plugins::despawn::DespawnPlugin)
        .add_plugins(plugins::game_state::GameStatePlugin)
        // .add_plugins(plugins::debug::DebugPlugin)
        .run();
}
