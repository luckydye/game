use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::level::*;

pub fn init_player(mut commands: Commands, mut positions: Query<Entity, With<Player>>) {
  // commands.en
}

pub fn start_level(
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

pub fn print_ball_altitude(mut positions: Query<&mut Transform, With<RigidBody>>) {
  for mut transform in positions.iter_mut() {
    dbg!(transform.rotation.to_axis_angle());
    transform.rotation = Quat::from_rotation_z(270_f32.to_radians());
  }
}

pub fn init_physics(mut commands: Commands) {
  /* Create the ground. */
  commands
    .spawn(Collider::cuboid(100.0, 0.1, 100.0))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 2.0, 0.0)));

  // /* Create the bouncing ball. */
  commands
    .spawn(RigidBody::Dynamic)
    .insert(Collider::ball(0.5))
    .insert(Restitution::coefficient(2.0))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}
