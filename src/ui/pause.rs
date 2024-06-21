use bevy::prelude::*;

use crate::despawn::remove_with_component;
use crate::state::GameState;

pub struct PauseUiPlugin;

impl Plugin for PauseUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), spawn_pause_ui)
            .add_systems(OnExit(GameState::Paused), remove_with_component::<PauseUi>);
    }
}

#[derive(Component)]
struct PauseUi;

fn spawn_pause_ui(mut commands: Commands) {
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
            PauseUi,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text::from_section(
                    "PAUSED",
                    TextStyle {
                        font_size: 32.0,
                        ..default()
                    },
                ),
                ..default()
            },));

            parent.spawn((TextBundle {
                text: Text::from_section(
                    "[Press Space to Resume]",
                    TextStyle {
                        font_size: 32.0,
                        ..default()
                    },
                ),
                ..default()
            },));

            parent.spawn((TextBundle {
                text: Text::from_section(
                    "[Press Q to Quit]",
                    TextStyle {
                        font_size: 28.0,
                        ..default()
                    },
                ),
                ..default()
            },));
        });
}
