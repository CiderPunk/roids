use bevy::prelude::*;

use crate::{asset_loader::AssetState, input::{InputEventAction, InputEventType, InputTriggerEvent}, player::Player};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum GameState {
  #[default]
  Startup,
  StartScreen,
  GameInit,
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
pub struct GameEntity;


pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_state::<GameState>()
      .init_state::<PauseState>()
      .add_event::<GameStateEvent>()
      .add_systems(OnEnter(AssetState::Ready), start_screen)
      .add_systems(OnEnter(GameState::GameInit), init_game)
      .add_systems(OnExit(GameState::GameOver), clean_game)
      .add_systems(Update, update_game_state)
      .add_systems(Update, check_for_pause.run_if(in_state(PauseState::Running)));
  }
}


fn clean_game(
  mut commands:Commands,
  query:Query<Entity, With<GameEntity>>
){
  for entity in query{
    commands.entity(entity).try_despawn();
  }

}

fn init_game(mut next_state: ResMut<NextState<GameState>>) {
  info!("Game initialized");
  next_state.set(GameState::Alive);
}


fn start_screen(mut next_state: ResMut<NextState<GameState>>) {
  info!("Switching to start screen");
  next_state.set(GameState::StartScreen);
}

#[derive(Event)]
pub struct GameStateEvent {
  state: GameState,
}

impl GameStateEvent {
  pub fn new(state: GameState) -> Self {
    Self { state }
  }
}

fn update_game_state(
  mut ev_game_state: EventReader<GameStateEvent>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  for &GameStateEvent { state } in ev_game_state.read() {
    info!("Switching game state {:?}", state);
    next_state.set(state);
  }
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
