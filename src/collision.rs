use bevy::prelude::*;

use crate::{
  bullet::{Bullet, BulletHitMessage}, health::HealthMessage, movement::{PhysicsMessage, PhysicsObject}, player::{Invulnerable, PlayerShip, Shield}, scheduling::GameSchedule
};
pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      PostUpdate,
      (bullet_collisions, detect_player_collisions, shield_collisions).in_set(GameSchedule::CollisionDetection),
    )
    //.add_systems(Update, _add_collision_shell)
    ;
  }
}




#[derive(Component, Default)]
pub struct Collider {
  pub radius: f32,
  pub damage: f32,
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

fn detect_player_collisions(
  player: Query<(Entity, &Collider, &GlobalTransform), (With<PlayerShip>, Without<Invulnerable>)>,
  baddies: Query<(Entity, &Collider, &GlobalTransform), Without<PlayerShip>>,
  mut health_writer: MessageWriter<HealthMessage>,
) {
  for (player_entity, player_collider, player_transform) in player.iter() {
    for (enemy_entity, enemy_collider, enemy_transform) in baddies.iter() {
      let dist_squared = player_transform
        .translation()
        .distance_squared(enemy_transform.translation());
      let allowded_dist = player_collider.radius + enemy_collider.radius;
      if dist_squared < allowded_dist * allowded_dist {
        //info!("ent collision {:?} {:?}", player_entity, enemy_entity);
        health_writer.write(HealthMessage::new(
          player_entity,
          Some(enemy_entity),
          enemy_collider.damage,
        ));
        health_writer.write(HealthMessage::new(
          enemy_entity,
          Some(player_entity),
          player_collider.damage,
        ));
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
      if bullet.owner.is_some() && bullet.owner.unwrap() == target_entity{
        continue;
      }
      let dist_squared = bullet_transform.translation().distance_squared(target_transform.translation());
      if dist_squared < collider.radius * collider.radius{
        info!("bullet hit ent {:?}", target_entity);
        health_writer.write(HealthMessage::new(target_entity, bullet.owner, bullet.damage));
        bullet_hit_writer.write(BulletHitMessage::new(bullet_entity));
      }
    }
  }
}


fn detect_bullet_collisions(
  bullets: Query<(Entity, &Bullet, &GlobalTransform)>,
  players: Query<(Entity, &Collider, &GlobalTransform), With<PlayerShip>>,
  baddies: Query<(Entity, &Collider, &GlobalTransform), (Without<PlayerShip>, Without<Shield>)>,
  mut health_writer: MessageWriter<HealthMessage>,
  mut bullet_hit_writer: MessageWriter<BulletHitMessage>,
) {
  for (bullet_entity, bullet, bullet_transform) in bullets.iter() {
    if bullet.is_players {
      for (target_entity, collider, target_transform) in baddies.iter() {
        let dist_squared = bullet_transform
          .translation()
          .distance_squared(target_transform.translation());
        if dist_squared < collider.radius * collider.radius {
          //info!("bullet hit ent {:?}", target_entity);
          health_writer.write(HealthMessage::new(target_entity, bullet.owner, bullet.damage));
          bullet_hit_writer.write(BulletHitMessage::new(bullet_entity));
        }
      }
    } else {
      for (target_entity, collider, target_transform) in players.iter() {
        let dist_squared = bullet_transform
          .translation()
          .distance_squared(target_transform.translation());
        if dist_squared < collider.radius * collider.radius {
          //info!("bullet hit player {:?}", target_entity);
          health_writer.write(HealthMessage::new(target_entity, bullet.owner, bullet.damage));
          bullet_hit_writer.write(BulletHitMessage::new(bullet_entity));
        }
      }
    }
  }
}
