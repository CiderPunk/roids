use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, bounds::BoundsWarp, bullet::ShootMessage, collision::Collider, effect_sprite::EffectSpriteMessage, game_manager::*, health::Health, level::{SpawnMessage, SpawnType}, movement::{PhysicsObject, Rotation, Velocity}, scheduling::GameSchedule};
pub struct UfoPlugin;

use bevy_prng::WyRand;
use bevy_rand::global::GlobalRng;
use rand::Rng;


#[derive(Component)]
pub struct Ufo{
  shoot_timer: Timer,
  target_entity: Option<Entity>,
}

impl Plugin for UfoPlugin{
  fn build(&self, app: &mut bevy::app::App) {
    app
      .add_systems(Update, (spawn_ufos, update_ufos).in_set(GameSchedule::EntityUpdates))
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
    Ufo{ 
      shoot_timer: Timer::from_seconds(1.6, TimerMode::Repeating),
      target_entity: None
    },
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

fn update_ufos(
  time: Res<Time>,
  mut query: Query<(Entity, &mut Ufo, &GlobalTransform, &Velocity)>,
  mut shoot_writer: MessageWriter<ShootMessage>,
  mut rng: Single<&mut WyRand, With<GlobalRng>>,
){
  for (entity, mut ufo, transform, velocity) in query.iter_mut(){
    ufo.shoot_timer.tick(time.delta());
    if ufo.shoot_timer.just_finished(){
      // shoot at player
      info!("UFO at {:?} shooting!", transform.translation());
      let direction = rng.random_range(-PI .. PI);
      let shoot_velocity = Vec3::new(direction.sin()*60., 0., direction.cos()*60.) + velocity.0;
      shoot_writer.write(ShootMessage::new(false, transform.translation(), shoot_velocity, 10.0, 1.0, entity));
    }
  }
}