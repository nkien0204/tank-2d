use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod components;
mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: String::from(
                    "Basic Example - Press Space to change Texture and H to show/hide tilemap.",
                ),
                ..Default::default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugins(TilemapPlugin)
        .add_plugins(plugins::camera::CameraPlugin)
        .add_plugins(plugins::map::MapPlugin)
        .add_plugins(plugins::tank::TankPlugin)
        .add_plugins(plugins::enemy::EnemyPlugin)
        .add_plugins(plugins::movement::MovementPlugin)
        .add_plugins(plugins::asset_loader::AssetLoaderPlugin)
        .add_plugins(plugins::collision_detection::CollisionDetectionPlugin)
        .add_plugins(plugins::despawn::DespawnPlugin)
        .add_plugins(plugins::game_state::GameStatePlugin)
        // .add_plugins(plugins::debug::DebugPlugin)
        .run();
}
