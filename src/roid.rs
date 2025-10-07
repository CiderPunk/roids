use std::{f32::consts::PI, time::Duration};

use crate::{
  asset_loader::SceneAssets, bounds::BoundsWarp, collision::Collider, effect_sprite::EffectSpriteMessage, game_manager::{GameEntity, GameState}, health::Health, level::{SpawnMessage, SpawnType}, movement::{Acceleration, PhysicsObject, Rotation, Velocity}, player::ScoreMessage, scheduling::GameSchedule
};
use bevy::prelude::*;
use rand::Rng;


#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum RoidSize {
  #[default]
  Large,
  Medium,
  Small,
}


const ROID_SPAWN_DISTANCE: f32 = 150.0;

const ROID_MAX_SPEED: f32 = 26.;

const ROID_LARGE_SCALE: Vec3 = Vec3::splat(5.);
const ROID_MEDIUM_SCALE: Vec3 = Vec3::splat(3.);
const ROID_SMALL_SCALE: Vec3 = Vec3::splat(1.);

const ROID_LARGE_RADIUS: f32 = 7.;
const ROID_MEDIUM_RADIUS: f32 = 4.;
const ROID_SMALL_RADIUS: f32 = 1.5;


const ROID_LARGE_MASS: f32 = 20.;
const ROID_MEDIUM_MASS: f32 = 10.;
const ROID_SMALL_MASS: f32 = 5.;

const ROID_COLLISION_DAMAGE: f32 = -100.;

pub struct RoidPlugin;

impl Plugin for RoidPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(WaveSpawnTimer{ timer: Timer::from_seconds(1., TimerMode::Repeating), count:1 })
      .add_systems(Update, spawn_roids.in_set(GameSchedule::EntityUpdates).run_if(in_state(GameState::Alive)))
      .add_systems(
        Update,
        check_asteroid_health.in_set(GameSchedule::PreDespawnEntities),
      );
  }
}


#[derive(Resource, Default)]
struct WaveSpawnTimer{
  timer:Timer,
  count:u32,
}

#[derive(Component, Default)]
pub struct Roid(RoidSize);

fn check_asteroid_health(
  mut commands: Commands,
  query: Query<(&Roid, &Health, &GlobalTransform, &Velocity, &BoundsWarp)>,
  mut effect_writer: MessageWriter<EffectSpriteMessage>,
  scene_assets: Res<SceneAssets>,
  mut score_writer:MessageWriter<ScoreMessage>,
) {
  let mut rng = rand::rng();
  for (roid, health, transform, velocity, orig_bounds_warp) in query.iter() {
    if health.value > 0. {
      continue;
    }

    score_writer.write(ScoreMessage::new(match roid.0 {
        RoidSize::Large => 50,
        RoidSize::Medium => 20,
        RoidSize::Small => 10,
    }));

    let effect_scale = match roid.0 {
      RoidSize::Large => 16.,
      RoidSize::Medium => 12.,
      RoidSize::Small => 8.,
    };
    effect_writer.write(EffectSpriteMessage::new(
      transform.translation(),
      effect_scale,
      velocity.0,
      crate::effect_sprite::EffectSpriteType::Splosion,
    ));

    if roid.0 == RoidSize::Small {
      continue;
    }

    let scale: Vec3;
    let collider_radius: f32;
    let next_size: RoidSize;
    let mass:f32;
    match roid.0 {
      RoidSize::Large => {
        scale = ROID_MEDIUM_SCALE;
        collider_radius = ROID_MEDIUM_RADIUS;
        next_size = RoidSize::Medium;
        mass = ROID_MEDIUM_MASS;
      }
      RoidSize::Medium => {
        scale = ROID_SMALL_SCALE;
        collider_radius = ROID_SMALL_RADIUS;
        next_size = RoidSize::Small;
        mass = ROID_SMALL_MASS;
      }
      RoidSize::Small => {
        scale = ROID_SMALL_SCALE;
        collider_radius = ROID_SMALL_RADIUS;
        next_size = RoidSize::Small;
        mass = ROID_SMALL_MASS;
      }
    }

    for _ in 0..2 {
      let rotation = Vec3::new(
        rng.random_range(-1. ..1.),
        rng.random_range(-1. ..1.),
        rng.random_range(-1. ..1.),
      );

    commands.spawn((
        GameEntity,
        SceneRoot(scene_assets.roid1.clone()),
        BoundsWarp(orig_bounds_warp.0),
        Transform::from_translation(transform.translation()).with_scale(scale),
        Velocity(
          velocity.0
            + Vec3::new(
              rng.random_range(-10. ..10.),
              0.,
              rng.random_range(-10. ..10.),
            ),
        ),
        Health {
          value: 10.,
          max: 10.,
          last_hurt_by: None,
        },
        Collider {
          radius: collider_radius,
          damage: ROID_COLLISION_DAMAGE,
        },
        Roid(next_size.clone()),
        Rotation(rotation),
        PhysicsObject::new(mass),
        Acceleration{ acceleration: Vec3::ZERO, max_speed: ROID_MAX_SPEED }
      ));
    }
  }
}


