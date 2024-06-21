use crate::{state::GameState, despawn::remove_with_component};
use bevy::prelude::*;

pub struct StartUiPlugin;

impl Plugin for StartUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Start), spawn_start_ui)
            .add_systems(OnExit(GameState::Start), remove_with_component::<StartUi>);
    }
}

#[derive(Component)]
struct StartUi;

fn spawn_start_ui(mut commands: Commands) {
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
                background_color: Color::rgba(1.0, 1.0, 1.0, 0.1).into(),
                ..default()
            },
            StartUi,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text::from_section(
                    "ASTEROIDS",
                    TextStyle {
                        font_size: 32.0,
                        ..default()
                    },
                ),
                ..default()
            },));

            parent.spawn((TextBundle {
                text: Text::from_section(
                    "[Press Space to Start]",
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
