use bevy::prelude::*;

use crate::game::PauseState;

const STOPPED_SPEED_SQUARED: f32 = 2.;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      Update,
      (update_velocity, update_position, update_rotation)
        .chain()
        .run_if(in_state(PauseState::Running)),
    );
  }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

#[derive(Component, Default)]
pub struct Acceleration {
  pub acceleration: Vec3,
  pub max_speed: f32,
  pub min_speed: f32,
  pub damping: f32,
}

impl Acceleration {
  pub fn new(acceleration: Vec3, max_speed: f32, min_speed: f32, damping: f32) -> Self {
    Self {
      acceleration,
      max_speed,
      min_speed,
      damping,
    }
  }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct Rotation(pub Vec3);

fn update_position(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
  for (mut transform, velocity) in &mut query {
    transform.translation += velocity.0 * time.delta_secs();
  }
}

fn update_velocity(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
  for (mut velocity, acceleration) in &mut query {
    let mut acc = acceleration.acceleration;

    //stopped
    if acc == Vec3::ZERO && velocity.length_squared() < acceleration.min_speed {
      velocity.0 = Vec3::ZERO;
      continue;
    }
    //damping
    if acceleration.damping > 0. {
      acc -= velocity.normalize_or_zero() * acceleration.damping;
    }
    velocity.0 += acc * time.delta_secs();
    //keep below max speed
    if velocity.length_squared() > acceleration.max_speed * acceleration.max_speed {
      velocity.0 = velocity.normalize() * acceleration.max_speed;
    }
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
