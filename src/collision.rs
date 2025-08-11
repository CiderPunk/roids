use bevy::prelude::*;

use crate::{
  bullet::{Bullet, BulletHitEvent},
  health::HealthEvent,
  player::Player,
  scheduling::GameSchedule,
};
pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      PostUpdate,
      (detect_bullet_collisions, detect_collisions).in_set(GameSchedule::CollisionDetection),
    )
    .add_systems(Update, add_collision_shell);
  }
}

#[derive(Component, Default)]
pub struct Collider {
  pub radius: f32,
  pub damage: f32,
}

fn add_collision_shell(
  mut commands:Commands,
  query:Query<(Entity, &Collider, &Transform), Added<Collider>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,

){
  for (entity, collider, transform) in query.iter(){
    let material = materials.add(StandardMaterial{ 
      base_color: Color::linear_rgba(0., 0.9, 0., 0.2), alpha_mode: AlphaMode::Blend,
      ..Default::default()
    });
    commands.spawn((
      Mesh3d(meshes.add(Sphere::new(collider.radius / transform.scale.x))),
      MeshMaterial3d(material),
      ChildOf(entity),
    ));
  }
}


fn detect_collisions(
  player: Query<(Entity, &Collider, &GlobalTransform), With<Player>>,
  baddies: Query<(Entity, &Collider, &GlobalTransform), Without<Player>>,
  mut ev_health_writer: EventWriter<HealthEvent>,
) {
  for (player_entity, player_collider, player_transform) in player.iter() {
    for (enemy_entity, enemy_collider, enemy_transform) in baddies.iter() {
      let dist_squared = player_transform
        .translation()
        .distance_squared(enemy_transform.translation());
      let allowded_dist = player_collider.radius + enemy_collider.radius;
      if dist_squared < allowded_dist * allowded_dist {
        info!("ent collision {:?} {:?}", player_entity, enemy_entity);
        ev_health_writer.write(HealthEvent::new(
          player_entity,
          Some(enemy_entity),
          enemy_collider.damage,
        ));
        ev_health_writer.write(HealthEvent::new(
          enemy_entity,
          Some(player_entity),
          player_collider.damage,
        ));
      }
    }
  }
}

fn detect_bullet_collisions(
  bullets: Query<(Entity, &Bullet, &GlobalTransform)>,
  players: Query<(Entity, &Collider, &GlobalTransform), With<Player>>,
  baddies: Query<(Entity, &Collider, &GlobalTransform), Without<Player>>,
  mut ev_health_writer: EventWriter<HealthEvent>,
  mut ev_bullet_hit_writer: EventWriter<BulletHitEvent>,
) {
  for (bullet_entity, bullet, bullet_transform) in bullets.iter() {
    if bullet.is_players {
      for (target_entity, collider, target_transform) in baddies.iter() {
        let dist_squared = bullet_transform
          .translation()
          .distance_squared(target_transform.translation());
        if dist_squared < collider.radius * collider.radius {
          info!("bullet hit ent {:?}", target_entity);
          ev_health_writer.write(HealthEvent::new(target_entity, bullet.owner, bullet.damage));
          ev_bullet_hit_writer.write(BulletHitEvent::new(bullet_entity));
        }
      }
    } else {
      for (target_entity, collider, target_transform) in players.iter() {
        let dist_squared = bullet_transform
          .translation()
          .distance_squared(target_transform.translation());
        if dist_squared < collider.radius * collider.radius {
          info!("bullet hit player {:?}", target_entity);
          ev_health_writer.write(HealthEvent::new(target_entity, bullet.owner, bullet.damage));
          ev_bullet_hit_writer.write(BulletHitEvent::new(bullet_entity));
        }
      }
    }
  }
}
