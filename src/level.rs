use bevy::prelude::*;



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