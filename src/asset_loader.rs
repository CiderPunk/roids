use bevy::{asset::LoadState, prelude::*};
use bevy_common_assets::json::JsonAssetPlugin;
use crate::level::LevelCollectionData;

const BULLET_COLOUR: LinearRgba = LinearRgba::new(2., 1.8, 0.2, 1.0);
//const SHIELD_SIZE: f32 = 3.;
const SHIELD_SIZE: f32 = 1.0;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum AssetState {
  #[default]
  Loading,
  Extracting,
  Ready,
}

#[derive(Resource, Default)]
pub struct AssetsLoading(pub Vec<UntypedHandle>);

#[derive(Resource, Default, Clone)]
pub struct SceneAssets {
  pub ship: Handle<Scene>,
  pub roid1: Handle<Scene>,
  pub flame: Handle<Scene>,
  pub ufo: Handle<Scene>,
  pub font: Handle<Font>,
  pub bullet: Handle<Mesh>,
  pub bullet_material: Handle<StandardMaterial>,
  pub ship_shield: Handle<Mesh>,
  pub shield_material: Handle<StandardMaterial>,
  pub ship_icon: Handle<Image>,
}

#[derive(Resource)]
struct GameFont(Handle<Font>);

#[derive(Resource, Clone)]
pub struct LevelHandle(pub Handle<LevelCollectionData>);


#[derive(Resource)]
struct RoidsScene(Handle<Gltf>);

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(JsonAssetPlugin::<LevelCollectionData>::new(&["levels.json"]))
      .insert_resource(AssetsLoading::default())
      .init_resource::<SceneAssets>()
      .init_state::<AssetState>()
      .add_systems(Startup, load_assets)
      .add_systems(
        Update,
        check_load_state.run_if(in_state(AssetState::Loading)),
      )
      .add_systems(OnExit(AssetState::Loading), extract_assets);
  }
}

fn load_assets(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut loading: ResMut<AssetsLoading>,
  mut scene_assets: ResMut<SceneAssets>,
) {
  info!("Loading assets");
  let gltf = asset_server.load("scenes/roids.glb");
  loading.0.push(gltf.clone().untyped());
  commands.insert_resource(RoidsScene(gltf));

  let font = asset_server.load("fonts/OpenSans_Condensed-Bold.ttf");
  loading.0.push(font.clone().untyped());
  commands.insert_resource(GameFont(font));

  let ship_icon = asset_server.load("ui/ship_icon.png");
  loading.0.push(ship_icon.clone().untyped());
  scene_assets.ship_icon = ship_icon;

  let level:Handle<LevelCollectionData> = asset_server.load("data/levels.json");
  loading.0.push(level.clone().untyped());
  commands.insert_resource(LevelHandle(level));
  
  //scene_assets.level_data = level;
}

fn check_load_state(
  loading: Res<AssetsLoading>,
  asset_server: Res<AssetServer>,
  mut next_state: ResMut<NextState<AssetState>>,
) {
  info!("Checking load state...");
  if loading
    .0
    .iter()
    .any(|asset|
      !matches!(asset_server.get_load_state(asset.id()), Some(LoadState::Loaded) )
    )
  {
    return;
  }
  info!("Assets loaded");
  next_state.set(AssetState::Extracting);
}

fn extract_assets(
  mut scene_assets: ResMut<SceneAssets>,
  roids_scene: Res<RoidsScene>,
  gltf_assets: Res<Assets<Gltf>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  game_font: Res<GameFont>,
  level_handle: Res<LevelHandle>,
  level_assets: Res<Assets<LevelCollectionData>>,
  mut next_state: ResMut<NextState<AssetState>>,
) {
  let Some(gltf) = gltf_assets.get(&roids_scene.0) else {
    return;
  };
  info!("extracting assets");
  scene_assets.ship = gltf.named_scenes["Ship"].clone();
  scene_assets.roid1 = gltf.named_scenes["Roid1"].clone();
  scene_assets.flame = gltf.named_scenes["Flame"].clone();
  scene_assets.ufo = gltf.named_scenes["Ufo"].clone();


  scene_assets.bullet = meshes.add(
    Sphere::default().mesh().ico(2).unwrap()
  );
  scene_assets.bullet_material = materials.add(StandardMaterial{

    emissive: BULLET_COLOUR,
    ..default()
  });
  scene_assets.font = game_font.0.clone();
  scene_assets.ship_shield = meshes.add(
    Sphere::new(SHIELD_SIZE).mesh().uv(32, 18)
/*
    Sphere::new(SHIELD_SIZE)
      .mesh()
      .ico(4)
      .unwrap()
       */
  );
  scene_assets.shield_material = materials.add(StandardMaterial{
    alpha_mode: AlphaMode::Blend,
    emissive: Srgba::new(0.,0.2,0.8,0.2).into(),
    base_color: Srgba::new(0.,0.2,0.8,0.2).into(),
    diffuse_transmission:0.8,
    ..default()
  });
  next_state.set(AssetState::Ready);
}
