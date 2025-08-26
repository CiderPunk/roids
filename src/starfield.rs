use std::f32::consts::PI;

use bevy::prelude::*;

use crate::shaders::ShaderMaterials;

pub struct StarfieldPlugin;

impl Plugin for StarfieldPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, spawn_starfield);
  }
}

fn spawn_starfield(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  materials:Res<ShaderMaterials>,
) {
  info!("spawned starfield");
  let quad = meshes.add(Rectangle::new(400.0, 400.0));
  commands.spawn((
    Mesh3d(quad),
    MeshMaterial3d(materials.starfield.clone()),
    Transform::from_xyz(0.0, -50., 0.).with_rotation(Quat::from_rotation_x(PI * -0.5)),
  ));
}
