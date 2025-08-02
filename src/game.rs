use bevy::prelude::*;

use crate::game_manager::{GameState, GameStateEvent};

pub struct GamePlugin;

impl Plugin for GamePlugin{
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(GameState::GameInit), init_game);
  }
}

fn init_game(mut commands:Commands,  mut ev_game_state:EventWriter<GameStateEvent>){
  info!("Game initialized");
  ev_game_state.write(GameStateEvent::new(GameState::Playing));
}