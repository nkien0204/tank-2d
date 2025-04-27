use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Pause,
    InGame,
    GameOver,
    GameVictory,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::InGame)
            .add_systems(Update, handle_game_pause);
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
