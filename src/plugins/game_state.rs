use bevy::prelude::*;
use std::collections::HashMap;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Idle,
    Pause,
    InGame,
    GameOver,
    GameVictory,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PluginName {
    Tank,
    Enemy,
}

#[derive(Event)]
pub struct PluginReadyEvent {
    pub plugin_name: PluginName,
}

#[derive(Resource)]
pub struct PluginStateChecklist {
    pub checklist: HashMap<PluginName, bool>,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::Idle)
            .insert_resource(PluginStateChecklist {
                checklist: HashMap::new(),
            })
            .add_systems(
                Update,
                (
                    handle_game_pause,
                    check_plugin_ready.run_if(in_state(GameState::Idle)),
                ),
            )
            .add_event::<PluginReadyEvent>();
    }
}

fn handle_game_pause(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        match state.get() {
            GameState::InGame => next_state.set(GameState::Pause),
            // GameState::Pause => next_state.set(GameState::InGame),
            _ => (),
        }
    }
    if keyboard_input.pressed(KeyCode::Space) {
        match state.get() {
            GameState::Pause => next_state.set(GameState::InGame),
            _ => (),
        }
    }
}

fn check_plugin_ready(
    mut events: EventReader<PluginReadyEvent>,
    mut state: ResMut<NextState<GameState>>,
    mut plugin_state: ResMut<PluginStateChecklist>,
) {
    for event in events.read() {
        if !plugin_state.checklist.contains_key(&event.plugin_name) {
            plugin_state
                .checklist
                .insert(event.plugin_name.clone(), true);
        }

        // check the checklist is done
        if !plugin_state.checklist.contains_key(&PluginName::Tank) {
            continue;
        };
        if !plugin_state.checklist.contains_key(&PluginName::Enemy) {
            continue;
        };
        state.set(GameState::InGame);
    }
}
