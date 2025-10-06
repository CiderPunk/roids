use bevy::{prelude::*, time::Stopwatch};

use crate::{
  asset_loader::AssetState, bounds::BoundsWarp, input::{InputEventAction, InputEventType, InputTriggerMessage}, level::LEVEL_DATA, roid::Roid, scheduling::GameSchedule
};
use crate::level::LevelConfiguration;

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




#[derive(Resource, Default)]
pub struct CurrentLevelIndex(pub usize);

#[derive(Resource, Default)]
pub struct CurrentLevel(pub Option<LevelConfiguration>);



#[derive(Resource)]
pub struct LevelData(Vec<LevelConfiguration>);



#[derive(Component)]
pub struct GameEntity;

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_state::<GameState>()
      .init_state::<PauseState>()
      .init_resource::<CurrentLevel>()
      .insert_resource(CurrentLevelIndex(0))
      .insert_resource(GameManager{ level_time: Stopwatch::new(), level_test_timer: Timer::from_seconds(0.5, TimerMode::Repeating)})
      .insert_resource(LevelData(LEVEL_DATA.to_vec()))
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
  mut current_level_index:ResMut<CurrentLevelIndex>,
  current_level:ResMut<CurrentLevel>,
  levels:Res<LevelData>,
) {

  current_level_index.0 = 0;
  select_level(0, current_level, levels);
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
  mut current_level_index:ResMut<CurrentLevelIndex>,
  current_level:ResMut<CurrentLevel>,
  levels:Res<LevelData>,
) {
  current_level_index.0 += 1;
  select_level(current_level_index.0, current_level, levels);
}

fn select_level(
  index:usize,
  mut current_level:ResMut<CurrentLevel>,
  levels:Res<LevelData>,
){
  current_level.0 = Some(levels.0[index]);
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
  current_level:Res<CurrentLevelIndex>,
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
