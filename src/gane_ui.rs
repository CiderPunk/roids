use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, game_manager::GameState};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::GameInit), init_game_ui)
      .add_systems(OnEnter(GameState::GameOver), remove_game_ui);
  }
}




#[derive(Component)]
struct ScoreDisplay;

#[derive(Component)]
struct GameUi;


fn remove_game_ui(
  mut commands:Commands,
  query:Query<Entity, With<GameUi>>,
){
  for entity in query{
    commands.entity(entity).despawn();
  }
}


fn init_game_ui(
  mut commands:Commands,
  scene_assets:Res<SceneAssets>,
){
  commands.spawn((
    GameUi,
    Node{
      width: Val::Percent(100.),
      align_items: AlignItems::Center,
      ..default()
    }
  ))
  .with_children(|parent|{
    parent.spawn((
      ScoreDisplay,
      Text::new("Score"),
      TextFont {
        font: scene_assets.font.clone(),
        font_size: 34.,
        ..default()
      },
      TextColor::WHITE,
      Node{
        margin: UiRect::all(Val::Px(5.)),
        ..default()
      }
    ));
  });


}