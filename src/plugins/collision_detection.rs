use super::asset_loader::ExplosionEffectFolder;
use crate::components::{
    animation::{AnimationIndices, AnimationTimer},
    collider::{Collider, Explosion},
    tank::{Opponent, Tank, TankShell},
};
use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use std::collections::HashMap;

use super::game_state::GameState;

pub struct CollisionDetectionPlugin;
impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                detect_collision,
                handle_collisions::<Tank>,
                handle_collisions::<Opponent>,
                handle_collisions::<TankShell>,
                animate_explosion_sprite,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn detect_collision(mut query: Query<(Entity, &GlobalTransform, &mut Collider, &Name)>) {
    let mut colliding_entity_map: HashMap<Entity, Vec<(Entity, Name)>> = HashMap::new();

    for (entity_a, transform_a, collider_a, _) in query.iter() {
        for (entity_b, transform_b, collider_b, name_b) in query.iter() {
            if entity_a == entity_b {
                continue;
            }

            let distance = transform_a
                .translation()
                .distance(transform_b.translation());
            let collision_distance = collider_a.radius + collider_b.radius;

            if distance < collision_distance {
                colliding_entity_map
                    .entry(entity_a)
                    .or_insert_with(Vec::new)
                    .push((entity_b, name_b.clone()));
            }
        }
    }

    for (entity, _, mut collider, _name) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(colliding_entities) = colliding_entity_map.get(&entity) {
            collider
                .colliding_entities
                .extend(colliding_entities.clone());
        }
    }
}

fn handle_collisions<T: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Name, &Collider, &Transform), With<T>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    explosion_effect: Res<ExplosionEffectFolder>,
    mut textures: ResMut<Assets<Image>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (entity, name, collider, transform) in query.iter() {
        for colliding_entity in collider.colliding_entities.iter() {
            if query.get(colliding_entity.0).is_ok() {
                continue;
            }
            if name.as_str() == colliding_entity.1.as_str() {
                continue;
            }
            commands.entity(entity).despawn_recursive();

            create_explosion_effect(
                &mut commands,
                transform,
                &loaded_folders,
                &explosion_effect,
                &mut textures,
                &mut layouts,
            );
        }
    }
}

fn animate_explosion_sprite(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (Entity, &AnimationIndices, &mut AnimationTimer, &mut Sprite),
        With<Explosion>,
    >,
) {
    for (entity, indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    commands.entity(entity).despawn_recursive();
                    return;
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

fn create_explosion_effect(
    commands: &mut Commands,
    entity_transform: &Transform,
    loaded_folders: &Res<Assets<LoadedFolder>>,
    explosion_effect: &Res<ExplosionEffectFolder>,
    textures: &mut ResMut<Assets<Image>>,
    layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut builder = TextureAtlasBuilder::default();
    let folder = loaded_folders.get(&explosion_effect.0).unwrap();

    for handle in folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();

        let Some(texture) = textures.get(id) else {
            warn!("Texture not loaded: {:?}", handle.path().unwrap());
            continue;
        };

        builder.add_texture(Some(id), texture);
    }

    let (texture_atlas_layout, _texture_atlas_sources, texture) = builder.build().unwrap();
    let animated_texture = textures.add(texture);
    let layout = layouts.add(texture_atlas_layout);
    let animation_indices = AnimationIndices { first: 0, last: 7 };
    commands.spawn((
        Sprite::from_atlas_image(
            animated_texture,
            TextureAtlas {
                layout,
                index: animation_indices.first,
            },
        ),
        Transform {
            translation: entity_transform.translation,
            scale: super::DEFAULT_SCALE,
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Explosion,
    ));
}
