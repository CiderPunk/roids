use std::f32::consts::PI;

use bevy::{
  prelude::*,
  render::render_resource::{AsBindGroup, ShaderRef},
};

pub struct StarfieldPlugin;

impl Plugin for StarfieldPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(MaterialPlugin::<StarfieldMaterial>::default())
      .add_systems(Startup, spawn_starfield);
  }
}

const STARFIELD_SHADER_PATH: &str = "shaders/starfield_material.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct StarfieldMaterial {
  alpha_mode: AlphaMode,
}

impl Material for StarfieldMaterial {
  fn fragment_shader() -> ShaderRef {
    STARFIELD_SHADER_PATH.into()
  }
  fn alpha_mode(&self) -> AlphaMode {
    self.alpha_mode
  }
}

fn spawn_starfield(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StarfieldMaterial>>,
) {
  info!("spawned starfield");
  let quad = meshes.add(Rectangle::new(400.0, 400.0));
  //let quad = meshes.add(Sphere::new(20.));
  let material_handle = materials.add(StarfieldMaterial {
    alpha_mode: AlphaMode::AlphaToCoverage,
  });
  commands.spawn((
    Mesh3d(quad),
    MeshMaterial3d(material_handle),
    Transform::from_xyz(0.0, -50., 0.).with_rotation(Quat::from_rotation_x(PI * -0.5)),
  ));
}
