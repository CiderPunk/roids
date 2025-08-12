use bevy::prelude::*;

use crate::{
  asset_loader::SceneAssets, game_manager::PauseState, input::{InputEventAction, InputEventType, InputTriggerEvent}
};
pub struct PauseScreenPlugin;
impl Plugin for PauseScreenPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(PauseState::Paused), show_pause_screen)
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

fn show_pause_screen(
  mut commands: Commands, 
  scene_assets: Res<SceneAssets>
){
  info!("show pause screen");
  commands.spawn((
    PauseScreenElement,
    Node {
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      ..default()
    },
  ))
  .with_children(|parent|{
    parent.spawn((
      Text::new("PAUSED"),
      TextFont {
        font: scene_assets.font.clone(),
        font_size: 32.,
        ..default()
      },
    ));
  });
}


fn remove_pause_screen(mut commands: Commands, query: Query<Entity, With<PauseScreenElement>>) {
  info!("despawning pause screen");
  for entity in query {
    commands.entity(entity).despawn();
  }
}