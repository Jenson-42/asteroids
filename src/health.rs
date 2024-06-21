use bevy::prelude::*;

use crate::schedule::InGameSet;

/// Handles the health of entities.
pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DieEvent>()
            .add_systems(Update, send_die_event.in_set(InGameSet::EntityUpdates));
    }
}

/// Component representing this entity's health.  When it becomes zero or less, a [DieEvent] is fired.
#[derive(Component, Debug)]
pub struct Health {
    pub value: f32,
}

impl Health {
    pub fn new(value: f32) -> Self {
        Health { value }
    }
}

impl From<f32> for Health {
    fn from(item: f32) -> Health {
        Health::new(item)
    }
}

impl AsRef<f32> for Health {
    fn as_ref(&self) -> &f32 {
        &self.value
    }
}

/// Event that is sent when an entity's health drops to or below zero.
#[derive(Debug, Event)]
pub struct DieEvent {
    pub entity: Entity,
}

/// Send a [DieEvent] event when an entity's health drops to or below zero.
fn send_die_event(
    mut event_writer: EventWriter<DieEvent>,
    query: Query<(Entity, &Health), Changed<Health>>,
) {
    for (entity, health) in query.iter() {
        if health.value <= 0.0 {
            event_writer.send(DieEvent { entity });
        }
    }
}
