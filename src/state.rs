use bevy::prelude::*;

const PAUSE_BUTTON: KeyCode = KeyCode::Escape;
const CONTINUE_BUTTON: KeyCode = KeyCode::Space;
const QUIT_BUTTON: KeyCode = KeyCode::KeyQ;

#[derive(Debug, Default, States, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Start,
    InGame,
    Paused,
    GameOver,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, game_state_input_events);
    }
}

fn game_state_input_events(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>
) {
    // Pause/Unpause the game.
    if keyboard_input.just_pressed(PAUSE_BUTTON) {
        match state.get() {
            GameState::InGame => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::InGame),
            _ => (),
        }
    }

    // Continue from the start and game over screen.
    if keyboard_input.just_pressed(CONTINUE_BUTTON) {
        match state.get() {
            GameState::Start => next_state.set(GameState::InGame),
            GameState::GameOver => next_state.set(GameState::Start),
            _ => (),
        }
    }

    // Quit the current game or the program.
    if keyboard_input.just_pressed(QUIT_BUTTON) {
        match state.get() {
            GameState::Start => {app_exit_events.send(bevy::app::AppExit);},
            GameState::Paused => next_state.set(GameState::GameOver),
            _ => (),
        }
    }
}
