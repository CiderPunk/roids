use bevy::{math::VectorSpace, prelude::*};

use crate::{asset_loader::SceneAssets, bounds::BoundsWarp, collision::Collider, game_manager::*, health::Health, level::{SpawnMessage, SpawnType}, movement::{PhysicsObject, Rotation, Velocity}, scheduling::GameSchedule};
pub struct UfoPlugin;

impl Plugin for UfoPlugin{
  fn build(&self, app: &mut bevy::app::App) {
    app.add_systems(Update, spawn_ufos.in_set(GameSchedule::EntityUpdates))
    .add_systems(OnEnter(GameState::LevelInit), ufo_startup);

  //  app.add_systems(Update, systems)
  }
}



fn ufo_startup(
  mut commands: Commands,
  scene_assets: Res<SceneAssets>,
){
  info!("spawning test UFO");
  commands.spawn((
    Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(10.)),
    SceneRoot(scene_assets.ufo.clone()),
  ));

}

fn spawn_ufos(
  mut commands: Commands, 
  scene_assets: Res<SceneAssets>,
  mut spawn_reader:MessageReader<SpawnMessage>,
){
  for spawn in spawn_reader.read(){
    match spawn.spawn_type{
      SpawnType::Ufo => spawn_ufo(&spawn,&mut commands, scene_assets.clone()),
      _ => continue
    }
  }
}



fn spawn_ufo(
  spawn: &SpawnMessage,
  commands: &mut Commands, 
  scene_assets: SceneAssets,
){
  info!("Spawning UFO");
  commands.spawn((
    LevelTarget,
    GameEntity,
    LevelEntity,
    BoundsWarp(false),
    Transform::from_translation(spawn.position),
    Velocity(spawn.velocity),
    SceneRoot(scene_assets.ufo.clone()),
    Rotation(Vec3::new(0.5,0.,0.)),
    Collider {
      radius: 4.0,
      damage: 20.0,
    },
    Health {
      value: 10.,
      max: 10.,
      last_hurt_by: None,
    },
    PhysicsObject::new(10.0),
  ));

}