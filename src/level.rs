use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
  #[asset(path = "Scene.glb")]
  pub level: Handle<Gltf>,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct StaticCollider {}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Base {}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Block {}

#[derive(Component)]
pub struct Killed {
  pub timer: Timer,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Player {}
