use crate::despawn::remove_with_component;
use bevy::prelude::*;

use crate::{health::Health, scoreboard::Scoreboard, spaceship::Spaceship, state::GameState};
pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_game_ui)
            .add_systems(OnExit(GameState::InGame), remove_with_component::<GameUi>)
            .add_systems(Update, (update_health_ui, update_score));
    }
}

#[derive(Component)]
struct GameUi;

#[derive(Component)]
struct HealthDisplay;

#[derive(Component)]
struct ScoreDisplay;

fn spawn_game_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            },
            GameUi,
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Health!",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                HealthDisplay,
            ));

            commands.spawn(
                NodeBundle {
                    style: Style {
                        width: Val::Px(25.0),
                        ..default()
                    },
                    ..default()
                }
            );

            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Score!",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                ScoreDisplay,
            ));
        });
}

fn update_health_ui(
    mut texts: Query<&mut Text, With<HealthDisplay>>,
    player_health: Query<&Health, With<Spaceship>>,
) {
    let Ok(health) = player_health.get_single() else {
        return;
    };

    for mut text in &mut texts {
        text.sections[0].value = format!("Health: {:.1}%", (health.value/crate::spaceship::SPACESHIP_HEALTH)*100.0);
    }
}

fn update_score(mut texts: Query<&mut Text, With<ScoreDisplay>>, score: Res<Scoreboard>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Score: {:.1}", score.score);
    }
}
