use bevy::asset::LoadedFolder;
use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct ImageAssets {
    pub tank: Handle<Image>,
    pub shell: Handle<Image>,
    pub enemy: Handle<Image>,
    pub tile_grass: Handle<Image>,
    pub tile_sand: Handle<Image>,
    pub tile_grass_road_crossing: Handle<Image>,
    pub tile_grass_corner_ll: Handle<Image>,
    pub tile_grass_corner_lr: Handle<Image>,
    pub tile_grass_corner_ul: Handle<Image>,
    pub tile_grass_corner_ur: Handle<Image>,
    pub tile_grass_road_east: Handle<Image>,
    pub tile_grass_road_north: Handle<Image>,
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
    image_assets.tile_grass = asset_server.load("tank_assets/PNG/Retina/tileGrass1.png");
    image_assets.tile_sand = asset_server.load("tank_assets/PNG/Retina/tileSand1.png");
    image_assets.tile_grass_road_crossing =
        asset_server.load("tank_assets/PNG/Retina/tileGrass_roadCrossing.png");
    image_assets.tile_grass_road_east =
        asset_server.load("tank_assets/PNG/Retina/tileGrass_roadEast.png");
    image_assets.tile_grass_road_north =
        asset_server.load("tank_assets/PNG/Retina/tileGrass_roadNorth.png");
    image_assets.tile_grass_corner_ur =
        asset_server.load("tank_assets/PNG/Retina/tileGrass_roadCornerUR.png");
    image_assets.tile_grass_corner_ul =
        asset_server.load("tank_assets/PNG/Retina/tileGrass_roadCornerUL.png");
    image_assets.tile_grass_corner_ll =
        asset_server.load("tank_assets/PNG/Retina/tileGrass_roadCornerLL.png");
    image_assets.tile_grass_corner_lr =
        asset_server.load("tank_assets/PNG/Retina/tileGrass_roadCornerLR.png");
}
