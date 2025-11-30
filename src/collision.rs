use bevy::prelude::*;
use bitmask_enum::bitmask;


use crate::{
  bullet::{Bullet, BulletHitMessage}, health::HealthMessage, movement::{PhysicsMessage, PhysicsObject}, player::{Invulnerable, PlayerShip, Shield}, scheduling::GameSchedule
};
pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      PostUpdate,
      (bullet_collisions, collider_collisions, shield_collisions).in_set(GameSchedule::CollisionDetection),
    )
    //.add_systems(Update, _add_collision_shell)
    ;
  }
}



#[derive(Default)]
#[bitmask]
pub enum CollisionFlags{
  #[default]
  None = 0,
  Asteroid, 
  Player,
  Enemy,
}
    

#[derive(Component, Default)]
pub struct Collider {
  pub owner: Option<Entity>,
  pub radius: f32,
  pub damage: f32,
  pub collison_group: CollisionFlags,
  pub collision_mask: CollisionFlags,
}

fn _add_collision_shell(
  mut commands: Commands,
  query: Query<(Entity, &Collider, &Transform), Added<Collider>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  for (entity, collider, transform) in query.iter() {
    let material = materials.add(StandardMaterial {
      base_color: Color::linear_rgba(0., 0.9, 0., 0.2),
      alpha_mode: AlphaMode::Blend,
      ..Default::default()
    });
    commands.spawn((
      Mesh3d(meshes.add(Sphere::new(collider.radius / transform.scale.x))),
      MeshMaterial3d(material),
      ChildOf(entity),
    ));
  }
}

fn shield_collisions(
  player: Query<( &Collider, &GlobalTransform, &Shield)>,
  baddies: Query<(Entity, &Collider, &GlobalTransform),(With<PhysicsObject>, Without<PlayerShip>, Without<Shield>)>,
  mut mw_physics: MessageWriter<PhysicsMessage>,
) {
  for (player_collider, player_transform, shield) in player.iter() {
    for (enemy_entity, enemy_collider, enemy_transform) in baddies.iter() {
      let dist_squared = player_transform
        .translation()
        .distance_squared(enemy_transform.translation());
      let allowed_dist = player_collider.radius + enemy_collider.radius;
      if dist_squared < allowed_dist * allowed_dist {
        //info!("ent collision {:?} {:?}", player_entity, enemy_entity);
        let launch_vector = (enemy_transform.translation() - player_transform.translation()).normalize();
        mw_physics.write(PhysicsMessage::new(enemy_entity, launch_vector * shield.repulse_force ));
      }
    }
  }
}

fn  collider_collisions(
  entity_query:Query<(Entity, &Collider, &GlobalTransform), Without<Invulnerable>>,
  mut health_writer: MessageWriter<HealthMessage>,
){
  for (entity_a, collider_a, transform_a) in entity_query.iter(){
    for (entity_b, collider_b, transform_b) in entity_query.iter(){
      if entity_a == entity_b || collider_a.collision_mask & collider_b.collison_group != collider_b.collison_group{
        continue;
      }
      let dist_squared = transform_a.translation().distance_squared(transform_b.translation());
      let allowed_dist = collider_a.radius + collider_b.radius;
      if dist_squared < allowed_dist * allowed_dist{
        info!("ent collision {:?} {:?}", entity_a, entity_b);
        health_writer.write(HealthMessage::new(entity_a, Some(entity_b), collider_b.damage));
        health_writer.write(HealthMessage::new(entity_b, Some(entity_a), collider_a.damage));
      }
    }
  }
}

fn bullet_collisions(
  bullets: Query<(Entity, &Bullet, &GlobalTransform)>,
  targets: Query<(Entity, &Collider, &GlobalTransform)>,
  mut health_writer: MessageWriter<HealthMessage>,
  mut bullet_hit_writer: MessageWriter<BulletHitMessage>,
){
  for (bullet_entity, bullet, bullet_transform) in bullets.iter(){
    for (target_entity, collider, target_transform) in targets.iter(){
      if bullet.owner == target_entity || (collider.owner.is_some() && collider.owner.unwrap() == bullet.owner){
        continue;
      }
      let dist_squared = bullet_transform.translation().distance_squared(target_transform.translation());
      if dist_squared < collider.radius * collider.radius{
        info!("bullet hit ent {:?}", target_entity);
        health_writer.write(HealthMessage::new(target_entity, Some(bullet.owner), bullet.damage));
        bullet_hit_writer.write(BulletHitMessage::new(bullet_entity));
      }
    }
  }
}

