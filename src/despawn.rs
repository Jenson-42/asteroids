use std::time::Duration;

use bevy::prelude::*;

use crate::{
    health::{DieEvent, Health},
    schedule::InGameSet,
    state::GameState,
};

/// Handles the removal of game entities from the world.
pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, despawn_on_die.in_set(InGameSet::DespawnEntities))
            .add_systems(Update, update_despawn_timer)
            .add_systems(
                OnEnter(GameState::GameOver),
                remove_with_component::<Health>,
            );
    }
}

/// With this component added, entities will be recursively despawned when their health component is zero or less.
#[derive(Debug, Component)]
pub struct DespawnOnDie;

/// Despawn an entity if it has the [DespawnOnDie] component and a [DieEvent] is sent about it.
fn despawn_on_die(
    despawn_on_die_query: Query<&DespawnOnDie>,
    mut die_events: EventReader<DieEvent>,
    mut commands: Commands,
) {
    die_events
        .read()
        .filter(|DieEvent { entity }| despawn_on_die_query.get(*entity).is_ok())
        .for_each(|DieEvent { entity }| commands.entity(*entity).despawn_recursive());
}

/// Recursively despawn an entity if it has a component of type T attached to it.
pub fn remove_with_component<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// With this component added, entities will die after a given time.
#[derive(Debug, Component)]
pub struct DespawnTimer {
    timer: Timer,
}

impl DespawnTimer {
    /// Create a new despawn timer.  The entity will despawn after the set duration.
    pub fn new(duration: Duration) -> Self { 
        Self { timer: Timer::new(duration, TimerMode::Once) }
    }
}

/// Tick the despawn timer and kill entities which have finished timers.
fn update_despawn_timer(
    mut query: Query<(Entity, &mut DespawnTimer)>,
    mut die_events: EventWriter<DieEvent>,
    time: Res<Time>
) {
    for (entity, mut die_timer) in query.iter_mut() {
        die_timer.timer.tick(time.delta());

        if die_timer.timer.just_finished() {
            die_events.send(DieEvent { entity });
        }
    }
}
