mod level;
mod lua;
mod systems;

use std::sync::Once;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_rapier3d::prelude::*;
use bevy_registry_export::*;
use level::*;
use lua::*;
use systems::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
  #[default]
  AssetLoading,
  SetupLevel,
  Next,
}

fn main() {
  App::new()
    .register_type::<Script>()
    .register_type::<Player>()
    .add_plugins((
      DefaultPlugins,
      RapierPhysicsPlugin::<NoUserData>::default(),
      RapierDebugRenderPlugin::default(),
      ExportRegistryPlugin::default(),
      ComponentsFromGltfPlugin::default(),
    ))
    .insert_state::<MyStates>(MyStates::default())
    .add_loading_state(
      LoadingState::new(MyStates::AssetLoading)
        .continue_to_state(MyStates::SetupLevel)
        .load_collection::<LevelAssets>(),
    )
    .add_loading_state(LoadingState::new(MyStates::SetupLevel).continue_to_state(MyStates::Next))
    .add_systems(OnEnter(MyStates::SetupLevel), start_level)
    .add_systems(OnEnter(MyStates::Next), init_player)
    .add_systems(Update, init_lua)
    .run();
}
