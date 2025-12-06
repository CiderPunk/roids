use crate::{
  camera::CameraBoundsChangeMessage, game_manager::{GameEntity, GameState}, scheduling::GameSchedule, shaders::ShaderMaterials
};
use bevy::{
  asset::RenderAssetUsages, mesh::{Indices, PrimitiveTopology}, prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef
};

const BOUNDS_SHADER_PATH: &str = "shaders/bounds_material.wgsl";
const BOUNDS_SIZE: Vec3 = Vec3::new(115.0, 0., 65.0);
//const BOUNDS_SIZE: Vec3 = Vec3::new(150.0, 0., 150.0);
const BOUNDS_BORDER_SIZE: f32 = 20.;

pub struct BoundsPlugin;

impl Plugin for BoundsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(MaterialPlugin::<CustomMaterial>::default())
      .add_systems(OnEnter(GameState::GameInit), spawn_bounds)
      .add_systems(
        Update,
        (
          bounds_despawn.in_set(GameSchedule::DespawnEntities),
          (bounds_warp, bounds_enter).chain().in_set(GameSchedule::EntityUpdates),
        ),
      );
  }
}

#[derive(Component)]
pub struct Bounds {
  pub half_size: Vec3,
}

#[derive(Component)]
pub struct InBounds;

#[derive(Component)]
pub struct BoundsDespawn;

#[derive(Component)]
pub struct BoundsWarp{
  pub warp_vertically: bool,
  pub warp_horizontally: bool,
}


impl Default for BoundsWarp {
  fn default() -> Self {
    BoundsWarp {
      warp_vertically: true,
      warp_horizontally: true,
    }
  }
} 

fn bounds_enter(
  mut commands: Commands,
  bounds: Single<&Bounds>,
  query: Query<(Entity, &GlobalTransform), (Without<InBounds> , Or<(With<BoundsDespawn>, With<BoundsWarp>)>)>,
){
 for (entity, transform) in query.iter() {
    let translation = transform.translation().abs();
    if translation.x < bounds.half_size.x && translation.z < bounds.half_size.z {
      info!("Entity {:?} entered bounds", entity);
      commands.entity(entity).insert(InBounds);
    }
  }
}

fn bounds_despawn(
  mut commands: Commands,
  bounds: Single<&Bounds>,
  query: Query<(Entity, &GlobalTransform), (With<BoundsDespawn>, With<InBounds>)>,
) {
  for (entity, transform) in query.iter() {
    let translation = transform.translation().abs();
    if translation.x > bounds.half_size.x || translation.z > bounds.half_size.z {
      commands.entity(entity).despawn();
    }
  }
}



fn bounds_warp(
  bounds: Single<&Bounds>, 
  mut query: Query<(&mut Transform, &BoundsWarp), With<InBounds>>
) {
  for (mut transform, bounds_warp) in &mut query {
    let abs_translation = transform.translation.abs();
    if bounds_warp.warp_horizontally && abs_translation.x > bounds.half_size.x {
      transform.translation.x += bounds.half_size.x * 2. * -transform.translation.x.signum();
    }
    if bounds_warp.warp_vertically && abs_translation.z > bounds.half_size.z {
      transform.translation.z += bounds.half_size.z * 2. * -transform.translation.z.signum();
    } 
  }
}

fn spawn_bounds(
  mut commands: Commands,
  //scene_assets: Res<SceneAssets>,
  mut meshes: ResMut<Assets<Mesh>>,
  materials:Res<ShaderMaterials>,
  mut ev_bounds_writer: MessageWriter<CameraBoundsChangeMessage>,
) {
  info!("creating bounds mesh");
  let mesh_handle: Handle<Mesh> = meshes.add(create_frame_mesh(
    BOUNDS_SIZE.x,
    BOUNDS_SIZE.z,
    BOUNDS_BORDER_SIZE,
  ));

  commands.spawn((
    GameEntity,
    Bounds {
      half_size: BOUNDS_SIZE,
    },
    Mesh3d(mesh_handle),
    MeshMaterial3d(materials.bounds.clone()),
    Transform::from_translation(Vec3::Y * 5.0),
  ));
  ev_bounds_writer.write(CameraBoundsChangeMessage);
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

fn create_frame_mesh(half_width: f32, half_height: f32, border: f32) -> Mesh {
  let hw = half_width - (border * 0.5);
  let hh = half_height - (border * 0.5);
  let hhb = hh + (border * 0.5);
  let hwb = hw + (border * 0.5);
  Mesh::new(
    PrimitiveTopology::TriangleList,
    RenderAssetUsages::default(),
  )
  .with_inserted_attribute(
    Mesh::ATTRIBUTE_POSITION,
    vec![
      [-hw, 0., -hh],
      [hw, 0., -hh],
      [hw, 0., hh],
      [-hw, 0., hh],
      [-hwb, 0., -hhb],
      [hwb, 0., -hhb],
      [hwb, 0., hhb],
      [-hwb, 0., hhb],
    ],
  )
  .with_inserted_attribute(
    Mesh::ATTRIBUTE_UV_0,
    vec![
      [0., 0.],
      [1., 0.],
      [0., 0.],
      [1., 0.],
      [0., 1.],
      [1., 1.],
      [0., 1.],
      [1., 1.],
    ],
  )
  .with_inserted_indices(Indices::U32(vec![
    0, 5, 4, 0, 1, 5, 1, 2, 6, 1, 6, 5, 2, 7, 6, 2, 3, 7, 3, 0, 4, 3, 4, 7,
  ]))
}
