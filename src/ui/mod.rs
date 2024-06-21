mod game;
mod game_over;
mod pause;
mod start;

use bevy::prelude::*;

use game::GameUiPlugin;
use game_over::GameOverUiPlugin;
use pause::PauseUiPlugin;
use start::StartUiPlugin;

/// Adds the UI into the game.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PauseUiPlugin)
            .add_plugins(GameUiPlugin)
            .add_plugins(StartUiPlugin)
            .add_plugins(GameOverUiPlugin);
    }
}
