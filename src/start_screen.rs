use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, game_manager::{GameState, GameStateEvent}, input::{InputEventAction, InputEventType, InputTriggerEvent}};

pub struct StartScreenPlugin;

impl Plugin for StartScreenPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::StartScreen), init_start_screen)
      .add_systems(OnExit(GameState::StartScreen), cleanup_start_screen)
      .add_systems(Update, (check_start_game).run_if(in_state(GameState::StartScreen)));
    }
}

#[derive(Component)]
pub struct StartScreenComponent;


fn check_start_game(mut ev_shoot_event:EventReader<InputTriggerEvent>, mut ev_game_state:EventWriter<GameStateEvent>){
  for InputTriggerEvent { action, input_type } in ev_shoot_event.read() {
    if *action == InputEventAction::Shoot && *input_type == InputEventType::Pressed{
      info!("Starting game");
      ev_game_state.write(GameStateEvent::new(GameState::GameInit));
    }
  }  
}

fn cleanup_start_screen(mut commands:Commands, query:Query<Entity, With<StartScreenComponent>>){
  info!("despawning start screen");
  for entity in query{
    commands.entity(entity).despawn();
  }
}

fn init_start_screen(
  mut commands: Commands,
  scene_assets:Res<SceneAssets>,
){
  info!("init start screen");
  commands.spawn((
    StartScreenComponent,
    Text::new("ROIDS"),
    TextFont {
      font: scene_assets.font.clone(),
      font_size: 190.,
      ..default()
    },
    TextColor(Color::srgb(0.9, 0.9, 0.9)),
    Node {
      position_type: PositionType::Absolute,
      top: Val::Px(0.0),
      left: Val::Px(10.0),
      ..default()
    },
  ));

 commands.spawn((
    StartScreenComponent,
    Text::new("Press FIRE to start"),
    TextFont {
      font: scene_assets.font.clone(),
      font_size: 20.,
      ..default()
    },
    TextColor(Color::srgb(0.9, 0.9, 0.9)),
    Node {
      position_type: PositionType::Absolute,
      bottom: Val::Px(10.0),
      right: Val::Px(10.0),
      ..default()
    },
  ));



}