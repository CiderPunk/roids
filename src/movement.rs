use bevy::{math::VectorSpace, prelude::*};

use crate::scheduling::GameSchedule;

//const STOPPED_SPEED_SQUARED: f32 = 2.;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<PhysicsEvent>()
      .add_systems(
      Update,
      (sum_physics_events, update_acceleration, update_velocity, apply_damping, update_position, update_rotation)
        .chain()
        .in_set(GameSchedule::EntityUpdates),
    );
  }
}



#[derive(Component, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

#[derive(Component, Default)]
#[require(Velocity)]
pub struct Acceleration {
  pub acceleration: Vec3,
  pub max_speed: f32,
}

#[derive(Component, Default)]
#[require(Acceleration)]
pub struct Damping{
  pub amount:f32,
  pub min_speed:f32,
}



#[derive(Component,Default)]
#[require(Acceleration)]
pub struct PhysicsObject{
  pub mass: f32,
  pub force: Vec3,
}

impl PhysicsObject{
  pub fn new(mass:f32) -> Self{
    Self{ mass, force: Vec3::ZERO } 
  }
}

fn sum_physics_events(
  mut ev_physics_reader: EventReader<PhysicsEvent>,
  mut entity_query:Query<&mut PhysicsObject>,
){
  for PhysicsEvent {entity, force} in ev_physics_reader.read(){
    let Ok(mut physics) = entity_query.get_mut(*entity) else{ continue; };
    physics.force += force;
  }
}

fn update_acceleration(
  query:Query<(&mut PhysicsObject, &mut Acceleration)>
){
  for (mut physics, mut acceleration) in query{
    acceleration.acceleration += physics.force / physics.mass;
    physics.force = Vec3::ZERO;
  }
}

#[derive(Event)]
pub struct PhysicsEvent{
  entity:Entity,
  force:Vec3,
}

impl PhysicsEvent{
  pub fn new (entity:Entity, force:Vec3)->Self{
    Self{entity, force} 
  }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct Rotation(pub Vec3);

fn update_position(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
  for (mut transform, velocity) in &mut query {
    transform.translation += velocity.0 * time.delta_secs();
  }
}

fn update_velocity(query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
  for (mut velocity, acceleration) in query {
    velocity.0 += acceleration.acceleration * time.delta_secs();
    //keep below max speed
    if velocity.length_squared() > acceleration.max_speed * acceleration.max_speed {
      velocity.0 = velocity.normalize() * acceleration.max_speed;
    }
  }
}


fn apply_damping(query:Query<(&Acceleration, &Damping, &mut Velocity)>){

  for (acceleration, damping, mut velocity) in query {

    if acceleration.acceleration == Vec3::ZERO && velocity.0.length_squared() < damping.min_speed{
      velocity.0 = Vec3::ZERO;
      continue;
    }
    //damping
    let adjust = velocity.0.normalize_or_zero() * damping.amount;
    velocity.0 -= adjust;
  }
}

fn update_rotation(mut query: Query<(&mut Transform, &Rotation)>, time: Res<Time>) {
  for (mut transform, rotation) in query.iter_mut() {
    if rotation.x != 0. {
      transform.rotate_local_x(rotation.x * time.delta_secs());
    }
    if rotation.y != 0. {
      transform.rotate_local_y(rotation.y * time.delta_secs());
    }
    if rotation.z != 0. {
      transform.rotate_local_z(rotation.z * time.delta_secs());
    }
  }
}
