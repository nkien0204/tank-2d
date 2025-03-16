use bevy::prelude::*;

mod components;
mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(plugins::camera::CameraPlugin)
        .add_plugins(plugins::tank::TankPlugin)
        .add_plugins(plugins::opponent::OpponentPlugin)
        .add_plugins(plugins::movement::MovementPlugin)
        .add_plugins(plugins::asset_loader::AssetLoaderPlugin)
        // .add_plugins(plugins::debug::DebugPlugin)
        .run();
}
