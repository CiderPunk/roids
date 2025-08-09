use bevy::prelude::*;

use crate::{bullet::{Bullet, BulletHitEvent}, health::HealthEvent, player::Player};
pub struct CollisionPlugin;
impl Plugin for CollisionPlugin{
  fn build(&self, app: &mut App) {
    app.add_systems(Update, (detect_bullet_collisions, detect_collisions));
  }
}

#[derive(Component, Default)]
pub struct Collider{
  radius:f32,
  damage:f32,
  owner:Option<Entity>,
}
fn detect_collisions(
  player:Query<(Entity, &Collider, &GlobalTransform), With<Player>>,
  baddies:Query<(Entity, &Collider, &GlobalTransform), Without<Player>>,
  ev_health_writer:EventWriter<HealthEvent>,
){

}


fn detect_bullet_collisions(
  bullets:Query<(Entity, &Bullet, &GlobalTransform)>,
  players:Query<(Entity, &Collider, &GlobalTransform), With<Player>>,
  baddies:Query<(Entity, &Collider, &GlobalTransform), Without<Player>>,
  mut ev_health_writer:EventWriter<HealthEvent>,
  mut ev_bullet_hit_writer:EventWriter<BulletHitEvent>,

){
  for (bullet_entity, bullet, bullet_transform) in bullets.iter(){
    if (bullet.is_players){
      for (target_entity, collider, target_transform) in baddies.iter(){
let dist_squared = bullet_transform.translation()
    .distance_squared(target_transform.translation());
  if (dist_squared < collider.radius * collider.radius){
    info!("hit ent {:?}", target_entity);
    ev_health_writer.write(HealthEvent::new(target_entity, bullet.owner, bullet.damage));
    ev_bullet_hit_writer.write(BulletHitEvent::new(bullet_entity));
  }      }
    }
    else{
      for (target_entity, collider, target_transform) in players.iter(){
let dist_squared = bullet_transform.translation()
    .distance_squared(target_transform.translation());
  if (dist_squared < collider.radius * collider.radius){
    info!("hit ent {:?}", target_entity);
    ev_health_writer.write(HealthEvent::new(target_entity, bullet.owner, bullet.damage));
    ev_bullet_hit_writer.write(BulletHitEvent::new(bullet_entity));
  }      }

    }
  }
}
