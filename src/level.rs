use std::{clone, f32::consts::PI, ops::Range};
use bevy_rand::{global::GlobalRng, plugin::EntropyPlugin};
use bevy::{color::palettes::css::{CRIMSON, FUCHSIA, GREEN, YELLOW}, math::ops::atan2, prelude::*};
use bevy_prng::WyRand;
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
   //   .add_systems(Startup, init_gizmos)
      .add_systems(OnExit(AssetState::Loading), extract_levels)
      .add_systems(OnEnter(GameState::LevelInit), init_level)
      .add_systems(Update, update_spawners.in_set(GameSchedule::EntityUpdates).run_if(in_state(GameState::Alive)));

  }
}

fn update_spawners(
  query:Query<(Entity, &mut WaveSpawner)>,
  mut spawn_writer:MessageWriter<SpawnMessage>,
  time:Res<Time>,
  mut commands:Commands,
  bounds:Single<&Bounds>,   
  mut rng: Single<&mut WyRand, With<GlobalRng>>
){
  //let mut rng = rand::rng();  
  for (entity, mut spawner) in query{
    //first fire timer
    spawner.start_time.tick(time.delta());
    if !spawner.start_time.is_finished() { continue; }
    //cycle timer
    spawner.cycle_time.tick(time.delta());
    if !(spawner.start_time.just_finished() || spawner.cycle_time.just_finished()) { continue; }
    //spawn a wave
    spawner.wave_count -= 1;

    info!("Spawning wave {:?}", spawner.spawn_type);
    spawn_wave(&mut spawn_writer, spawner.clone(), bounds.half_size, &mut *rng);

    if spawner.wave_count == 0{
      //despawn spawner
      info!("Despawning wave spawner");
      commands.entity(entity).despawn();
    }
  }
}

fn spawn_wave(mut spawn_writer: &mut MessageWriter<SpawnMessage>, spawner: WaveSpawner, half_size: Vec3, rng: &mut WyRand) {
  for _ in 0 .. spawner.wave_data.wave_size{

    let x_dist = spawner.wave_data.x_distribution.unwrap_or(0.9);
    let y_dist = spawner.wave_data.y_distribution.unwrap_or(0.9);
    let x_iter = spawner.wave_data.x_iterations.unwrap_or(1);
    let y_iter = spawner.wave_data.y_iterations.unwrap_or(1);
    
    //pick target point
    let target = Vec3::new(bell_curve_distribute(-x_dist * half_size.x .. x_dist * half_size.x, rng, x_iter as usize), 0.,bell_curve_distribute(-y_dist * half_size.z .. y_dist * half_size.z, rng, y_iter as usize));
    let target = loop_constrain(target, half_size);

    let mut direction_range = 0. .. PI*2.;
    if let Some(direction_collection) = &spawner.wave_data.spawn_direction {
      let direction_array = direction_collection[ rng.random_range(0 .. direction_collection.len())];
      direction_range = direction_array[0] .. direction_array[1];
    }

    //pick angle
    let angle:f32 = rng.random_range(direction_range);

    let direction = Vec3::new(angle.cos(), 0., angle.sin());

    let distance = rng.random_range(
      spawner.wave_data.min_spawn_distance.unwrap_or(ROID_SPAWN_DISTANCE) ..
      spawner.wave_data.max_spawn_distance.unwrap_or(ROID_SPAWN_DISTANCE *1.1)
    );

    let Some(intersect) = bounds_intersect(target, direction, distance) else{ continue; };
    spawn_writer.write(SpawnMessage { 
      spawn_type: spawner.spawn_type, 
      position: intersect,
      velocity: -direction * rng.random_range(spawner.wave_data.min_speed .. spawner.wave_data.max_speed),
      in_bounds:false,
    });
  }
}


/// returns Restricted target to inside the half_size zone looping to the other end for out of bounds
fn loop_constrain(target: Vec3, half_size: Vec3) -> Vec3 {
  let mut target = target;
  while target.x > half_size.x{
    target.x -= half_size.x * 2.;
  }  
  while target.x < -half_size.x{
    target.x += half_size.x * 2.;
  }  
  while target.z > half_size.z{
    target.z -= half_size.z * 2.;
  }  
  while target.z < -half_size.z{
    target.z += half_size.z * 2.;
  }  
  target
}

fn bounds_intersect(
  position:Vec3,
  direction_unit:Vec3, 
  radius:f32,
)->Option<Vec3>{
  let b = 2. * direction_unit.dot(position);
  let c = position.length_squared() - (radius * radius);
  let discriminant = b *b - 4.0 * c;
  if discriminant < 0.0 {
    return None;
  }
  let sqrt_discriminant = discriminant.sqrt();
  let t = (-b + sqrt_discriminant) / 2.0;
  Some(position + (direction_unit * t))
}


fn bell_curve_distribute(
  range:Range<f32>,
  rng: &mut WyRand,  
  samples:usize,
)->f32{

  if range.start == range.end{
    return range.start;
  }
  let mut total:f32 = 0.;
  for _ in 0 .. samples{
    total += rng.random_range(range.clone());
  }
  total / samples as f32
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
  spawn_direction:Option<Vec<[f32;2]>>,
  x_distribution:Option<f32>,
  y_distribution:Option<f32>,
  x_iterations:Option<u32>,
  y_iterations:Option<u32>,
  min_spawn_distance:Option<f32>,
  max_spawn_distance:Option<f32>,
}

#[derive(serde::Deserialize, Asset, TypePath, Clone, Copy, Debug)]
pub enum SpawnType{
  Roid,
  RoidSmall,
  RoidMedium,
  Ufo,
  Missile,
}

#[derive(Message)]
pub struct SpawnMessage{
  pub spawn_type:SpawnType,
  pub position:Vec3,
  pub velocity:Vec3,
  pub in_bounds:bool,
}

#[derive(Component, Clone)]
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