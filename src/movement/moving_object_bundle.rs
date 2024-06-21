use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{Acceleration, AngularVelocity, Velocity};

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub angular_velocity: AngularVelocity,
    pub model: SceneBundle,
    pub sensor: Sensor,
    pub active_collision_types: ActiveCollisionTypes,
    pub collider: Collider,
}

impl MovingObjectBundle {
    pub fn new(
        model: SceneBundle,
        collider: Collider,
        velocity: impl Into<Velocity>,
        acceleration: impl Into<Acceleration>,
        angular_velocity: impl Into<AngularVelocity>,
    ) -> Self {
        let velocity = velocity.into();
        let acceleration = acceleration.into();
        let angular_velocity = angular_velocity.into();
        let sensor = Sensor;
        let active_collision_types = ActiveCollisionTypes::all();

        MovingObjectBundle {
            velocity,
            acceleration,
            angular_velocity,
            model,
            sensor,
            active_collision_types,
            collider,
        }
    }
}
