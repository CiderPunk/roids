use bevy::prelude::*;

use crate::{
  asset_loader::SceneAssets, bounds::BoundsWarp, game::PauseState, movement::Velocity,
  scheduling::GameSchedule,
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<ShootEvent>()
      .add_event::<BulletHitEvent>()
      .add_systems(
        Update,
        (do_shooting, time_to_live, bullet_hit).in_set(GameSchedule::EntityUpdates),
      );
  }
}

fn do_shooting(
  mut commands: Commands,
  mut ev_shoot_reader: EventReader<ShootEvent>,
  scene_assets: Res<SceneAssets>,
) {
  for &ShootEvent {
    is_player,
    start,
    velocity,
    damage,
    scale,
    owner,
  } in ev_shoot_reader.read()
  {
    let transform = Transform::from_translation(start).with_scale(Vec3::new(scale, scale, scale));
    commands.spawn((
      BoundsWarp(true),
      Bullet {
        damage,
        owner: Some(owner),
        is_players: is_player,
      },
      Mesh3d(scene_assets.bullet.clone()),
      MeshMaterial3d(scene_assets.bullet_material.clone()),
      transform,
      Velocity(velocity),
      TimeToLive(3.0),
    ));
  }
}

fn bullet_hit(mut commands: Commands, mut ev_bullet_hit_reader: EventReader<BulletHitEvent>) {
  for &BulletHitEvent { bullet } in ev_bullet_hit_reader.read() {
    //add effect
    commands.entity(bullet).despawn();
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

#[derive(Event)]
pub struct ShootEvent {
  pub is_player: bool,
  pub start: Vec3,
  pub velocity: Vec3,
  pub damage: f32,
  pub scale: f32,
  pub owner: Entity,
}

impl ShootEvent {
  pub fn new(
    is_player: bool,
    start: Vec3,
    velocity: Vec3,
    damage: f32,
    scale: f32,
    owner: Entity,
  ) -> Self {
    Self {
      is_player,
      start,
      velocity,
      damage,
      scale,
      owner,
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

#[derive(Event)]
pub struct BulletHitEvent {
  bullet: Entity,
}

impl BulletHitEvent {
  pub fn new(entity: Entity) -> Self {
    Self { bullet: entity }
  }
}
