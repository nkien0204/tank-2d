use super::{BACKGROUND_Z_INDEX, DEFAULT_SCALE, asset_loader::ImageAssets};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_map);
    }
}

fn spawn_map(mut commands: Commands, image_assets: Res<ImageAssets>) {
    let texture_handle: Handle<Image> = image_assets.tile_grass_corner_ll.clone();
    let map_size = TilemapSize { x: 4, y: 4 };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
    let tile_size = TilemapTileSize { x: 128.0, y: 128.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, BACKGROUND_Z_INDEX),
            scale: Vec3::new(DEFAULT_SCALE, DEFAULT_SCALE, DEFAULT_SCALE),
            ..Default::default()
        },
        ..Default::default()
    });
}
