use bevy::{asset::RenderAssetUsages, math::VectorSpace, prelude::*, render::{mesh::Indices, render_resource::{AsBindGroup, ShaderRef}}};

use crate::{asset_loader::SceneAssets, game_manager::GameState};


const BOUNDS_SHADER_PATH:&str = "shaders/bounds_material.wgsl";

pub struct BoundsPlugin;

impl Plugin for BoundsPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_plugins(MaterialPlugin::<CustomMaterial>::default())
      .add_systems(OnEnter(GameState::GameInit), build_bounds_mesh);
  }
}

fn build_bounds_mesh(
  mut commands:Commands,
  //scene_assets: Res<SceneAssets>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials:ResMut<Assets<CustomMaterial>>,
){
  info!("creating bounds mesh");
  let mesh_handle: Handle<Mesh> = meshes.add(create_frame_mesh(200., 120., 10.));
  let material_handle = materials.add(CustomMaterial{
    color: LinearRgba::rgb(0.,0.,1.),
    alpha_mode:AlphaMode::AlphaToCoverage,
  });

  commands.spawn((   
    Mesh3d(mesh_handle),
    MeshMaterial3d(material_handle),
    Transform::from_translation(Vec3::Y * 5.0),
  ));
}


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        BOUNDS_SHADER_PATH.into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

fn create_frame_mesh(width:f32, height:f32, border:f32) -> Mesh{
  let hw = width / 2.;
  let hh = height / 2.;
  let hhb = hh + border;
  let hwb = hw + border;
  Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList, RenderAssetUsages::default())
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, 
      vec![
        [-hw, 0., -hh],[hw,0.,-hh],[hw,0.,hh],[-hw,0.,hh], 
        [-hwb, 0., -hhb],[hwb,0.,-hhb],[hwb,0.,hhb],[-hwb,0.,hhb]
      ]
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, 
      vec![
        [0.,0.],[1.,0.],[0.,0.],[1.,0.],
        [0.,1.],[1.,1.],[0.,1.],[1.,1.]
      ]
    )
    .with_inserted_indices(Indices::U32(vec![
      0,5,4,
      0,1,5,
      1,2,6,
      1,6,5,
      2,7,6,
      2,3,7,
      3,0,4,
      3,4,7
    ]))
}