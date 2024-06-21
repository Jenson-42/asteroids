#![allow(clippy::type_complexity)]

mod asset_loader;
mod asteroids;
mod camera;
mod collision;
mod debug;
mod despawn;
mod health;
mod movement;
mod ring;
mod schedule;
mod scoreboard;
mod spaceship;
mod state;
mod ui;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidPlugin;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier3d::prelude::*;
use camera::CameraPlugin;
use collision::CollisionPlugin;
// use debug::DebugPlugin;
use despawn::DespawnPlugin;
use health::HealthPlugin;
use movement::MovementPlugin;
use ring::RingPlugin;
use schedule::SchedulePlugin;
use scoreboard::ScoreboardPlugin;
use spaceship::SpaceshipPlugin;
use state::GameStatePlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        // Bevy built-ins.
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Asteroids".to_string(),
                resolution: WindowResolution::new(800., 800.).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 500.0,
        })
        // Rapier Physics.
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        // Game plugins.
        .add_plugins(SchedulePlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(HealthPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(ScoreboardPlugin)
        .add_plugins(RingPlugin)
        // .add_plugins(DebugPlugin)
        .run();
}