fn spawn_roids(
  mut commands: Commands, 
  scene_assets: Res<SceneAssets>,
  mut spawn_reader:MessageReader<SpawnMessage>,
){
  let mut rng = rand::rng();
  for spawn in spawn_reader.read(){
    let scale: Vec3;
    let collider_radius: f32;
    let next_size: RoidSize;
    let mass:f32;
    match spawn.spawn_type{
      SpawnType::Roid => {
        scale = ROID_LARGE_SCALE;
        collider_radius = ROID_LARGE_RADIUS;
        next_size = RoidSize::Medium;
        mass = ROID_LARGE_MASS;
      }
      SpawnType::RoidMedium => {
        scale = ROID_MEDIUM_SCALE;
        collider_radius = ROID_MEDIUM_RADIUS;
        next_size = RoidSize::Small;
        mass = ROID_MEDIUM_MASS;
      }
      SpawnType::RoidSmall => {
        scale = ROID_SMALL_SCALE;
        collider_radius = ROID_SMALL_RADIUS;
        next_size = RoidSize::Small;
        mass = ROID_SMALL_MASS;
      }
      _ => continue
    }

    let rotation = Vec3::new(
      rng.random_range(-1. ..1.),
      rng.random_range(-1. ..1.),
      rng.random_range(-1. ..1.),
    );

    commands.spawn((
      GameEntity,
      Roid(RoidSize::Large),
      BoundsWarp(false),
      Transform::from_translation(spawn.position).with_scale(ROID_LARGE_SCALE),
      Velocity(spawn.velocity),
      SceneRoot(scene_assets.roid1.clone()),
      Rotation(rotation),
      Collider {
        radius: ROID_LARGE_RADIUS,
        damage: ROID_COLLISION_DAMAGE,
      },
      Health {
        value: 10.,
        max: 10.,
        last_hurt_by: None,
      },
      PhysicsObject::new(ROID_LARGE_MASS),
      Acceleration{ acceleration: Vec3::ZERO, max_speed: ROID_MAX_SPEED}
    ));
  }
}

/*
  //spawn a new wave
  spawn_timer.count-=1;
  
  let mut rng = rand::rng();
  for _ in 0..level.wave_size {
    let angle = rng.random_range(0. ..PI * 2.);
    let return_angle = angle + rng.random_range(-0.3..0.3);

    let rotation = Vec3::new(
      rng.random_range(-1. ..1.),
      rng.random_range(-1. ..1.),
      rng.random_range(-1. ..1.),
    );

    let start_position = Vec3::new(angle.cos(), 0., angle.sin()) * ROID_SPAWN_DISTANCE;
    let velocity = Vec3::new(return_angle.cos(), 0., return_angle.sin())
      * -rng.random_range(level.max_speed - level.speed_variance..level.max_speed);
    //let velocity = Vec3::ZERO;

    commands.spawn((
      GameEntity,
      Roid(RoidSize::Large),
      BoundsWarp(false),
      Transform::from_translation(start_position).with_scale(ROID_LARGE_SCALE),
      Velocity(velocity),
      SceneRoot(scene_assets.roid1.clone()),
      Rotation(rotation),
      Collider {
        radius: ROID_LARGE_RADIUS,
        damage: ROID_COLLISION_DAMAGE,
      },
      Health {
        value: 10.,
        max: 10.,
        last_hurt_by: None,
      },
      PhysicsObject::new(ROID_LARGE_MASS),
      Acceleration{ acceleration: Vec3::ZERO, max_speed: ROID_MAX_SPEED}
    ));
     */
