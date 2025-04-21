use bevy::asset::LoadedFolder;
use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct ImageAssets {
    pub tank: Handle<Image>,
    pub shell: Handle<Image>,
    pub enemy: Handle<Image>,
    pub map: Handle<Image>,
    pub barricade_metal: Handle<Image>,
    pub tree_brown_large: Handle<Image>,
    pub tree_green_large: Handle<Image>,
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ImageAssets>()
            .add_systems(Startup, (load_assets, load_textures));
    }
}

#[derive(Resource, Default)]
pub struct ExplosionEffectFolder(pub Handle<LoadedFolder>);

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let folder =
        ExplosionEffectFolder(asset_server.load_folder("tank_assets/PNG/Retina/explosion"));

    // load multiple, individual sprites from a folder
    commands.insert_resource(folder);
}

fn load_assets(mut image_assets: ResMut<ImageAssets>, asset_server: Res<AssetServer>) {
    image_assets.tank = asset_server.load("tank_assets/PNG/Retina/tank_blue.png");
    image_assets.shell = asset_server.load("tank_assets/PNG/Retina/bulletBlue1.png");
    image_assets.enemy = asset_server.load("tank_assets/PNG/Retina/tank_red.png");
    image_assets.map = asset_server.load("tank_assets/PNG/Retina/tank_2d_map.png");
    image_assets.barricade_metal = asset_server.load("tank_assets/PNG/Retina/barricadeMetal.png");
    image_assets.tree_brown_large = asset_server.load("tank_assets/PNG/Retina/treeBrown_large.png");
    image_assets.tree_green_large = asset_server.load("tank_assets/PNG/Retina/treeGreen_large.png");
}
