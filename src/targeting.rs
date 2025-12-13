use core::f32;

use bevy::{ecs::entity::index_map::IterMut, prelude::*, state::commands};

use crate::{player::{Player, PlayerShip}, scheduling::GameSchedule};

pub struct TargetingPlugin;
impl Plugin for TargetingPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, update_target.in_set(GameSchedule::EntityUpdates))
      .add_observer(aquire_target);
  }
}

#[derive(Component, Default)]
#[require(Transform)]
pub struct Targeter{
  pub target:Option<Entity>,
  target_update_timer:Timer,
  
}

impl Targeter{
  pub fn new() -> Self{
    Targeter{
      target:None,
      target_update_timer:Timer::from_seconds(0.2, TimerMode::Repeating),
    }
  }
}

fn update_target(
  mut commands:Commands,
  mut query:Query<(Entity, &mut Targeter)>,
  target_query:Query<Entity>,
  time:Res<Time>,
){
  for (entity, mut targeter) in query.iter_mut(){
    targeter.target_update_timer.tick(time.delta());
    if targeter.target_update_timer.just_finished(){
      if targeter.target.is_some() && target_query.get(targeter.target.unwrap()).is_ok(){
        continue;
      }
      info!("Reacquiring target for entity {:}", entity);
      commands.entity(entity).insert(Targeter::new());
    }
  }
}


fn aquire_target(
  added:On<Insert, Targeter>,
  mut query:Query<(&GlobalTransform, &mut Targeter)>,
  target_query:Query<(&GlobalTransform, Entity), With<PlayerShip>>,
){
  let (transform, mut targeter) = query.get_mut(added.entity).unwrap();
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

/*
fn aquire_target(
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
 */