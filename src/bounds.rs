use bevy::{asset::RenderAssetUsages, prelude::*, render::{mesh::Indices, render_resource::{AsBindGroup, ShaderRef}}};
use crate::game_manager::GameState;

const BOUNDS_SHADER_PATH:&str = "shaders/bounds_material.wgsl";
const BOUNDS_SIZE:Vec3 = Vec3::new(115.0, 0., 65.0);
const BOUNDS_BORDER_SIZE:f32 = 20.;

pub struct BoundsPlugin;

impl Plugin for BoundsPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_plugins(MaterialPlugin::<CustomMaterial>::default())
      .add_systems(OnEnter(GameState::GameInit), build_bounds_mesh)
      .add_systems(Update, (bounds_despawn, bounds_warp));
  }
}

#[derive(Component)]
struct Bounds{
  half_size:Vec3,
}

#[derive(Component)]
pub struct BoundsDespawn;


#[derive(Component, Default, Deref, DerefMut)]
pub struct BoundsWarp(pub bool);

fn bounds_despawn(
  mut commands:Commands,
  bounds:Query<&Bounds>,
  query:Query<(Entity, &GlobalTransform), With<BoundsDespawn>>,
){
  let Ok(Bounds{ half_size }) = bounds.single() else{
    return;
  };

  for (entity, transform) in  query.iter(){
    let translation = transform.translation().abs();
    if translation.x > half_size.x || translation.z > half_size.z{
     commands.entity(entity).despawn();
    }
  }
}

fn bounds_warp(
  bounds:Query<&Bounds>,
  mut query:Query<(&mut Transform, &mut BoundsWarp)>,
){
  let Ok(Bounds{ half_size }) = bounds.single() else{
    return;
  };
  for (mut transform, mut bounds_warp) in &mut query{
    if bounds_warp.0{
      if transform.translation.x < -half_size.x{
        transform.translation.x += half_size.x * 2.;
      }
      else if transform.translation.x > half_size.x{
        transform.translation.x -= half_size.x * 2.;
      }
      if transform.translation.z < -half_size.z{
        transform.translation.z += half_size.z * 2.;
      }
      else if transform.translation.z > half_size.z{
        transform.translation.z -= half_size.z * 2.;
      }
    }
    else{
      let translation = transform.translation.abs();
      if translation.x < half_size.x && translation.z < half_size.z{
        bounds_warp.0 = true;
      }
    }
  }
}

fn build_bounds_mesh(
  mut commands:Commands,
  //scene_assets: Res<SceneAssets>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials:ResMut<Assets<CustomMaterial>>,
){
  info!("creating bounds mesh");
  let mesh_handle: Handle<Mesh> = meshes.add(create_frame_mesh(BOUNDS_SIZE.x, BOUNDS_SIZE.z, BOUNDS_BORDER_SIZE));
  let material_handle = materials.add(CustomMaterial{
    color1: LinearRgba::rgb(0.8,0.8,0.),
    color2: LinearRgba::rgb(0.8,0.,0.),
    alpha_mode:AlphaMode::AlphaToCoverage,
  });

  commands.spawn((   
    Bounds{ half_size: BOUNDS_SIZE, },
    Mesh3d(mesh_handle),
    MeshMaterial3d(material_handle),
    Transform::from_translation(Vec3::Y * 5.0),
  ));
}


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color1: LinearRgba,
    #[uniform(1)]
    color2: LinearRgba,
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

fn create_frame_mesh(half_width:f32, half_height:f32, border:f32) -> Mesh{
  let hw = half_width - (border*0.5);
  let hh = half_height - (border* 0.5);
  let hhb = hh + (border * 0.5);
  let hwb = hw + (border * 0.5);
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