use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, bounds::{BoundsDespawn, BoundsWarp, InBounds}, bullet::ShootMessage, collision::{Collider, CollisionFlags}, effect_sprite::EffectSpriteMessage, game_manager::*, health::Health, level::{SpawnMessage, SpawnType}, movement::{PhysicsObject, Rotation, Velocity}, player::PlayerShip, scheduling::GameSchedule, warning::Warn, wreckage::SpawnWreckageMessage};
pub struct UfoPlugin;

use bevy_prng::WyRand;
use bevy_rand::global::GlobalRng;
use rand::Rng;

const UFO_BULLET_TIME_TO_LIVE: f32 =1.5;
const UFO_BULLET_SPEED:f32 = 60.;
const UFO_FIRE_DELAY:f32 = 0.6;
const UFO_AIMED_FIRE_DELAY:f32 = 1.2;
  

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
  scene_assets: Res<SceneAssets>,
  mut rng: Single<&mut WyRand, With<GlobalRng>>,
  mut effect_writer: MessageWriter<EffectSpriteMessage>,
  mut msg_wreckage_writer: MessageWriter<SpawnWreckageMessage>,
){

  for (health, transform, velocity) in query.iter() {
    if health.value > 0. {
      continue;
    }
    info!("UFO destroyed, spawning wreckage");
    for i in 0 .. 3{
      msg_wreckage_writer.write(SpawnWreckageMessage::new(
        crate::wreckage::WreckageType::UfoPartRim,
        Transform::from_translation(transform.translation()).with_scale(Vec3::splat(4.)).with_rotation(transform.rotation() * Quat::from_rotation_y(i as f32 * PI * 0.66)),
        Vec3::new(0. + rng.random_range(-0.5 .. 0.5), 2. + rng.random_range(-0.5 .. 0.5), 0.1 + rng.random_range(-0.5 .. 0.5)),
        velocity.0 + Vec3::new(rng.random_range(-7. .. 7.), rng.random_range(-2. .. 2.), rng.random_range(-7. .. 7.)),
        rng.random_range(0.7 .. 2.0),
      ));
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
  info!("Spawning UFO at {:}", spawn.position);
  commands.spawn((
    LevelTarget,
    GameEntity,
    LevelEntity,
    Ufo{ 
      shoot_timer: Timer::from_seconds(UFO_FIRE_DELAY, TimerMode::Repeating),
      target_entity: None
    },
    BoundsWarp{
      warp_vertically: true,
      warp_horizontally: false,
    },
    BoundsDespawn,
    Transform::from_translation(spawn.position).with_scale(Vec3::splat(4.)).with_rotation(Quat::from_rotation_x(0.25*PI)),
    Velocity(spawn.velocity),
    SceneRoot(scene_assets.ufo.clone()),
    Rotation(Vec3::new(0.,2.,0.1)),

    Collider {
      collison_group: CollisionFlags::Enemy,
      collision_mask: CollisionFlags::Player | CollisionFlags::Asteroid,
      owner: None,
      radius: 3.5,
      damage: -20.0,
    },

    Health {
      value: 5.,
      max: 5.,
      last_hurt_by: None,
    },
    Warn(None),
    //PhysicsObject::new(10.0),
  ));
}

fn update_ufos(
  time: Res<Time>,
  mut query: Query<(Entity, &mut Ufo, &GlobalTransform, &Velocity), With<InBounds>>,
  mut shoot_writer: MessageWriter<ShootMessage>,
  mut rng: Single<&mut WyRand, With<GlobalRng>>,
  player_query:Query<(&GlobalTransform, &Velocity), With<PlayerShip>>,
){
  for (entity, mut ufo, transform, velocity) in query.iter_mut(){
    ufo.shoot_timer.tick(time.delta());
    if ufo.shoot_timer.just_finished(){
      // shoot at player
      //info!("UFO at {:?} shooting!", transform.translation());


      let direction;
      let mut closest_diff:Vec3 = Vec3::ZERO;
      let mut closest_velocity:Vec3 = Vec3::ZERO;
      let mut shortest_distance = std::f32::MAX;
      let mut found = false;

      for (player_transform, player_velocity) in player_query.iter(){
        let diff =  player_transform.translation() - transform.translation();
        if diff.length_squared() < 4000.0{
          found = true;
          let length_squared = diff.length_squared();
          if length_squared < shortest_distance{
            shortest_distance = length_squared;
            closest_diff = diff;
            closest_velocity = player_velocity.0;
          }
        }
      }
      if found{
        let time_to_hit = closest_diff.length() / UFO_BULLET_SPEED;
        let future_diff = closest_diff + (closest_velocity * time_to_hit) - (velocity.0 * time_to_hit);
        direction = future_diff.x.atan2(future_diff.z);
        //info!("UFO in range diff: {:?} dir:{:?}", diff, direction);          
        ufo.shoot_timer.set_duration(Duration::from_secs_f32( UFO_AIMED_FIRE_DELAY));        
      }
      else{
        ufo.shoot_timer.set_duration(Duration::from_secs_f32( UFO_FIRE_DELAY));        
        direction = rng.random_range(-PI .. PI)
      }

      

    let shoot_velocity = Vec3::new(direction.sin() * UFO_BULLET_SPEED, 0., direction.cos()*UFO_BULLET_SPEED) + velocity.0;

    shoot_writer.write(ShootMessage::new(
        false, 
        transform.translation(), 
        shoot_velocity, 
        -10.0, 
        1.0, 
        entity, 
        UFO_BULLET_TIME_TO_LIVE
      ));
    }
  }
}