use std::os::unix::raw::time_t;

use bevy::prelude::*;

use crate::{
  asset_loader::SceneAssets, bounds::BoundsWarp, effect_sprite::EffectSpriteMessage,
  game_manager::GameEntity, movement::Velocity, scheduling::GameSchedule,
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_message::<ShootMessage>()
      .add_message::<BulletHitMessage>()
      .add_systems(
        Update,
        (do_shooting, time_to_live, bullet_hit).in_set(GameSchedule::EntityUpdates),
      );
  }
}

fn do_shooting(
  mut commands: Commands,
  mut ev_shoot_reader: MessageReader<ShootMessage>,
  scene_assets: Res<SceneAssets>,
) {
  for &ShootMessage {
    is_player,
    start,
    velocity,
    damage,
    scale,
    owner,
    time_to_live,
  } in ev_shoot_reader.read()
  {
    let transform = Transform::from_translation(start).with_scale(Vec3::splat(scale));
    commands.spawn((
      GameEntity,
      BoundsWarp::default(),
      Bullet {
        damage,
        owner: Some(owner),
        is_players: is_player,
      },
      Mesh3d(scene_assets.bullet.clone()),
      MeshMaterial3d(scene_assets.bullet_material.clone()),
      transform,
      Velocity(velocity),
      TimeToLive(time_to_live),
    ));
  }
}

fn bullet_hit(
  mut commands: Commands,
  mut ev_bullet_hit_reader: MessageReader<BulletHitMessage>,
  mut ev_effect_writer: MessageWriter<EffectSpriteMessage>,
  query: Query<&GlobalTransform>,
) {
  for &BulletHitMessage { bullet } in ev_bullet_hit_reader.read() {
    //add effect
    commands.entity(bullet).despawn();
    let Ok(transform) = query.get(bullet) else {
      continue;
    };
    ev_effect_writer.write(EffectSpriteMessage::new(
      transform.translation(),
      4.,
      Vec3::ZERO,
      crate::effect_sprite::EffectSpriteType::Ricochet,
    ));
  }
}

fn time_to_live(
  mut commands: Commands,
  mut query: Query<(&mut TimeToLive, Entity)>,
  time: Res<Time>,
) {
  for (mut time_to_live, entity) in &mut query {
    time_to_live.0 -= time.delta_secs();
    if time_to_live.0 < 0. {
      commands.entity(entity).despawn();
    }
  }
}

#[derive(Message)]
pub struct ShootMessage {
  pub is_player: bool,
  pub start: Vec3,
  pub velocity: Vec3,
  pub damage: f32,
  pub scale: f32,
  pub owner: Entity,
  pub time_to_live: f32,
}

impl ShootMessage {
  pub fn new(
    is_player: bool,
    start: Vec3,
    velocity: Vec3,
    damage: f32,
    scale: f32,
    owner: Entity,
    time_to_live: f32,
  ) -> Self {
    Self {
      is_player,
      start,
      velocity,
      damage,
      scale,
      owner,
      time_to_live,
    }
  }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct TimeToLive(pub f32);

#[derive(Component)]
#[require(Velocity)]
pub struct Bullet {
  pub is_players: bool,
  pub damage: f32,
  pub owner: Option<Entity>,
}

#[derive(Message)]
pub struct BulletHitMessage {
  bullet: Entity,
}

impl BulletHitMessage {
  pub fn new(entity: Entity) -> Self {
    Self { bullet: entity }
  }
}
