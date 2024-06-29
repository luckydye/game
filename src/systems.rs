use std::time::Duration;

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
      .insert(Damping {
        linear_damping: 0.2,
        angular_damping: 0.0,
      })
      .insert(Velocity {
        linvel: Vec3::new(0.0, -20.0, 0.0),
        angvel: Vec3::new(0.0, 0.0, 0.0),
      })
      .insert(ColliderMassProperties::Density(8.0));
  }
}

pub fn init_base(mut commands: Commands, bases: Query<Entity, With<Base>>) {
  info!("init_base");

  for base in bases.iter() {
    let mut ent = commands.entity(base);

    // println!("{mesh:?}");

    ent
      .insert(RigidBody::KinematicVelocityBased)
      .insert(Velocity {
        linvel: Vec3::new(0.0, 0.0, 0.0),
        angvel: Vec3::new(0.0, 0.0, 0.0),
      })
      .insert(Restitution::coefficient(3.0))
      .insert(Collider::cuboid(1.0, 1.0, 1.0));
  }
}

pub fn init_blocks(mut commands: Commands, blocks: Query<Entity, With<Block>>) {
  info!("init_blocks");

  println!("{blocks:?}");

  for block in blocks.iter() {
    commands
      .entity(block)
      .insert(RigidBody::Dynamic)
      .insert(GravityScale(0.0))
      .insert(ColliderMassProperties::Density(8.0))
      .insert(Restitution::coefficient(1.0))
      .insert(Collider::cuboid(1.0, 1.0, 1.0));
  }
}

pub fn init_static_colliders(
  mut commands: Commands,
  colliders: Query<Entity, With<StaticCollider>>,
) {
  info!("init_static_colliders");

  for collider in colliders.iter() {
    commands
      .entity(collider)
      .insert(RigidBody::Fixed)
      .insert(Collider::cuboid(1.0, 1.0, 1.0));
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

  // commands
  //   .spawn(Collider::cuboid(100.0, 0.1, 100.0))
  //   .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}

pub fn keyboard_input(
  mut basees: Query<&mut Velocity, With<Base>>,
  keys: Res<ButtonInput<KeyCode>>,
) {
  // println!("input {basees:?}");

  if keys.pressed(KeyCode::KeyA) {
    for mut base in basees.iter_mut() {
      base.linvel.x = -8.0;
    }
  }
  if keys.pressed(KeyCode::KeyD) {
    for mut base in basees.iter_mut() {
      base.linvel.x = 8.0;
    }
  }

  if keys.just_released(KeyCode::KeyA) {
    for mut base in basees.iter_mut() {
      base.linvel.x = 0.0;
    }
  }
  if keys.just_released(KeyCode::KeyD) {
    for mut base in basees.iter_mut() {
      base.linvel.x = 0.0;
    }
  }
}

pub fn display_events(
  mut commands: Commands,
  rapier_context: Res<RapierContext>,
  blocks: Query<Entity, With<Block>>,
  players: Query<Entity, With<Player>>,
) {
  for player in players.iter() {
    for block in blocks.iter() {
      let contact = rapier_context.contact_pair(block, player);
      if contact.is_some() {
        commands.entity(block).insert(Killed {
          timer: Timer::new(Duration::from_millis(250), TimerMode::Once),
        });
      }
    }
  }
}

pub fn cleanup(mut commands: Commands, mut killed: Query<(Entity, &mut Killed)>, time: Res<Time>) {
  for (entity, mut killed) in killed.iter_mut() {
    // timers gotta be ticked, to work
    killed.timer.tick(time.delta());

    // if it finished, despawn the bomb
    if killed.timer.finished() {
      commands.entity(entity).despawn_recursive();
    }
  }
}
