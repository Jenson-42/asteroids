use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::health::Health;

/// Handles collision events sent by the Rapier physics plugin.
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_collision_damage);
    }
}

/// When an entity with this component applies collides with an entity with health, the other entity's health will be reduced by this amount.
#[derive(Debug, Component)]
pub struct CollisionDamage {
    pub amount: f32,
}

impl CollisionDamage {
    pub fn new(amount: f32) -> Self {
        CollisionDamage { amount }
    }
}

impl From<f32> for CollisionDamage {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl AsRef<f32> for CollisionDamage {
    fn as_ref(&self) -> &f32 {
        &self.amount
    }
}

/// Check for and apply collision damage when relevant collisions happen.
fn apply_collision_damage(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut health_query: Query<&mut Health>,
    collision_damage_query: Query<&CollisionDamage>,
) {
    for event in collision_event_reader.read() {
        // We only care about collisions that have just started.
        let CollisionEvent::Started(entity1, entity2, _) = event else {
            continue;
        };

        // Because only one collision event is generated for each collision, we need to check both entities for damage.
        try_damage(
            *entity1,
            *entity2,
            &mut health_query,
            &collision_damage_query,
        );
        try_damage(
            *entity2,
            *entity1,
            &mut health_query,
            &collision_damage_query,
        );
    }
}

/// Apply the collision damage of one entity to the health of another if they both have the correct components to do so.
fn try_damage(
    damager: Entity,
    damaged: Entity,
    health_query: &mut Query<&mut Health>,
    collision_damage_query: &Query<&CollisionDamage>,
) {
    // Return early if the damager doesn't have a CollisionDamage component.
    let Ok(collision_damage) = collision_damage_query.get(damager) else {
        return;
    };

    // Return early if the damaged doesn't have a Health component.
    let Ok(mut health) = health_query.get_mut(damaged) else {
        return;
    };

    health.value -= collision_damage.amount;
}
