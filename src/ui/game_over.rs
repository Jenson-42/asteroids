use crate::{state::GameState, despawn::remove_with_component};
use bevy::prelude::*;

pub struct GameOverUiPlugin;

impl Plugin for GameOverUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn_gameover_ui)
            .add_systems(
                OnExit(GameState::GameOver),
                remove_with_component::<GameOverUi>,
            );
    }
}

#[derive(Component)]
struct GameOverUi;

fn spawn_gameover_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            GameOverUi,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text::from_section(
                    "Game Over!",
                    TextStyle {
                        font_size: 32.0,
                        ..default()
                    },
                ),
                ..default()
            },));

            parent.spawn((TextBundle {
                text: Text::from_section(
                    "[Press Space to Continue]",
                    TextStyle {
                        font_size: 32.0,
                        ..default()
                    },
                ),
                ..default()
            },));
        });
}
