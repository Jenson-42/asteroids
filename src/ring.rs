use std::f32::consts::PI;

use crate::{asset_loader::SceneAssets, movement::WORLD_SIZE};
use bevy::prelude::*;

const SATELLITES: i32 = 15;

pub struct RingPlugin;

impl Plugin for RingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_satellites);
    }
}

fn spawn_satellites(mut commands: Commands, asset_server: Res<SceneAssets>) {
    for i in 0..SATELLITES {
        // Work out the 2D position of this satellite.
        let thing = 2.0 * PI / (SATELLITES as f32) * (i as f32);
        let point = (f32::cos(thing) * WORLD_SIZE, f32::sin(thing) * WORLD_SIZE);

        // Spawn a satellite at that position.
        commands.spawn((SceneBundle {
            scene: asset_server.satellite.clone(),
            transform: Transform::from_translation(Vec3::new(point.1, 0.0, point.0)),
            ..Default::default()
        },));
    }
}
