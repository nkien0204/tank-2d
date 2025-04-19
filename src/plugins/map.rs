use super::{BACKGROUND_Z_INDEX, DEFAULT_SCALE, asset_loader::ImageAssets};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

const TILEMAP_SIZE: TilemapSize = TilemapSize { x: 20, y: 12 };
const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 128.0, y: 128.0 };

pub const MAP_SIZE_PIXELS: (f32, f32) = (
    TILEMAP_SIZE.x as f32 * TILE_SIZE.x * DEFAULT_SCALE,
    TILEMAP_SIZE.y as f32 * TILE_SIZE.y * DEFAULT_SCALE,
);

// left, right, top, bottom
const MAP_BOUNDS: (f32, f32, f32, f32) = (
    MAP_SIZE_PIXELS.0 / 2.0 * -1.0,
    MAP_SIZE_PIXELS.0 / 2.0,
    MAP_SIZE_PIXELS.1 / 2.0,
    MAP_SIZE_PIXELS.1 / 2.0 * -1.0,
);

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    BarricadeMetal,
}

pub enum OutOfBoundTypes {
    LeftVertical(f32),
    RightVertical(f32),
    LowHorizontal(f32),
    HighHorizontal(f32),
    None,
}

#[derive(Bundle, Debug)]
pub struct MapBundle {
    pub sprite: Sprite,
    pub transform: Transform,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_map);
    }
}

fn spawn_map(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn(MapBundle {
        sprite: Sprite {
            image: image_assets.map.clone(),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, BACKGROUND_Z_INDEX),
            scale: Vec3::new(DEFAULT_SCALE, DEFAULT_SCALE, 1.0),
            ..default()
        },
    });

    // let tilemap_entity = commands.spawn_empty().id();
    // let mut tile_storage = TileStorage::empty(TILEMAP_SIZE);

    // let image_handles = vec![image_assets.barricadeMetal.clone()];
    // let texture_vec = TilemapTexture::Vector(image_handles);

    // let tile_positions: Vec<TilePos> = vec![TilePos { x: 0, y: 0 }];
    // fill_texture_to_map(
    //     &mut commands,
    //     tilemap_entity,
    //     &mut tile_storage,
    //     tile_positions,
    //     TileType::BarricadeMetal,
    // );

    // let tile_size = TilemapTileSize { x: 128.0, y: 128.0 };
    // let grid_size = tile_size.into();
    // let map_type = TilemapType::default();

    // let center_x = (TILEMAP_SIZE.x as f32 * -1.0 / 2.0 + 0.5) * tile_size.x * DEFAULT_SCALE;
    // let center_y = (TILEMAP_SIZE.y as f32 * -1.0 / 2.0 + 0.5) * tile_size.y * DEFAULT_SCALE;

    // commands.entity(tilemap_entity).insert(TilemapBundle {
    //     grid_size,
    //     map_type,
    //     size: TILEMAP_SIZE,
    //     storage: tile_storage,
    //     texture: texture_vec,
    //     tile_size,

    //     transform: Transform {
    //         translation: Vec3::new(center_x, center_y, BACKGROUND_Z_INDEX),
    //         scale: Vec3::new(DEFAULT_SCALE, DEFAULT_SCALE, DEFAULT_SCALE),
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });
}

fn fill_texture_to_map(
    commands: &mut Commands,
    tilemap_entity: Entity,
    tile_storage: &mut TileStorage,
    tile_positions: Vec<TilePos>,
    texture_index: TileType,
) {
    for tile_pos in tile_positions {
        let tile_entity = commands
            .spawn(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: TileTextureIndex(texture_index.clone() as u32),
                ..Default::default()
            })
            .id();
        tile_storage.set(&tile_pos, tile_entity);
    }
}

// check whether the entity is out of the map
pub fn check_out_of_bounds(translation: Vec3, image_size: UVec2, padding: f32) -> OutOfBoundTypes {
    let entity_half_size_x = image_size.x as f32 / 2.0 * DEFAULT_SCALE;
    let entity_half_size_y = image_size.y as f32 / 2.0 * DEFAULT_SCALE;

    if translation.x - entity_half_size_x - padding < MAP_BOUNDS.0 {
        OutOfBoundTypes::LeftVertical(MAP_BOUNDS.0 + entity_half_size_x + padding)
    } else if translation.x + entity_half_size_x + padding > MAP_BOUNDS.1 {
        OutOfBoundTypes::RightVertical(MAP_BOUNDS.1 - entity_half_size_x - padding)
    } else if translation.y - entity_half_size_y - padding < MAP_BOUNDS.3 {
        OutOfBoundTypes::LowHorizontal(MAP_BOUNDS.3 + entity_half_size_y + padding)
    } else if translation.y + entity_half_size_y + padding > MAP_BOUNDS.2 {
        OutOfBoundTypes::HighHorizontal(MAP_BOUNDS.2 - entity_half_size_y - padding)
    } else {
        OutOfBoundTypes::None
    }
}
