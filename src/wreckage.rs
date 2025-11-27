use bevy::{prelude::*, state::commands};
use bevy_prng::WyRand;
use bevy_rand::global::GlobalRng;

use crate::{asset_loader::SceneAssets, bounds::BoundsDespawn, effect_sprite::{EffectSpriteMessage, EffectSpriteType}, game_manager::GameEntity, movement::Velocity, scheduling::GameSchedule};

pub struct WreckagePlugin;
impl Plugin for WreckagePlugin {
  fn build(&self, app: &mut bevy::app::App) {
    app
      .add_message::<SpawnWreckageMessage>()
      .add_systems(Update, (
        spawn_wreckage.in_set(GameSchedule::PostEntityUpdates),
        remove_wreckage.in_set(GameSchedule::PreDespawnEntities),
      ));
  }
}


pub enum WreckageType{
  UfoPartRim,
  UfoPartHub,
}

#[derive(Message)]
pub struct SpawnWreckageMessage{
  wreck_type:WreckageType,
  transform:Transform,
  rotation:Vec3,
  velocity:Vec3,  
  time_to_live:f32,
}

impl SpawnWreckageMessage{
  pub fn new(wreck_type:WreckageType, transform:Transform, rotation:Vec3, velocity:Vec3, time_to_live:f32) -> Self{
    SpawnWreckageMessage{
      wreck_type,
      transform,
      rotation,
      velocity,
      time_to_live,
    }
  }
    
}

#[derive(Component)]
pub struct Wreckage{
  time_to_live:Timer,
  effect_scale:f32,
}

fn spawn_wreckage(
  mut commands: Commands,
  scene_assets: Res<SceneAssets>,
  mut msg_wreckage_reader: MessageReader<SpawnWreckageMessage>,
){
  for msg in msg_wreckage_reader.read(){
    let scene_handle = match msg.wreck_type{
      WreckageType::UfoPartRim => scene_assets.ufo_part_rim.clone(),
      WreckageType::UfoPartHub => scene_assets.ufo_part_hub.clone(),
    };
    commands.spawn((
      GameEntity,
      Wreckage{ 
        time_to_live: Timer::from_seconds(msg.time_to_live, TimerMode::Once),
        effect_scale: msg.transform.scale.x,
      },
      Velocity(msg.velocity),
      SceneRoot(scene_handle),
      msg.transform.clone(),
    ));
  }
}

fn remove_wreckage(
  mut commands: Commands,
  time: Res<Time>,    
  mut query:Query<(Entity, &GlobalTransform, &Velocity, &mut Wreckage)>,
  mut effect_writer: MessageWriter<EffectSpriteMessage>,
){
  for (entity, transform, velocity, mut wreckage) in query.iter_mut(){
    wreckage.time_to_live.tick(time.delta());
    if wreckage.time_to_live.just_finished(){
      commands.entity(entity).despawn();
      effect_writer.write(
        EffectSpriteMessage::new(transform.translation(), wreckage.effect_scale *4., velocity.0, EffectSpriteType::Splosion));
    }
  }


}