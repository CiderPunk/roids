use std::f32::consts::PI;

use bevy::{math::VectorSpace, prelude::*};

use crate::{asset_loader::SceneAssets, bounds::BoundsWarp, collision::Collider, effect_sprite::EffectSpriteMessage, game_manager::*, health::Health, level::{SpawnMessage, SpawnType}, movement::{PhysicsObject, Rotation, Velocity}, scheduling::GameSchedule};
pub struct UfoPlugin;


#[derive(Component)]
pub struct Ufo;

impl Plugin for UfoPlugin{
  fn build(&self, app: &mut bevy::app::App) {
    app
      .add_systems(Update, spawn_ufos.in_set(GameSchedule::EntityUpdates))
      .add_systems(Update, check_ufo_heath.in_set(GameSchedule::PreDespawnEntities));
  }
}



fn check_ufo_heath(
  query: Query<(&Health, &GlobalTransform, &Velocity), With<Ufo>>,
  mut effect_writer: MessageWriter<EffectSpriteMessage>,
){

  for (health, transform, velocity) in query.iter() {
    if health.value > 0. {
      continue;
    }

    effect_writer.write(
      EffectSpriteMessage::new(
        transform.translation(), 14.0, velocity.0, crate::effect_sprite::EffectSpriteType::Splosion));
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
    Ufo,
    BoundsWarp(false),
    Transform::from_translation(spawn.position).with_scale(Vec3::splat(4.)).with_rotation(Quat::from_rotation_x(0.25*PI)),
    Velocity(spawn.velocity),
    SceneRoot(scene_assets.ufo.clone()),
    Rotation(Vec3::new(0.,5.,0.1)),

    Collider {
      radius: 3.5,
      damage: 20.0,
    },
     
    Health {
      value: 20.,
      max: 20.,
      last_hurt_by: None,
    },

    //PhysicsObject::new(10.0),
  ));

}