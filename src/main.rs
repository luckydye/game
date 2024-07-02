mod level;
// mod lua;
mod systems;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_rapier3d::prelude::*;
// use bevy_registry_export::*;
use level::*;
use systems::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
  #[default]
  AssetLoading,
  SetupLevelAssets,
  Runntime,
}

fn main() {
  App::new()
    // .register_type::<lua::Script>()
    .register_type::<StaticCollider>()
    .register_type::<Base>()
    .register_type::<Block>()
    .register_type::<Player>()
    .add_plugins((
      DefaultPlugins,
      RapierPhysicsPlugin::<NoUserData>::default(),
      RapierDebugRenderPlugin::default(),
      // ExportRegistryPlugin::default(),
      ComponentsFromGltfPlugin::default(),
    ))
    .insert_state::<MyStates>(MyStates::default())
    .add_loading_state(
      LoadingState::new(MyStates::AssetLoading)
        .continue_to_state(MyStates::SetupLevelAssets)
        .load_collection::<LevelAssets>(),
    )
    .add_loading_state(
      LoadingState::new(MyStates::SetupLevelAssets).continue_to_state(MyStates::Runntime),
    )
    .add_systems(OnEnter(MyStates::SetupLevelAssets), start_level)
    .add_systems(OnEnter(MyStates::Runntime), init_player)
    .add_systems(OnEnter(MyStates::Runntime), init_base)
    .add_systems(OnEnter(MyStates::Runntime), init_static_colliders)
    .add_systems(OnEnter(MyStates::Runntime), init_blocks)
    // .add_systems(Update, lua::init_lua)
    .add_systems(Update, keyboard_input)
    .add_systems(Update, display_events)
    .add_systems(Update, cleanup)
    .run();
}
