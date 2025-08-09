use std::f32::consts::PI;

use bevy::{math::VectorSpace, prelude::*};
use rand::Rng;
use crate::{asset_loader::SceneAssets, bounds::BoundsWarp, game_manager::GameState, movement::{Rotation, Velocity}};


const ROID_SPAWN_DISTANCE:f32 = 150.0;
const ROID_LOW_SPEED:f32 = 4.;
const ROID_HIGH_SPEED:f32 = 20.;


const ROID_LARGE_SCALE:Vec3 = Vec3::splat(6.);
const ROID_MEDIUM_SCALE:Vec3 = Vec3::splat(3.);
const ROID_SMALL_SCALE:Vec3 = Vec3::splat(1.);


pub struct RoidPlugin;

impl Plugin for RoidPlugin{
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(GameState::GameInit), spawn_roids);
  }
}


#[derive(Component)]
struct Roid;

fn spawn_roids(
  mut commands:Commands,
  scene_assets: Res<SceneAssets>,
){
  for _ in 0 .. 15{
    let mut rng = rand::rng();
    
    let angle = rng.random_range(0. .. PI * 2.);
    let return_angle = angle + rng.random_range(-0.3 .. 0.3);
 
    let rotation = Vec3::new(
      rng.random_range(-1. .. 1.),
      rng.random_range(-1. .. 1.),
      rng.random_range(-1. .. 1.));

    let start_position = Vec3::new(angle.cos(), 0., angle.sin()) * ROID_SPAWN_DISTANCE;

    let velocity = Vec3::new(return_angle.cos(), 0., return_angle.sin()) * -rng.random_range(ROID_LOW_SPEED .. ROID_HIGH_SPEED);
    //let velocity = Vec3::ZERO;

    commands.spawn((
      Roid,
      BoundsWarp(false),
      Transform::from_translation(start_position).with_scale(ROID_LARGE_SCALE),
      Velocity(velocity),
      SceneRoot(scene_assets.roid1.clone()),
      Rotation(rotation),
    ));
  }
}