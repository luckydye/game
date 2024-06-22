mod lua;

use bevy::{gltf::Gltf, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_registry_export::*;
use hematita::{
    lua_lib::*,
    lua_table,
    vm::{
        value::{IntoNillable, Table, Value},
        VirtualMachine,
    },
};
use lua::compile;
use std::{fs::File, io::Read, sync::Arc};

fn main() {
    App::new()
        .register_type::<Coin>()
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
        .add_systems(Update, lua_scripts)
        .run();
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
struct Coin;

fn lua_scripts(mut query: Query<&mut Transform, With<Coin>>, time: Res<Time>) {
    for mut transform in &mut query {
        let file = File::open("scripts/entity.lua");

        if file.is_ok() {
            let mut code = String::new();
            file.unwrap().read_to_string(&mut code).unwrap();

            let globals = lua_table! {
                delta = Value::Integer(time.delta().as_millis() as i64),
                print = Value::NativeFunction(&print),
                // require = Value::NativeFunction(&require),
                type = Value::NativeFunction(&r#type),
                setmetatable = Value::NativeFunction(&setmetatable),
                getmetatable = Value::NativeFunction(&getmetatable),
                pcall = Value::NativeFunction(&pcall),
                error = Value::NativeFunction(&error),
            }
            .arc();

            let res = run_script(code.as_str(), globals);

            if let Ok(res) = res {
                // let n = res.table().unwrap().index(&Value::Integer(1)).option();

                // let f: f32 = n.string().unwrap().parse().unwrap();
                println!("{:?}", res.table());
                // transform.rotate_y(f);
            } else {
                println!("lua error: {:?}", res.err().unwrap());
            }
        }
    }
}

#[derive(AssetCollection, Resource)]
struct LevelAssets {
    #[asset(path = "level.glb")]
    level: Handle<Gltf>,
}

pub fn run_script<'n>(code: &str, arguments: Arc<Table<'n>>) -> anyhow::Result<Value<'n>> {
    let vm = VirtualMachine::new(arguments);

    let function1 = compile(code)?;
    let function2 = compile("return update(entity)")?;

    // define functions
    vm.execute(&function1, Table::default().arc()).ok();

    // execute functions
    if let Ok(res) = vm.execute(&function2, Table::default().arc()) {
        Ok(res.nillable().option().unwrap())
    } else {
        Err(anyhow::anyhow!("runtime error"))
    }
}

fn start_level(
    mut commands: Commands,
    assets: Res<LevelAssets>,
    models: Res<Assets<bevy::gltf::Gltf>>,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 4000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    let my_gltf = models.get(assets.level.clone()).unwrap();

    commands.spawn((
        SceneBundle {
            scene: my_gltf.scenes[0].clone(),
            ..default()
        },
        Name::new("Level1"),
    ));
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}
