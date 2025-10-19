use bevy::{prelude::*, time::Stopwatch};

use crate::{
  asset_loader::AssetState, bounds::{Bounds, BoundsWarp}, input::{InputEventAction, InputEventType, InputTriggerMessage}, level::CurrentLevel, roid::Roid, scheduling::GameSchedule
};


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

#[derive(Component)]
pub struct LevelTarget;

#[derive(Resource, Default)]
pub struct CurrentLevelIndex(pub usize);



#[derive(Component)]
pub struct GameEntity;

#[derive(Component)]
pub struct LevelEntity;

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_state::<GameState>()
      .init_state::<PauseState>()
      .init_resource::<CurrentLevel>()
      .insert_resource(CurrentLevelIndex(0))
      .insert_resource(GameManager{ level_time: Stopwatch::new(), level_test_timer: Timer::from_seconds(0.5, TimerMode::Repeating)})
     // .insert_resource(LevelData(LEVEL_DATA.to_vec()))
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
  mut current_level_index:ResMut<CurrentLevelIndex>
) {
  current_level_index.0 = 0;
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
  mut current_level_index:ResMut<CurrentLevelIndex>,
  cleanup_query:Query<Entity, With<LevelEntity>>,
  mut commands:Commands,
) {
  current_level_index.0 += 1;
  info!("next level {}", current_level_index.0);
  for entity in cleanup_query{
    commands.entity(entity).try_despawn();
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
  //current_level:Res<CurrentLevel>,
  mut game_manager:ResMut<GameManager>,
  time:Res<Time>,
  target_query:Query<&LevelTarget>,
  mut next_state: ResMut<NextState<GameState>>,
){
  //let Some(level) = current_level.0.clone() else{ return; };
  game_manager.level_test_timer.tick(time.delta());
  if !game_manager.level_test_timer.just_finished(){ return; }
  if !target_query.is_empty(){ return; }
  /*
  for bounds in target_query.iter(){
    if bounds.is_some_and(|f| f.0) || bounds.is_none() { 
      return; 
    }
  }
   */
  info!("LEVEL END");
  next_state.set(GameState::LevelEnd);
}

fn check_for_pause(
  mut msg_input_reader: MessageReader<InputTriggerMessage>,
  mut next_state: ResMut<NextState<PauseState>>,
) {
  for InputTriggerMessage { action, input_type } in msg_input_reader.read() {
    if *input_type == InputEventType::Pressed && *action == InputEventAction::Pause {
      info!("Pausing game");
      next_state.set(PauseState::Paused);
    }
  }
}
