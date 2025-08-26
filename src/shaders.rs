
use bevy::{
  prelude::*,
  render::render_resource::{AsBindGroup, ShaderRef},
};
pub struct ShadersPlugin;

impl Plugin for ShadersPlugin{
  fn build(&self, app: &mut App) {
    app
      .init_resource::<ShaderMaterials>()
      .add_plugins(MaterialPlugin::<StarfieldMaterial>::default())
      .add_plugins(MaterialPlugin::<BoundaryMaterial>::default())
      .add_systems(PreStartup, init_materials);
  }
}

const STARFIELD_SHADER_PATH: &str = "shaders/starfield_material.wgsl";
const BOUNDS_SHADER_PATH: &str = "shaders/bounds_material.wgsl";

#[derive(Resource, Default)]
pub struct ShaderMaterials{
  pub starfield:Handle<StarfieldMaterial>,
  pub bounds:Handle<BoundaryMaterial>,
}

fn init_materials(
  mut commands:Commands,
  mut starfield_materials: ResMut<Assets<StarfieldMaterial>>,
  mut bounds_materials: ResMut<Assets<BoundaryMaterial>>,
){
  let shader_materials = ShaderMaterials{ 
    starfield:
      starfield_materials.add(StarfieldMaterial {
        alpha_mode: AlphaMode::AlphaToCoverage,
      }),
    bounds:
      bounds_materials.add(BoundaryMaterial {
        color1: LinearRgba::rgb(0.8, 0.8, 0.),
        color2: LinearRgba::rgb(0.8, 0., 0.),
        alpha_mode: AlphaMode::AlphaToCoverage,
      }),
   };

   commands.insert_resource::<ShaderMaterials>(shader_materials);
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct StarfieldMaterial {
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




#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BoundaryMaterial {
  #[uniform(0)]
  color1: LinearRgba,
  #[uniform(1)]
  color2: LinearRgba,
  alpha_mode: AlphaMode,
}

impl Material for BoundaryMaterial {
  fn fragment_shader() -> ShaderRef {
    BOUNDS_SHADER_PATH.into()
  }
  fn alpha_mode(&self) -> AlphaMode {
    self.alpha_mode
  }
}
