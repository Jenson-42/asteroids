use bevy::prelude::*;

use crate::{asteroids::Asteroid, health::DieEvent, schedule::InGameSet, state::GameState};

/// Keeps the player's score.
pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scoreboard::new())
            .add_systems(
                Update,
                destroy_asteroids_for_points.in_set(InGameSet::EntityUpdates),
            )
            .add_systems(OnEnter(GameState::InGame), reset_scoreboard);
    }
}

/// A resource for keeping track of the player's score.
#[derive(Debug, Resource)]
pub struct Scoreboard {
    pub score: f32,
}

impl Scoreboard {
    fn new() -> Self {
        Self { score: 0.0 }
    }
}

/// The player's score goes up when an asteroid is destroyed.
fn destroy_asteroids_for_points(
    mut die_events: EventReader<DieEvent>,
    mut scoreboard: ResMut<Scoreboard>,
    asteroids_query: Query<Entity, With<Asteroid>>,
) {
    let asteroids_killed = die_events
        .read()
        .filter(|DieEvent { entity }| asteroids_query.get(*entity).is_ok())
        .count();

    scoreboard.score += asteroids_killed as f32;
}

/// Reset the scoreboard back to zero.
fn reset_scoreboard(mut scoreboard: ResMut<Scoreboard>) {
    scoreboard.score = 0.0;
}
