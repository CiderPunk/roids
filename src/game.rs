use bevy::prelude::*;

use crate::{
  game_manager::{GameState, GameStateEvent},
  input::{InputEventAction, InputEventType, InputTriggerEvent},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_state::<PauseState>()
      .add_systems(OnEnter(GameState::GameInit), init_game)
      .add_systems(Update, update_game.run_if(in_state(PauseState::Running)));
  }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum PauseState {
  Paused,
  #[default]
  Running,
}

fn init_game(mut ev_game_state: EventWriter<GameStateEvent>) {
  info!("Game initialized");
  ev_game_state.write(GameStateEvent::new(GameState::Playing));
}

fn update_game(
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
