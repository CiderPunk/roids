use bevy::prelude::*;


pub struct LevelPlugin;

impl Plugin for LevelPlugin{
  fn build(&self, app: &mut App) {
    app.init_asset::<LevelCollectionData>();
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

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct LevelData{
  name:String,
  min_level_time:f32,
  waves:Vec<WaveData>,
}


#[derive(serde::Deserialize, Asset, TypePath)]
pub struct WaveData{
  wave_type:WaveType,
  first_spawn:f32,
  cycle_time:f32,
  cycles:u32,
  wave_size:u32,
}

#[derive(serde::Deserialize, Asset, TypePath)]
enum WaveType{
  Roid,
  Ufo,
}