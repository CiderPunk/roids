use core::f32;

use bevy::{ecs::entity::index_map::IterMut, prelude::*};

use crate::player::{Player, PlayerShip};

pub struct TargetingPlugin;
impl Plugin for TargetingPlugin{
  fn build(&self, app: &mut App) {
    app.add_systems(Update, aquire_target);  
  }
}

#[derive(Component, Default)]
pub struct Targeter{
  pub target:Option<Entity>
}

pub fn aquire_target(
  mut query:Query<(&GlobalTransform, &mut Targeter), Added<Targeter>>,
  target_query:Query<(&GlobalTransform, Entity), With<PlayerShip>>,
){
  for (transform, mut targeter) in query.iter_mut(){
    info!("Looking for targets...");
    let mut nearest_dist = f32::INFINITY;
    let mut nearest_entity:Option<Entity> = None;
    for (target_transform, target_entity) in target_query.iter(){
      let dist = (target_transform.translation() - transform.translation()).length_squared();
      if dist < nearest_dist{
        info!("Found target at {:}", target_transform.translation());
        nearest_dist = dist ;
        nearest_entity = Some(target_entity);
      }
    }
    targeter.target = nearest_entity;
  }
}
