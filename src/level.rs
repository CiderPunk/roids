use std::clone;

use bevy::prelude::*;

use crate::{asset_loader::{AssetState, LevelHandle}, game_manager::{CurrentLevelIndex, GameEntity, GameState}, movement::Velocity};


pub struct LevelPlugin;

impl Plugin for LevelPlugin{
  fn build(&self, app: &mut App) {
    app
      .init_asset::<LevelCollectionData>()
      .add_message::<SpawnMessage>()
      .init_resource::<Levels>()
      .add_systems(OnExit(AssetState::Loading), extract_levels)
      .add_systems(OnEnter(GameState::LevelInit), init_level);

  }
}

#[derive(serde::Deserialize, Asset, TypePath, Clone, Copy)]
pub struct LevelConfiguration{
  pub wave_size:u32,
  pub wave_count:u32,
  pub wave_time:f32,
  pub max_speed:f32,
  pub speed_variance:f32,
  pub time_before_comnplete:f32,
}

pub const LEVEL_DATA: [LevelConfiguration; 4] =[
  LevelConfiguration{ wave_size: 2, wave_count: 1, wave_time: 10., max_speed: 30., speed_variance: 15., time_before_comnplete:5. },
  LevelConfiguration{ wave_size: 1, wave_count: 10, wave_time: 1., max_speed: 40., speed_variance: 10., time_before_comnplete:5. },
  LevelConfiguration{ wave_size: 10, wave_count: 1, wave_time: 10., max_speed: 30., speed_variance: 15., time_before_comnplete:5. },
  LevelConfiguration{ wave_size: 4, wave_count: 2, wave_time: 10., max_speed: 30., speed_variance: 15., time_before_comnplete: 5.},
];


#[derive(serde::Deserialize, Asset, TypePath)]
pub struct LevelCollectionData{
  levels:Vec<LevelData>
}

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub struct LevelData{
  name:String,
  min_level_time:f32,
  waves:Vec<WaveData>,
}


#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub struct WaveData{
  wave_type:SpawnType,
  first_spawn:f32,
  cycle_time:f32,
  cycles:u32,
  wave_size:u32,
  min_speed:f32,
  max_speed:f32,
  min_spawn_radians:Option<f32>,
  max_spawn_radians:Option<f32>,
}

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
enum SpawnType{
  Roid,
  Ufo,
}

#[derive(Message)]
pub struct SpawnMessage{
  spawn_type:SpawnType,
  position:Transform,
  velocity:Velocity,
}

#[derive(Component)]
pub struct WaveSpawner{
  spawn_type:SpawnType,
  start_time:Timer,
  cycle_time:Timer,
  wave_count:u32,
  wave_data:WaveData,
}


#[derive(Resource, Default)]
pub struct Levels(pub Vec<LevelData>);

fn extract_levels(
  level_handle: Res<LevelHandle>,
  level_assets: Res<Assets<LevelCollectionData>>,
  mut levels: ResMut<Levels>,
){
  if let Some(level_data) = level_assets.get(level_handle.0.id()){
    levels.0.clone_from_slice(&level_data.levels);
  }
}


fn init_level(
  current_level_index:Res<CurrentLevelIndex>,
  levels:Res<Levels>,
  mut commands:Commands,
){
  let level = levels.0[current_level_index.0].clone();
  //spawn spawners...
  for wave in level.waves.iter(){
    commands.spawn((
      WaveSpawner{
        spawn_type: wave.wave_type.clone(),
        start_time: Timer::from_seconds(wave.first_spawn, TimerMode::Once),
        cycle_time: Timer::from_seconds(wave.cycle_time, TimerMode::Repeating),
        wave_count: wave.cycles,
        wave_data: wave.clone(),
      },
      GameEntity,
    ));
  }
}