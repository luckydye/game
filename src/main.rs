use bevy::{gltf::Gltf, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_registry_export::*;
use piccolo::{Callback, CallbackReturn, Closure, Executor, FunctionPrototype, Lua, Value};
use std::{
  fs::File,
  sync::{Arc, Mutex},
};

#[derive(AssetCollection, Resource)]
struct LevelAssets {
  #[asset(path = "Scene.glb")]
  level: Handle<Gltf>,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
struct Script {
  file: String,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
  #[default]
  AssetLoading,
  Next,
}

fn main() {
  App::new()
    .register_type::<Script>()
    .add_plugins((
      DefaultPlugins,
      ExportRegistryPlugin::default(),
      ComponentsFromGltfPlugin::default(),
    ))
    .insert_state::<MyStates>(MyStates::default())
    .add_loading_state(
      LoadingState::new(MyStates::AssetLoading)
        .continue_to_state(MyStates::Next)
        .load_collection::<LevelAssets>(),
    )
    .add_systems(OnEnter(MyStates::Next), start_level)
    .add_systems(
      Update,
      |mut commands: Commands,
       mut query: Query<(Entity, &mut Transform, &Script)>,
       time: Res<Time>| {
        let t = time.delta().as_millis().clone();

        for (entity, mut transform, script) in &mut query {
          let mut lua = Lua::full();

          let file_name = script.file.as_str();
          let file = File::open(file_name);
          let _transform = Arc::new(Mutex::new(transform.clone()));

          if let Ok(file) = file {
            lua
              .try_enter(|ctx| {
                let tr = _transform.clone();

                ctx.set_global(
                  "delta",
                  Callback::from_fn(&ctx, move |_, _, mut stack| {
                    stack.push_back(Value::Number(t as f64));
                    Ok(CallbackReturn::Return)
                  }),
                )?;

                ctx.set_global(
                  "rotate",
                  Callback::from_fn(&ctx, move |_, _, stack| {
                    let n = stack.get(0).to_number();

                    if let Some(n) = n {
                      tr.lock().unwrap().rotate_y(n as f32 / 100.0);
                    }

                    Ok(CallbackReturn::Return)
                  }),
                )?;

                Ok(())
              })
              .ok();

            let executor = lua
              .try_enter(|ctx| {
                let proto = FunctionPrototype::compile(ctx, file_name, file)?;
                let closure = Closure::new(&ctx, proto, Some(ctx.globals()))?;

                let stash = ctx.stash(Executor::start(ctx, closure.into(), ()));
                Ok(stash)
              })
              .ok();

            if let Some(executor) = executor {
              lua.finish(&executor);
            }

            println!("{:?}", _transform.lock().unwrap().rotation);
            transform.rotate(_transform.lock().unwrap().rotation);
          }
        }
      },
    )
    .run();
}

fn start_level(
  mut commands: Commands,
  assets: Res<LevelAssets>,
  models: Res<Assets<bevy::gltf::Gltf>>,
) {
  let my_gltf = models.get(assets.level.clone()).unwrap();

  commands.spawn((
    SceneBundle {
      scene: my_gltf.scenes[0].clone(),
      ..default()
    },
    Name::new("Level1"),
  ));
}
