use std::{clone, f32::consts::PI, ops::Range};
use bevy_rand::{global::GlobalRng, plugin::EntropyPlugin};
use bevy::{color::palettes::css::{CRIMSON, FUCHSIA, GREEN, YELLOW}, math::ops::atan2, prelude::*};
use bevy_prng::WyRand;
use rand::Rng;


use rand_distr::{Distribution, Normal};
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


/*
fn init_gizmos( 
  mut commands:Commands,
  mut gizmo_assets: ResMut<Assets<GizmoAsset>>){

   let mut gizmo = GizmoAsset::new();
       gizmo
        .sphere(Isometry3d::IDENTITY, 20., CRIMSON)
        .resolution(30_000 / 3);
      commands.spawn((
        Gizmo {
            handle: gizmo_assets.add(gizmo),
            line_config: GizmoLineConfig {
                width: 5.,
                ..default()
            },
            ..default()
        },
        Transform::from_xyz(4., 1., 0.),
    ));
}
 */

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
    //pick target point
    let target = Vec3::new(bell_curve_distribute(-half_size.x .. half_size.x, rng, 2), 0.,bell_curve_distribute(-half_size.z .. half_size.z, rng, 2 ));
    //pick angle
    let angle:f32 = rng.random_range(0. .. PI * 2.);
    let direction = Vec3::new(angle.cos(), 0., angle.sin());
    let Some(intersect) = bounds_intersect(target, direction, ROID_SPAWN_DISTANCE) else{ continue; };
    spawn_writer.write(SpawnMessage { 
      spawn_type: spawner.spawn_type, 
      position: intersect,
      velocity: -direction * rng.random_range(spawner.wave_data.min_speed .. spawner.wave_data.max_speed) });
  }
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
  let mut total:f32 = 0.;
  for _ in 0 .. samples{
    total += rng.random_range(range.clone());
  }
  total / samples as f32
}


/*
fn update_spawners(
  query:Query<(Entity, &mut WaveSpawner)>,
  mut spawn_write:MessageWriter<SpawnMessage>,
  time:Res<Time>,
  mut commands:Commands,
  bounds:Single<&Bounds>,   
  mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
  mut rng: Single<&mut WyRand, With<GlobalRng>>
){
  //let mut rng = rand::rng();  
  for (entity, mut spawner) in query{
    spawner.start_time.tick(time.delta());
    if spawner.start_time.is_finished(){
      spawner.cycle_time.tick(time.delta());
      if spawner.start_time.just_finished() || spawner.cycle_time.just_finished(){
        //spawn a wave
        spawner.wave_count -= 1;

        for _ in [0 .. spawner.wave_data.wave_size]{

          //let normal_x = Normal::new(0., 100.).unwrap();
          //let x = normal_x.sample(&mut *rng);


          let target = Vec3::new(rng.random_range(-bounds.half_size.x .. bounds.half_size.x),0., rng.random_range(-bounds.half_size.z .. bounds.half_size.z));
          let angle:f32 = rng.random_range(0. ..PI * 2.);
          
          let angle:f32 = PI;
          //let return_angle = angle + rng.random_range(-0.3..0.3);
          let position = Vec3::new(angle.cos(), 0., angle.sin()) * ROID_SPAWN_DISTANCE;
          //range
          let target_arc = get_target_arc(bounds.half_size, position);
          info!("Target arc {:?}", target_arc);

          let return_angle = rng.random_range(target_arc.clone());
          let velocity = Vec3::new(return_angle.cos(), 0., return_angle.sin()) * -rng.random_range(spawner.wave_data.min_speed .. spawner.wave_data.max_speed);

//let target_arc = 0. .. PI * -0.5;


      let mut gizmo = GizmoAsset::new();
      gizmo.arrow(position, position + velocity, FUCHSIA);

      let start = target_arc.clone().start;
      const ARC_LENGTH:f32 = 50.;
    gizmo.arrow(position, position + Vec3::new(start.cos(), 0.,start.sin()) * ARC_LENGTH , YELLOW);
      let end = target_arc.clone().end;
      gizmo.arrow(position, position + Vec3::new(end.cos(), 0.,end.sin())  * ARC_LENGTH, GREEN);



      commands.spawn(
        Gizmo {
            handle: gizmo_assets.add(gizmo),
            ..default()
        }
      );



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




fn get_target_arc(half_bounds:Vec3, position:Vec3)->Range<f32>{
  let mut low:f32 = PI;
  let mut high:f32 = -PI;
  for corner_transform in CORNERS{
    let corner = Vec3::new(half_bounds.x * corner_transform.0, 0., half_bounds.z * corner_transform.1);
    let diff = corner - position;
    let mut angle = atan2(diff.z, diff.x);
    if angle < 0.{
      angle += PI * 2.;
    }
    low = low.min(angle);
    high = high.max(angle);
  }
  low .. high
}

 */

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

#[derive(serde::Deserialize, Asset, TypePath, Clone, Copy)]
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