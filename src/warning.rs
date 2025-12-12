use std::f32::consts::PI;
use bevy::{prelude::*, scene};
use strum_macros::EnumIter;
use crate::{asset_loader::SceneAssets, bounds::{Bounds, InBounds}, game_manager::LevelEntity, movement::Velocity, scheduling::GameSchedule};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum WarningType{
  Ufo,
  Missile,
  Roid,
}


pub struct WarningPlugin;

impl Plugin for WarningPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, (spawn_warnings, despawn_warning).in_set(GameSchedule::EntityUpdates));
  }
}

#[derive(Component)]
pub struct Warn{
  pub warn_type:WarningType,
  pub entity:Option<Entity>,
}

impl Warn{
  pub fn new(warn_type:WarningType) -> Self{
    Self{
      warn_type,
      entity:None,
    }
  }
}

#[derive(Component)]
struct Warning(Entity);


fn despawn_warning(
  mut commands: Commands,
  mut query:Query<&Warn, Added<InBounds>>,
){
  for warn in query.iter_mut(){
    if let Some(warning_entity) = warn.entity{
      commands.entity(warning_entity).despawn();
    }
  }
}

fn spawn_warnings(
  bounds:Single<&Bounds>,
  mut query:Query<(&mut Warn, &GlobalTransform, &Velocity), (Added<Warn>, Without<InBounds>)>,
  mut commands: Commands,
  scene_assets: Res<SceneAssets>,
){


  for (mut warn, transform, velocity) in query.iter_mut(){
    let intersect = calc_intersect(bounds.half_size, transform.translation(), velocity.0);
    if intersect.is_none(){
      continue;
    }
    let (intersect_point, intersect_time) = intersect.unwrap();

    let inset = velocity.0.normalize() * 12.0;
    let position = intersect_point + inset;
    let angle = velocity.0.x.atan2(velocity.0.z);

    let warning_entity = commands.spawn((
      LevelEntity,
      Transform::from_translation(position).with_scale(Vec3::splat(3.)).with_rotation(Quat::from_axis_angle(Vec3::Y, angle + PI)),
      SceneRoot(scene_assets.warning_back.clone()),
    )).id();

    warn.entity = Some(warning_entity);
    
  }
}


fn calc_intersect(half_size: Vec3, origin: Vec3, velocity: Vec3) -> Option<(Vec3, f32)> {
  let mut t_in = f32::NEG_INFINITY;
  let mut t_out = f32::INFINITY;
  const EPSILON: f32 = 1e-6; // A small floating-point tolerance

  if velocity.x.abs() < EPSILON{
    if origin.z.abs() > half_size.z{
      return None;
    }
  }
  else{
    let t_1 = (-half_size.x - origin.x) / velocity.x;
    let t_2 = (half_size.x - origin.x) / velocity.x;
    t_in = t_in.max(t_1.min(t_2));
    t_out = t_out.min(t_1.max(t_2));
  }

  if velocity.z.abs() < EPSILON{
    if origin.x.abs() > half_size.x{
      return None;
    }
  }
  else{
    let t_1 = (-half_size.z - origin.z) / velocity.z;
    let t_2 = (half_size.z - origin.z) / velocity.z;
    t_in = t_in.max(t_1.min(t_2));
    t_out = t_out.min(t_1.max(t_2));
  }

  if t_in > t_out{
    return None;
  }
  
  //calculate actual intersect
  Some((Vec3::new(origin.x + velocity.x * t_in, 0., origin.z + velocity.z * t_in), t_in))
}

