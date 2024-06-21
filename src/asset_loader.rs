use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missiles: Handle<Scene>,
    pub satellite: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        spaceship: asset_server.load("Spaceship.glb#Scene0"),
        asteroid: asset_server.load("Planet.glb#Scene0"),
        missiles: asset_server.load("Bush.glb#Scene0"),
        satellite: asset_server.load("Base Large.glb#Scene0"),
    }
}
