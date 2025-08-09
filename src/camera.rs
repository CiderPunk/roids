use bevy::prelude::*;

pub const CAMERA_LOCATION: Vec3 = Vec3::new(0., 160., 0.);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn_camera);
  }
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn((
    Camera3d::default(),
    Camera {
      order: 0,
      ..default()
    },
    Transform::from_translation(CAMERA_LOCATION).looking_at(Vec3::ZERO, Vec3::Z),
  ));
}
