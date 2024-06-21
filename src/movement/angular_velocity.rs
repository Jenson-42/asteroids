use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct AngularVelocity {
    pub value: Vec3,
}

impl AngularVelocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

impl From<Vec3> for AngularVelocity {
    fn from(item: Vec3) -> Self {
        Self::new(item)
    }
}

pub fn update_rotation(mut query: Query<(&AngularVelocity, &mut Transform)>, time: Res<Time>) {
    for (angular_velocity, mut transform) in query.iter_mut() {
        transform.rotate_local_x(angular_velocity.value.x * time.delta_seconds());
        transform.rotate_local_y(angular_velocity.value.y * time.delta_seconds());
        transform.rotate_local_z(angular_velocity.value.z * time.delta_seconds());
    }
}
