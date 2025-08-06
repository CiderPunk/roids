use bevy::prelude::*;

use crate::asset_loader::{AssetState, SceneAssets};


#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum GameState{
  #[default]
  Startup,
  StartScreen,
  GameInit,
  Playing,
  Shutdown,
}

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(AssetState::Ready), start_screen)
      .init_state::<GameState>()
      .add_event::<GameStateEvent>()
      .add_systems(Update, update_game_state);
  }
}

fn start_screen(mut next_state: ResMut<NextState<GameState>>){
  info!("Switching to start screen");
  next_state.set(GameState::GameInit);
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