mod asset_loader;
mod game_manager;
mod start_screen;
mod camera;
mod input;
mod game;
mod player;
mod movement;

use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowCloseRequested};
use crate::{asset_loader::AssetLoaderPlugin, camera::CameraPlugin, game::GamePlugin, game_manager::{GameManagerPlugin, GameState, GameStateEvent}, input::GameInputPlugin, movement::MovementPlugin, player::PlayerPlugin, start_screen::StartScreenPlugin};



const APP_NAME: &str = "Roids";

fn main() {
  run_game();
}


pub fn run_game(){
  App::new()
    .insert_resource(ClearColor(Color::srgb(0.1,0.1,0.15)))
    .insert_resource(AmbientLight{
      color: Color::WHITE,
      brightness: 750.0,
      ..Default::default()
    })
    .add_plugins(
      DefaultPlugins.set(WindowPlugin{
        primary_window: Some(Window{
          title: APP_NAME.into(),
          name: Some(APP_NAME.into()),
          fit_canvas_to_parent: true,
          visible: true,
          ..default()
        }),
        ..default()
      })
      //prevent meta check issues on itch.io 
      .set(AssetPlugin {
        meta_check: AssetMetaCheck::Never,
        ..default()
      }),
  )
  .add_plugins((
    AssetLoaderPlugin,
    GameManagerPlugin,
    StartScreenPlugin,
    CameraPlugin,
    GameInputPlugin,
    GamePlugin,
    PlayerPlugin,
    MovementPlugin,

  ))
  .add_systems(PreUpdate, check_window)
  .run();
}


fn check_window(
  mut ev_windows_close_reader: EventReader<WindowCloseRequested>,
  mut ev_game_state_writer: EventWriter<GameStateEvent>,
) {
  for _ in ev_windows_close_reader.read() {
    info!("shutting down");
    ev_game_state_writer.write(GameStateEvent::new(GameState::Shutdown));
  }
}