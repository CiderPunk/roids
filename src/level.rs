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
      .init_resource::<CurrentLevel>()
      .add_systems(OnExit(AssetState::Loading), extract_levels)
      .add_systems(OnEnter(GameState::LevelInit), init_level);

  }
}


#[derive(Resource, Default)]
pub struct CurrentLevel(pub Option<LevelData>);

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct LevelCollectionData{
  levels:Vec<LevelData>
}

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub struct LevelData{
  pub name:String,
  pub min_level_time:f32,
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
pub enum SpawnType{
  Roid,
  RoidSmall,
  RoidMedium,
  Ufo,
}

#[derive(Message)]
pub struct SpawnMessage{
  pub spawn_type:SpawnType,
  pub position:Vec3,
  pub velocity:Vec3,
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
  mut current_level:ResMut<CurrentLevel>,
  levels:Res<Levels>,
  mut commands:Commands,
){
  let level = levels.0[current_level_index.0 % levels.0.len()].clone();
  current_level.0 = Some(level.clone());


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