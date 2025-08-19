use std::f32::consts::PI;

use crate::{
  asset_loader::SceneAssets,
  bounds::BoundsWarp,
  collision::Collider,
  effect_sprite::EffectSpriteEvent,
  game_manager::{GameEntity, GameState},
  health::Health,
  movement::{Rotation, Velocity},
  scheduling::GameSchedule,
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
const ROID_LOW_SPEED: f32 = 4.;
const ROID_HIGH_SPEED: f32 = 20.;

const ROID_LARGE_SCALE: Vec3 = Vec3::splat(5.);
const ROID_MEDIUM_SCALE: Vec3 = Vec3::splat(3.);
const ROID_SMALL_SCALE: Vec3 = Vec3::splat(1.);

const ROID_LARGE_RADIUS: f32 = 6.5;
const ROID_MEDIUM_RADIUS: f32 = 4.;
const ROID_SMALL_RADIUS: f32 = 1.5;

const ROID_COLLISION_DAMAGE: f32 = -100.;

pub struct RoidPlugin;

impl Plugin for RoidPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::GameInit), spawn_roids)
      .add_systems(
        Update,
        check_asteroid_health.in_set(GameSchedule::PreDespawnEntities),
      );
  }
}

#[derive(Component, Default, Deref, DerefMut)]
struct Roid(RoidSize);

fn check_asteroid_health(
  mut commands: Commands,
  query: Query<(&Roid, &Health, &GlobalTransform, &Velocity)>,
  mut ev_effect_writer: EventWriter<EffectSpriteEvent>,
  scene_assets: Res<SceneAssets>,
) {
  let mut rng = rand::rng();
  for (roid, health, transform, velocity) in query.iter() {
    if health.value > 0. {
      continue;
    }

    let scale = match roid.0 {
      RoidSize::Large => 16.,
      RoidSize::Medium => 12.,
      RoidSize::Small => 8.,
    };
    ev_effect_writer.write(EffectSpriteEvent::new(
      transform.translation(),
      scale,
      velocity.0,
      crate::effect_sprite::EffectSpriteType::Splosion,
    ));

    if roid.0 == RoidSize::Small {
      continue;
    }

    let scale: Vec3;
    let collider_radius: f32;
    let next_size: RoidSize;
    match roid.0 {
      RoidSize::Large => {
        scale = ROID_MEDIUM_SCALE;
        collider_radius = ROID_MEDIUM_RADIUS;
        next_size = RoidSize::Medium;
      }
      RoidSize::Medium => {
        scale = ROID_SMALL_SCALE;
        collider_radius = ROID_SMALL_RADIUS;
        next_size = RoidSize::Small;
      }
      RoidSize::Small => {
        scale = ROID_SMALL_SCALE;
        collider_radius = ROID_SMALL_RADIUS;
        next_size = RoidSize::Small;
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
        BoundsWarp(true),
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
      ));
    }
  }
}

fn spawn_roids(mut commands: Commands, scene_assets: Res<SceneAssets>) {
  let mut rng = rand::rng();
  for _ in 0..15 {
    let angle = rng.random_range(0. ..PI * 2.);
    let return_angle = angle + rng.random_range(-0.3..0.3);

    let rotation = Vec3::new(
      rng.random_range(-1. ..1.),
      rng.random_range(-1. ..1.),
      rng.random_range(-1. ..1.),
    );

    let start_position = Vec3::new(angle.cos(), 0., angle.sin()) * ROID_SPAWN_DISTANCE;
    let velocity = Vec3::new(return_angle.cos(), 0., return_angle.sin())
      * -rng.random_range(ROID_LOW_SPEED..ROID_HIGH_SPEED);
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
    ));
  }
}
