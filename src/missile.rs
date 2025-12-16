use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, bounds::{BoundsDespawn, InBounds}, collision::{Collider, CollisionFlags}, game_manager::{GameEntity, LevelEntity, LevelTarget}, health::Health, level::{SpawnMessage, SpawnType}, movement::{Acceleration, Rotation, Velocity}, scheduling::GameSchedule, targeting::Targeter, warning::Warn};
pub struct MissilePlugin;

impl Plugin for MissilePlugin{
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, (spawn_missiles.in_set(GameSchedule::EntityUpdates),  update_missiles.in_set(GameSchedule::PreEntityUpdates)));
    ;
  }
}

#[derive(Component)]
pub struct Missile{
  angle:f32,
}

const MISSILE_ACCELERATION: f32 = 20.;
const MISSILE_TURN_RATE:f32 = 3.;
const NAVIGATION_GAIN_N: f32 = 3.;


fn update_missiles(
  mut query:Query<(&Targeter, &mut Acceleration, &mut Transform, &Velocity), (With<Missile>, With<InBounds>)>,
  target_query:Query<(&GlobalTransform, &Velocity)>,
  time:Res<Time>,
){
  for ( targeter, mut acceleration, mut transform, velocity) in query.iter_mut(){
    if targeter.target.is_none(){
      continue;
    }
    if let Ok((target_transform, target_velocity)) = target_query.get(targeter.target.unwrap()){

      let relative_position = target_transform.translation() - transform.translation;
      let relative_velocity =  target_velocity.0 - velocity.0;

      // Closing velocity (scalar)
      let closing_velocity = -relative_velocity.dot(relative_position.normalize());

      //2D cross product to get angle rate
      let angle_rate = (relative_position.x * relative_velocity.z - relative_position.z * relative_velocity.x) / relative_position.length_squared();
      let normal = Vec3::new(-relative_position.z, 0.0, relative_position.x).normalize();
      //let angle_rate = relative_position.cross(relative_velocity) / relative_position.length_squared();
      let optimal_vector = NAVIGATION_GAIN_N * closing_velocity * angle_rate * normal;

      //let optimal_vector = NAVIGATION_GAIN_N * relative_velocity * angle_rate;

      let target_angle = optimal_vector.x.atan2(optimal_vector.z);
      let current_angle = transform.rotation.to_euler(EulerRot::YXZ).0;

      //info!("current angle: {:}, target angle: {:}", current_angle, target_angle);


      let mut diff = target_angle - current_angle;
      if diff < -PI{
        diff += PI * 2.;
      }
      if diff > PI{
        diff -= PI * 2.;
      }

      let max_turn_rate = MISSILE_TURN_RATE * time.delta_secs();
      let turn = diff.clamp(-max_turn_rate, max_turn_rate);

      transform.rotation = Quat::from_axis_angle(Vec3::Y, current_angle + turn);

      acceleration.acceleration = Vec3::new(
        MISSILE_ACCELERATION * current_angle.sin(), 
        0., 
        MISSILE_ACCELERATION * current_angle.cos());
 
    }
  }
}

fn spawn_missiles(
  mut commands: Commands, 
  scene_assets: Res<SceneAssets>,
  mut spawn_reader:MessageReader<SpawnMessage>,
){
  for spawn in spawn_reader.read(){
    match spawn.spawn_type{
      SpawnType::Missile => spawn_missile(&spawn,&mut commands, scene_assets.clone()),
      _ => continue
    }
  }
}
    
fn spawn_missile(
  spawn: &SpawnMessage,
  commands: &mut Commands, 
  scene_assets: SceneAssets,
){
  let init_angle = spawn.velocity.x.atan2(spawn.velocity.z);

  info!("Spawning Missiles at {:}", spawn.position);
  commands.spawn((
    LevelTarget,
    GameEntity,
    LevelEntity,
    Targeter::new(),
    Missile{ angle:init_angle, },
    BoundsDespawn,
    Transform::from_translation(spawn.position).with_scale(Vec3::splat(1.)).with_rotation(Quat::from_axis_angle(Vec3::Y, init_angle)),
    Velocity(spawn.velocity),
    SceneRoot(scene_assets.missile.clone()),
    Acceleration{ acceleration: Vec3::ZERO, max_speed: 40. },
    Collider {
      collison_group: CollisionFlags::Enemy,
      collision_mask: CollisionFlags::Player | CollisionFlags::Asteroid,
      owner: None,
      radius: 0.75,
      damage: -20.0,
    },

    Health {
      value: 5.,
      max: 5.,
      last_hurt_by: None,
    },
    Warn::new(crate::warning::WarningType::Missile),
    //PhysicsObject::new(10.0),
  ));
}