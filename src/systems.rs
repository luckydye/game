use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::level::*;

pub fn init_player(mut commands: Commands, players: Query<Entity, With<Player>>) {
  info!("init players");

  for player in players.iter() {
    let mut ent = commands.entity(player);

    info!("{players:?}");

    ent
      .insert(RigidBody::Dynamic)
      .insert(Collider::ball(0.5))
      .insert(Restitution::coefficient(1.5));
  }
}

pub fn start_level(
  mut commands: Commands,
  assets: Res<LevelAssets>,
  models: Res<Assets<bevy::gltf::Gltf>>,
) {
  info!("start level");

  let my_gltf = models.get(assets.level.clone()).unwrap();

  commands.spawn((
    SceneBundle {
      scene: my_gltf.scenes[0].clone(),
      ..default()
    },
    Name::new("Level1"),
  ));

  commands
    .spawn(Collider::cuboid(100.0, 0.1, 100.0))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}
