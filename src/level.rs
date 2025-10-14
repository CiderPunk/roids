use std::{clone, f32::consts::PI, ops::Range};

use bevy::prelude::*;
use rand::Rng;

use crate::{asset_loader::{AssetState, LevelHandle}, bounds::Bounds, game_manager::{CurrentLevelIndex, GameEntity, GameState, LevelEntity, LevelTarget}, movement::Velocity, scheduling::GameSchedule};


const ROID_SPAWN_DISTANCE: f32 = 150.0;
const CORNERS:[(f32, f32);4] = [(1.,1.),(-1.,1.),(-1.,-1.),(1.,-1.), ];

pub struct LevelPlugin;

impl Plugin for LevelPlugin{
  fn build(&self, app: &mut App) {
    app
      .init_asset::<LevelCollectionData>()
      .add_message::<SpawnMessage>()
      .init_resource::<Levels>()
      .init_resource::<CurrentLevel>()
      .add_systems(OnExit(AssetState::Loading), extract_levels)
      .add_systems(OnEnter(GameState::LevelInit), init_level)
      .add_systems(Update, update_spawners.in_set(GameSchedule::EntityUpdates).run_if(in_state(GameState::Alive)));

  }
}


fn update_spawners(
  query:Query<(Entity, &mut WaveSpawner)>,
  mut spawn_write:MessageWriter<SpawnMessage>,
  time:Res<Time>,
  mut commands:Commands,
  bounds:Single<&Bounds>,
){
  let mut rng = rand::rng();  
  for (entity, mut spawner) in query{
    spawner.start_time.tick(time.delta());
    if spawner.start_time.is_finished(){
      spawner.cycle_time.tick(time.delta());
      if spawner.start_time.just_finished() || spawner.cycle_time.just_finished(){
        //spawn a wave
        spawner.wave_count -= 1;

        for _ in [0 .. spawner.wave_data.wave_size]{
          let angle = rng.random_range(0. ..PI * 2.);
          let return_angle = angle + rng.random_range(-0.3..0.3);

          let position = Vec3::new(angle.cos(), 0., angle.sin()) * ROID_SPAWN_DISTANCE;
          //range
          let velocity = Vec3::new(return_angle.cos(), 0., return_angle.sin()) * -rng.random_range(spawner.wave_data.min_speed .. spawner.wave_data.max_speed);

          spawn_write.write(SpawnMessage{ 
            spawn_type: spawner.spawn_type.clone() , 
            position,
            velocity, 
          });
        }

        if spawner.wave_count == 0{
          //despawn spawner
          commands.entity(entity).despawn();
          info!("Despawning wave spawner");
        }
      }
    }
  }
}

fn get_target_range(half_bounds:Vec3, position:Vec3)->Range<f32>{
  for corner in CORNERS{


  }

  (0.0 .. 1.0)
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
  must_complete:Option<bool>,
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
    levels.0 = level_data.levels.to_vec();
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
    let mut entity = commands.spawn((
      WaveSpawner{
        spawn_type: wave.wave_type.clone(),
        start_time: Timer::from_seconds(wave.first_spawn, TimerMode::Once),
        cycle_time: Timer::from_seconds(wave.cycle_time, TimerMode::Repeating),
        wave_count: wave.cycles,
        wave_data: wave.clone(),
      },
      GameEntity,
      LevelEntity,
    ));
    if let Some(true) = wave.must_complete {
      entity.insert(LevelTarget);
    }
    
  }
}