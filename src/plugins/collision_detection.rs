use super::asset_loader::ExplosionEffectFolder;
use crate::components::{
    animation::{AnimationIndices, AnimationTimer},
    collider::{Collider, Explosion},
    tank::{Enemy, Tank, TankShell},
};
use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use std::collections::HashMap;

use super::game_state::GameState;

#[derive(Event)]
pub struct CollisionEvent {
    pub entity: (Entity, Name),
    pub colliding_entities: Vec<(Entity, Name)>,
}

pub struct CollisionDetectionPlugin;
impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    detect_collision,
                    handle_collisions::<Tank>,
                    handle_collisions::<Enemy>,
                    handle_collisions::<TankShell>,
                )
                    .run_if(in_state(GameState::InGame)),
                animate_explosion_sprite,
            ),
        )
        .add_event::<CollisionEvent>();
    }
}

fn detect_collision(
    mut collision_event_writer: EventWriter<CollisionEvent>,
    query: Query<(Entity, &GlobalTransform, &Collider, &Name)>,
) {
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

    for (entity, colliding_entities) in colliding_entity_map.iter() {
        let Ok((_, _, _, name)) = query.get(entity.clone()) else {
            continue;
        };
        collision_event_writer.send(CollisionEvent {
            entity: (entity.clone(), name.clone()),
            colliding_entities: colliding_entities.clone(),
        });
    }
}

fn handle_collisions<T: Component>(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut commands: Commands,
    query: Query<&Transform, With<T>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    explosion_effect: Res<ExplosionEffectFolder>,
    mut textures: ResMut<Assets<Image>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for collision_event in collision_event_reader.read() {
        for colliding_entity in collision_event.colliding_entities.iter() {
            let Ok(transform) = query.get(collision_event.entity.0) else {
                continue;
            };

            // Don't despawn itself
            if collision_event.entity.0 == colliding_entity.0 {
                continue;
            }

            // Don't despawn the ally entity
            if collision_event.entity.1.as_str() == colliding_entity.1.as_str() {
                continue;
            }

            commands
                .entity(collision_event.entity.0)
                .despawn_recursive();

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
    let animation_indices = AnimationIndices { first: 0, last: 4 };
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
            scale: Vec3::new(
                super::DEFAULT_SCALE,
                super::DEFAULT_SCALE,
                super::DEFAULT_SCALE,
            ),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Explosion,
    ));
}
