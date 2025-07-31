use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, game_manager::GameState};

pub struct StartScreenPlugin;

impl Plugin for StartScreenPlugin{
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(GameState::StartScreen), init_start_screen);
  }
}

#[derive(Component)]
pub struct StartScreenComponent;

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
      font_size: 20.,
      ..default()
    },
    TextColor(Color::srgb(0.9, 0.9, 0.9)),
    Node {
      position_type: PositionType::Absolute,
      
      top: Val::Px(12.0),
      left: Val::Percent(50.0),
      ..default()
    },
  ));
}