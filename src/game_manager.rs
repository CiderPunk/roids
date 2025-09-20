use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::{
  asset_loader::AssetState, bounds::BoundsWarp, input::{InputEventAction, InputEventType, InputTriggerEvent}, roid::Roid, scheduling::GameSchedule
};


//time before which it is not possible to complete a level - give roids time to get on screen
const LEVEL_START_TIME: f32 = 5.0;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum GameState {
  #[default]
  Startup,
  StartScreen,
  GameInit,
  LevelInit,
  LevelEnd,
  Alive,
  Dead,
  GameOver,
  Shutdown,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum PauseState {
  Paused,
  #[default]
  Running,
}


#[derive(Clone, Copy)]
pub struct LevelConfiguration{
  pub wave_size:u32,
  pub wave_count:u32,
  pub wave_time:f32,
  pub max_speed:f32,
  pub speed_variance:f32,
  pub time_before_comnplete:f32,
}

#[derive(Resource, Default)]
pub struct CurrentLevel(pub usize);

pub const LEVEL_DATA: [LevelConfiguration; 4] =[
  LevelConfiguration{ wave_size: 2, wave_count: 1, wave_time: 10., max_speed: 30., speed_variance: 15., time_before_comnplete:5. },
  LevelConfiguration{ wave_size: 1, wave_count: 10, wave_time: 1., max_speed: 40., speed_variance: 10., time_before_comnplete:5. },
  LevelConfiguration{ wave_size: 10, wave_count: 1, wave_time: 10., max_speed: 30., speed_variance: 15., time_before_comnplete:5. },
  LevelConfiguration{ wave_size: 4, wave_count: 2, wave_time: 10., max_speed: 30., speed_variance: 15., time_before_comnplete: 5.},
];


#[derive(Component)]
pub struct GameEntity;

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_state::<GameState>()
      .init_state::<PauseState>()
      .insert_resource(CurrentLevel(0))
      .insert_resource(GameManager{ level_time: Stopwatch::new(), level_test_timer: Timer::from_seconds(0.5, TimerMode::Repeating)})

      .add_systems(OnEnter(AssetState::Ready), start_screen)
      .add_systems(OnEnter(GameState::GameInit), init_game)
      .add_systems(OnEnter(GameState::LevelInit), init_level)
      .add_systems(OnEnter(GameState::LevelEnd), level_end)
      .add_systems(OnExit(GameState::GameOver), clean_game)
      .add_systems(Update, check_for_pause.run_if(in_state(PauseState::Running)))
      .add_systems(Update,check_game_state.in_set(GameSchedule::EntityUpdates)
        .run_if(in_state(GameState::Alive))
      );
  }
}

fn clean_game(mut commands: Commands, query: Query<Entity, With<GameEntity>>) {
  for entity in query {
    commands.entity(entity).try_despawn();
  }
}

fn init_game(
  mut next_state: ResMut<NextState<GameState>>,
  mut current_level:ResMut<CurrentLevel>,
) {

  current_level.0 = 0;
  info!("Game initialized");
  next_state.set(GameState::LevelInit);
}

fn init_level(
  mut next_state: ResMut<NextState<GameState>>,
  mut game_manager:ResMut<GameManager>,
) {
  info!("Level initialized");
  game_manager.level_time.reset();
  next_state.set(GameState::Alive);
}

fn level_end(
  mut current_level:ResMut<CurrentLevel>,
) {
  current_level.0 += 1;
  if current_level.0 > LEVEL_DATA.len(){
    current_level.0 =0;
  }
}



fn start_screen(mut next_state: ResMut<NextState<GameState>>) {
  info!("Switching to start screen");
  next_state.set(GameState::StartScreen);
}


#[derive(Resource)]
struct GameManager{
  level_time:Stopwatch,
  level_test_timer:Timer,
}


fn check_game_state(
  current_level:Res<CurrentLevel>,
  mut game_manager:ResMut<GameManager>,
  time:Res<Time>,
  roid_query:Query<&BoundsWarp, With<Roid>>,
  mut next_state: ResMut<NextState<GameState>>,
){
  game_manager.level_time.tick(time.delta());
  if game_manager.level_time.elapsed_secs() < LEVEL_DATA[current_level.0].time_before_comnplete { return; }

  game_manager.level_test_timer.tick(time.delta());
  
  if !game_manager.level_test_timer.just_finished(){ return; }
  for bounds in roid_query.iter(){
    if bounds.0 { 
      return; }
  }
  info!("LEVEL END");
  next_state.set(GameState::LevelEnd);
}

fn check_for_pause(
  mut ev_input_reader: EventReader<InputTriggerEvent>,
  mut next_state: ResMut<NextState<PauseState>>,
) {
  for InputTriggerEvent { action, input_type } in ev_input_reader.read() {
    if *input_type == InputEventType::Pressed && *action == InputEventAction::Pause {
      info!("Pausing game");
      next_state.set(PauseState::Paused);
    }
  }
}
