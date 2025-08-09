use bevy::prelude::*;

use crate::{
  game::PauseState,
  input::{InputEventAction, InputEventType, InputTriggerEvent},
};
pub struct PauseScreenPlugin;
impl Plugin for PauseScreenPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(PauseState::Paused), init_pause_screen)
      .add_systems(OnExit(PauseState::Paused), remove_pause_screen)
      .add_systems(
        Update,
        update_pause_screen.run_if(in_state(PauseState::Paused)),
      );
  }
}

#[derive(Component)]
struct PauseScreenElement;

fn update_pause_screen(
  mut ev_input_event: EventReader<InputTriggerEvent>,
  mut next_state: ResMut<NextState<PauseState>>,
) {
  for InputTriggerEvent { action, input_type } in ev_input_event.read() {
    if *input_type == InputEventType::Pressed && *action == InputEventAction::Shoot {
      info!("Resuming game");
      next_state.set(PauseState::Running);
    }
  }
}

fn init_pause_screen(mut ev_input_event: EventReader<InputTriggerEvent>) {
  ev_input_event.clear();
  info!("entered pause screen")
}
fn remove_pause_screen() {
  info!("exited pause screen")
}
