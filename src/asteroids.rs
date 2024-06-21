use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::{rngs::ThreadRng, Rng};
use std::ops::Range;

use crate::{
    asset_loader::SceneAssets,
    collision::CollisionDamage,
    despawn::DespawnOnDie,
    health::Health,
    movement::{
        Acceleration, AngularVelocity, ConfinedToPlayArea, MovingObjectBundle, Velocity, WORLD_SIZE,
    },
    schedule::InGameSet,
};

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 0.0;
const SPAWN_TIME_SECONDS: f32 = 2.5;
const ASTEROID_HEALTH_RANGE: Range<f32> = 5.0..20.0;

/// Function to scale the asteroid with its health.
fn scale_from_health(health: f32) -> f32 {
    (health / 15.0) + 1.0
}

/// Function to scale an asteroid's collision damage with its health.
fn collision_damage_from_health(health: f32) -> f32 {
    health
}

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(
            Update,
            (
                spawn_asteroid,
                asteroids_scale_with_health,
                confine_once_in_play_area,
            )
                .in_set(InGameSet::EntityUpdates),
        );
    }
}

fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    asset_server: Res<SceneAssets>,
) {
    // Check if we're ready to spawn a new asteroid yet.
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    // Spawn the asteroid somewhere out of the game area.
    let translation = loop {
        let position = random_2d_unit_vector(&mut rng) * 100.0;
        if position.distance(Vec3::ZERO) > (WORLD_SIZE * 1.5) {
            break position;
        }
    };

    // Have the asteroid moving towards the middle of the game area.
    let velocity = ((random_2d_unit_vector(&mut rng) * (WORLD_SIZE * 0.75)) - translation)
        .normalize_or_zero()
        * VELOCITY_SCALAR;

    let acceleration = random_2d_unit_vector(&mut rng) * ACCELERATION_SCALAR;
    let angular_velocity = random_2d_unit_vector(&mut rng);

    let health = rng.gen_range(ASTEROID_HEALTH_RANGE);

    commands.spawn(AsteroidBundle::new(
        asset_server.asteroid.clone(),
        translation,
        velocity,
        acceleration,
        angular_velocity,
        health,
    ));
}

#[derive(Bundle)]
pub struct AsteroidBundle {
    pub moving_object_bundle: MovingObjectBundle,
    pub asteroid: Asteroid,
    pub health: Health,
    pub collision_damage: CollisionDamage,
    pub active_events: ActiveEvents,
    pub despawn_on_die: DespawnOnDie,
}

impl AsteroidBundle {
    pub fn new(
        model: Handle<Scene>,
        translation: Vec3,
        velocity: impl Into<Velocity>,
        acceleration: impl Into<Acceleration>,
        angular_velocity: impl Into<AngularVelocity>,
        health: f32,
    ) -> AsteroidBundle {
        let transform = Transform::from_translation(translation)
            .with_scale(Vec3::splat(scale_from_health(health)));

        let moving_object_bundle = MovingObjectBundle::new(
            SceneBundle {
                scene: model,
                transform,
                ..Default::default()
            },
            Collider::ball(2.5),
            velocity,
            acceleration,
            angular_velocity,
        );

        AsteroidBundle {
            moving_object_bundle,
            asteroid: Asteroid,
            health: Health::new(health),
            collision_damage: CollisionDamage::new(health),
            active_events: ActiveEvents::COLLISION_EVENTS,
            despawn_on_die: DespawnOnDie,
        }
    }
}

/// The physical size of the asteroids and the amount of damage they do on collision scales by how much health they have.
fn asteroids_scale_with_health(
    mut query: Query<
        (&mut Transform, &mut CollisionDamage, &Health),
        (With<Asteroid>, Changed<Health>),
    >,
) {
    for (mut transform, mut collision_damage, health) in query.iter_mut() {
        collision_damage.amount = collision_damage_from_health(health.value);
        transform.scale = Vec3::splat(scale_from_health(health.value));
    }
}

/// Generate a random XZ vector with length 1.0.
fn random_2d_unit_vector(rng: &mut ThreadRng) -> Vec3 {
    Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0)).normalize_or_zero()
}

/// Once the asteroids enter the play area, they can't leave.
fn confine_once_in_play_area(
    query: Query<
        (Entity, &Transform),
        (
            With<Asteroid>,
            Changed<Transform>,
            Without<ConfinedToPlayArea>,
        ),
    >,
    mut commands: Commands,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.distance(Vec3::ZERO) <= WORLD_SIZE {
            commands.entity(entity).insert(ConfinedToPlayArea);
        }
    }
}
