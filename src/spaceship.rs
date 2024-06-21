use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use bevy::ecs::query::QuerySingleError;

use crate::{
    asset_loader::SceneAssets,
    collision::CollisionDamage,
    despawn::DespawnOnDie,
    health::{DieEvent, Health},
    movement::{Acceleration, AngularVelocity, ConfinedToPlayArea, MovingObjectBundle, Velocity},
    schedule::InGameSet,
    state::GameState,
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0.0, -20.);
pub const SPACESHIP_HEALTH: f32 = 150.0;
const SPACESHIP_SPEED: f32 = 30.0;
const SPACESHIP_ACCELERATION: f32 = 1.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 0.1;
const MISSILE_SPEED: f32 = 50.0;
const MISSILE_FORWARD_SCALAR: f32 = 7.5;
const WEAPON_FIRE_RATE: f32 = 5.0;

/// Marker component for the player's spaceship.
#[derive(Component, Debug)]
pub struct Spaceship;

/// Marker component for spaceship projectiles.
#[derive(Component, Debug)]
pub struct SpaceshipMissile;

/// Marker component for the spaceship's shield.
#[derive(Component, Debug)]
pub struct SpaceshipShield;

/// Cooldown timer for the spaceship's weapon.
#[derive(Resource, Debug)]
pub struct SpaceshipWeaponTimer {
    timer: Timer,
}

/// Adds the player's spaceship with weapons.
pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpaceshipWeaponTimer {
            timer: Timer::from_seconds(1.0 / WEAPON_FIRE_RATE, TimerMode::Repeating),
        })
        .add_systems(
            OnTransition {
                from: GameState::Start,
                to: GameState::InGame,
            },
            spawn_spaceship,
        )
        .add_systems(Update, game_over_when_spaceship_byebye)
        .add_systems(
            Update,
            (
                spaceship_movement_controls,
                spaceship_weapon_controls,
                spaceship_shield_controls,
            )
                .chain()
                .in_set(InGameSet::UserInput),
        );
    }
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<SceneAssets>) {
    commands
        .spawn((
            MovingObjectBundle::new(
                SceneBundle {
                    scene: asset_server.spaceship.clone(),
                    transform: Transform::from_translation(STARTING_TRANSLATION),
                    ..Default::default()
                },
                Collider::cuboid(4.0, 1.0, 5.0),
                Velocity::new(Vec3::ZERO),
                Acceleration::new(Vec3::ZERO),
                AngularVelocity::new(Vec3::ZERO),
            ),
            Spaceship,
            CollisionDamage::new(20.0),
            Health::new(SPACESHIP_HEALTH),
            ActiveEvents::COLLISION_EVENTS,
            DespawnOnDie,
            ConfinedToPlayArea,
        ))
        .with_children(|builder| {
            builder.spawn(PointLightBundle {
                transform: Transform::from_xyz(0.0, 2.0, 2.0).with_scale(Vec3::new(5.0, 5.0, 5.0)),
                point_light: PointLight {
                    intensity: 100_000.0,
                    color: Color::WHITE,
                    shadows_enabled: true,
                    ..default()
                },
                ..default()
            });
        });
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // Return early if the spaceship doesn't exist.
    let (mut transform, mut velocity) = match query.get_single_mut() {
        Ok(spaceship) => spaceship,
        Err(not_single_error) => {
            let reason = match not_single_error {
                QuerySingleError::NoEntities(_) => "there's no spaceship!",
                QuerySingleError::MultipleEntities(_) => "there's more than one spaceship!",
            };

            warn!("Can't do spaceship movement, {reason}");
            return;
        }
    };

    let left_input = [KeyCode::KeyA, KeyCode::ArrowLeft];
    let right_input = [KeyCode::KeyD, KeyCode::ArrowRight];
    let forward_input = [KeyCode::KeyW, KeyCode::ArrowUp];

    // Handle rotation of the spaceship.
    let mut rotation = 0.0;
    if keyboard_input.any_pressed(right_input) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.any_pressed(left_input) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }
    transform.rotate_y(rotation);

    // Handle roll of the spaceship.
    let mut roll = 0.0;
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        roll = -SPACESHIP_ROLL_SPEED;
    } else if keyboard_input.pressed(KeyCode::ControlLeft) {
        roll = SPACESHIP_ROLL_SPEED;
    }
    transform.rotate_local_z(roll);

    // The spaceship slows down over time.
    velocity.value = velocity.value - velocity.value * (0.5 * time.delta_seconds());

    if keyboard_input.any_pressed(forward_input) {
        // If the speed in the direction we are facing is less than SPACESHIP_SPEED, accelerate in that direction.
        if -transform.forward().dot(velocity.value) <= SPACESHIP_SPEED {
            velocity.value += -transform.forward() * SPACESHIP_ACCELERATION;
        }
    }
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<SceneAssets>,
    time: Res<Time>,
    mut timer: ResMut<SpaceshipWeaponTimer>,
) {
    let Ok(transform) = query.get_single() else {
        return;
    };

    // Press space to shoot. Pew pew.
    if !keyboard_input.pressed(KeyCode::Space) {
        return;
    }

    // Shooting cooldown.
    timer.timer.tick(time.delta());
    if !timer.timer.just_finished() {
        return;
    }

    commands.spawn((
        MovingObjectBundle::new(
            SceneBundle {
                scene: asset_server.missiles.clone(),
                transform: Transform::from_translation(
                    transform.translation + -transform.forward() * MISSILE_FORWARD_SCALAR,
                )
                .with_rotation(transform.rotation),
                ..Default::default()
            },
            Collider::ball(0.5),
            Velocity::new(-transform.forward() * MISSILE_SPEED),
            Acceleration::new(Vec3::ZERO),
            AngularVelocity::new(Vec3::ZERO),
        ),
        SpaceshipMissile,
        CollisionDamage::new(5.0),
        Health::new(2.5),
        DespawnOnDie,
        ConfinedToPlayArea,
    ));
}

fn spaceship_shield_controls(
    mut commands: Commands,
    query: Query<Entity, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(spaceship) = query.get_single() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::Tab) {
        commands.entity(spaceship).insert(SpaceshipShield);
    }
}

fn game_over_when_spaceship_byebye(
    query: Query<Entity, With<Spaceship>>,
    mut event_reader: EventReader<DieEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Check each die event to see if it's the spaceship, then set gameover if it is.
    for DieEvent { entity } in event_reader.read() {
        if query.get(*entity).is_ok() {
            next_state.set(GameState::GameOver)
        }
    }
}
