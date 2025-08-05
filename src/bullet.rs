use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, game::PauseState, movement::Velocity};

pub struct BulletPlugin;

impl Plugin for BulletPlugin{
  fn build(&self, app: &mut App) {
    app.add_event::<ShootEvent>()
    .add_systems(Update, do_shooting.run_if(in_state(PauseState::Running)));
      
  }
}

fn do_shooting(
  mut commands: Commands,
  mut ev_shoot_events: EventReader<ShootEvent>,
  scene_assets: Res<SceneAssets>,
) {
  for &ShootEvent {
    is_player,
    start,
    velocity,
    damage,
    scale,
    owner,
  } in ev_shoot_events.read()
  {
    let transform =  Transform::from_translation(start).with_scale(Vec3::new(scale,scale,scale));
    commands.spawn((
      Bullet { damage, owner:Some(owner) },
      Mesh3d(scene_assets.bullet.clone()),
      MeshMaterial3d(scene_assets.bullet_material.clone()),
      transform,
      Velocity(velocity),
    ));
  }
}



#[derive(Event)]
pub struct ShootEvent {
  pub is_player: bool,
  pub start: Vec3,
  pub velocity: Vec3,
  pub damage: f32,
  pub scale:f32,
  pub owner:Entity
}

impl ShootEvent {
  pub fn new(is_player: bool, start: Vec3, velocity: Vec3, damage: f32, scale:f32, owner:Entity) -> Self {
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


#[derive(Component)]
#[require(Velocity)]
pub struct Bullet {
  //pub hit: bool,
  pub damage: f32,
  pub owner:Option<Entity>,
}
