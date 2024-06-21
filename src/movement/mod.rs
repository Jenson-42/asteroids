mod acceleration;
mod angular_velocity;
mod moving_object_bundle;
mod velocity;

pub use acceleration::Acceleration;
pub use angular_velocity::AngularVelocity;
pub use moving_object_bundle::MovingObjectBundle;
pub use velocity::Velocity;

use crate::schedule::InGameSet;
use bevy::prelude::*;

/// The size of the game area in units.
pub const WORLD_SIZE: f32 = 50.0;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                acceleration::update_velocity,
                velocity::update_position,
                confine_to_play_area,
                angular_velocity::update_rotation,
            )
                .chain()
                .in_set(InGameSet::EntityUpdates),
        );
    }
}

#[derive(Component)]
pub struct ConfinedToPlayArea;

fn confine_to_play_area(mut query: Query<&mut Transform, With<ConfinedToPlayArea>>) {
    for mut transform in query.iter_mut() {
        // Send the entity to the other side.
        if transform.translation.distance(Vec3::ZERO) >= WORLD_SIZE {
            transform.translation = -transform.translation.normalize_or_zero() * WORLD_SIZE - 0.01;
        }
    }
}
