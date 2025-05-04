use bevy::prelude::*;

use super::map::MAP_SIZE_PIXELS;

pub struct ManualPlugin;
impl Plugin for ManualPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_text);
    }
}

fn spawn_text(mut commands: Commands) {
    commands.spawn((
        Text2d::new("Move: W A S D"),
        TextColor::WHITE,
        TextFont {
            font_size: 20.0,
            ..default()
        },
        Transform::from_translation(Vec3::new(
            (-MAP_SIZE_PIXELS.0 + 170.0) / 2.0,
            (MAP_SIZE_PIXELS.1 - 40.0) / 2.0,
            10.0,
        )),
    ));

    commands.spawn((
        Text2d::new("Fire: J"),
        TextColor::WHITE,
        TextFont {
            font_size: 20.0,
            ..default()
        },
        Transform::from_translation(Vec3::new(
            (-MAP_SIZE_PIXELS.0 + 95.0) / 2.0,
            (MAP_SIZE_PIXELS.1 - 100.0) / 2.0,
            10.0,
        )),
    ));

    commands.spawn((
        Text2d::new("Pause: ESC, Continue: SPACE"),
        TextColor::WHITE,
        TextFont {
            font_size: 20.0,
            ..default()
        },
        Transform::from_translation(Vec3::new(
            (-MAP_SIZE_PIXELS.0 + 333.0) / 2.0,
            (MAP_SIZE_PIXELS.1 - 160.0) / 2.0,
            10.0,
        )),
    ));
}
