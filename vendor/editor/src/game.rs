use crate::insert_dependant_component;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// this file is just for demo purposes, contains various types of components, systems etc

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub enum SoundMaterial {
    Metal,
    Wood,
    Rock,
    Cloth,
    Squishy,
    #[default]
    None,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
/// Demo marker component
pub struct Player;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
/// Demo component showing auto injection of components
pub struct ShouldBeWithPlayer;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
/// Demo marker component
pub struct Interactible;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
/// Demo marker component
pub struct Pickable;

fn player_move_demo(
    keycode: Res<Input<KeyCode>>,
    mut players: Query<&mut Transform, With<Player>>,
) {
    let speed = 0.2;
    if let Ok(mut player) = players.get_single_mut() {
        if keycode.pressed(KeyCode::Left) {
            player.translation.x += speed;
        }
        if keycode.pressed(KeyCode::Right) {
            player.translation.x -= speed;
        }

        if keycode.pressed(KeyCode::Up) {
            player.translation.z += speed;
        }
        if keycode.pressed(KeyCode::Down) {
            player.translation.z -= speed;
        }
    }
}

// collision tests/debug
pub fn test_collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        println!("collision");
        match collision_event {
            CollisionEvent::Started(_entity1, _entity2, _) => {
                println!("collision started")
            }
            CollisionEvent::Stopped(_entity1, _entity2, _) => {
                println!("collision ended")
            }
        }
    }

    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

pub struct DemoPlugin;
impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Interactible>()
            .register_type::<Pickable>()
            .register_type::<SoundMaterial>()
            .register_type::<Player>()
            // little helper utility, to automatically inject components that are dependant on an other component
            // ie, here an Entity with a Player component should also always have a ShouldBeWithPlayer component
            // you get a warning if you use this, as I consider this to be stop-gap solution (usually you should have either a bundle, or directly define all needed components)
            .add_systems(
                Update,
                (
                    insert_dependant_component::<Player, ShouldBeWithPlayer>,
                    player_move_demo, //.run_if(in_state(AppState::Running)),
                    test_collision_events,
                ),
            );
    }
}
