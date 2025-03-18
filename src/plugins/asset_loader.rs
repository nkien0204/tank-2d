use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct ImageAssets {
    pub tank: Handle<Image>,
    pub tank_gun: Handle<Image>,
    pub shell: Handle<Image>,
    pub opponent: Handle<Image>,
    pub opponent_gun: Handle<Image>,
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ImageAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut image_assets: ResMut<ImageAssets>, asset_server: Res<AssetServer>) {
    image_assets.tank = asset_server.load("tanks/PNG/Hulls_Color_A/Hull_01.png");
    image_assets.tank_gun = asset_server.load("tanks/PNG/Weapon_Color_A/Gun_01.png");
    image_assets.shell = asset_server.load("tanks/PNG/Effects/Heavy_Shell.png");
    image_assets.opponent = asset_server.load("tanks/PNG/Hulls_Color_D/Hull_01.png");
    image_assets.opponent_gun = asset_server.load("tanks/PNG/Weapon_Color_D/Gun_01.png");
}
