use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, bounds::{Bounds, InBounds}, game_manager::GameState, movement::Velocity, scheduling::GameSchedule};

pub struct WarningPlugin;

impl Plugin for WarningPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, spawm_warnings.in_set(GameSchedule::EntityUpdates));
  }
}

#[derive(Component)]
pub struct Warn;


#[derive(Component)]
struct Warning{
  entity:Entity,
}



fn spawm_warnings(
  bounds:Single<&Bounds>,
  query:Query<(Entity, &Warn, &GlobalTransform, &Velocity), (Added<Warn>, Without<InBounds>)>,
  mut commands: Commands,
  scene_assets: Res<SceneAssets>,
){

  for (entity, warn, transform, velocity) in query.iter(){
    let intersect = calc_intersect(bounds.half_size, transform.translation(), velocity.0);
    match intersect{
        Some(intersect) => {
          info!("Spawning warning at {:}", intersect);
    commands.spawn((
      Warning{
        entity,
      },
      Transform::from_translation(intersect).with_scale(Vec3::splat(10.)),

      SceneRoot(scene_assets.warning_back.clone()),
      //MeshMaterial3d(scene_assets.bullet_material.clone()),
      
      //Mesh3d(scene_assets.bullet.clone()),
    ));

        },
        None => (),
    }

  }

}


fn calc_intersect(half_size: Vec3, origin: Vec3, velocity: Vec3) -> Option<Vec3> {
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
  Some(Vec3::new(origin.x + velocity.x * t_in, 0., origin.z + velocity.z * t_in))
}

