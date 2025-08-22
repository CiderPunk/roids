use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, game_manager::GameState, player::{LifeEvent, Player, ScoreEvent}, scheduling::GameSchedule};


const GAME_UI_FONT_SIZE: f32 = 34.;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::GameInit), init_game_ui)
      .add_systems(OnEnter(GameState::GameOver), remove_game_ui)
      .add_systems(Update, (update_lives, update_score).in_set(GameSchedule::PostEntityUpdates));
  }
}


fn update_score(
  player:Single<&Player>,
  mut text:Single<&mut Text, With<ScoreDisplay>>,
  mut ev_score_reader: EventReader<ScoreEvent>,
){
  if !ev_score_reader.is_empty(){
    text.0 = format!("{:}", player.score);
    ev_score_reader.clear();
  }
}

fn update_lives(
  mut ev_lives_reader: EventReader<LifeEvent>,
  mut query:Query<(&LifeIcon, &mut Visibility)>
){
  for life_event in ev_lives_reader.read(){
    let lives = life_event.lives;
    for (life_icon, mut visibility) in query.iter_mut(){

      if life_icon.0 > lives{
        *visibility = Visibility::Hidden;
      }
      else{
        *visibility = Visibility::Visible;
      }
    }

  }
}

#[derive(Component)]
struct ScoreDisplay;

#[derive(Component)]
struct GameUi;


#[derive(Component)]
struct LifeIcon(u32);

#[derive(Component)]
struct LifeContainer;

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
      margin: UiRect::all(Val::Px(50.)),
      width: Val::Percent(100.),
      flex_direction: FlexDirection::Row,
      ..default()
    }
  ))
  .with_children(|parent|{
    parent.spawn((
      Text::new("Score: "),
      TextFont {
        font: scene_assets.font.clone(),
        font_size:GAME_UI_FONT_SIZE,
        ..default()
      },
      TextColor::WHITE,
     
    ));    
    parent.spawn((
      ScoreDisplay,
      Text::new("0"),
      TextFont {
        font: scene_assets.font.clone(),
        font_size:GAME_UI_FONT_SIZE,
        ..default()
      },
      TextColor::WHITE,
    
    ));
  });

  commands.spawn((
    GameUi,
    Node{
      margin: UiRect::all(Val::Px(50.)),
      width: Val::Percent(100.),
      flex_direction: FlexDirection::Row,
      bottom:Val::Px(0.),
      position_type: PositionType::Absolute,
      ..default()
    }
  ))
  .with_children(|parent|{

    parent.spawn((
      Text::new("Lives: "),
      TextFont {
        font: scene_assets.font.clone(),
        font_size: GAME_UI_FONT_SIZE,
        ..default()
      },
      TextColor::WHITE,
    ));
    
    parent.spawn((
      LifeContainer,
      Node{
        ..default()
      }
    ))
    .with_children(|lives_container|{

      for i in 0..5{
        lives_container.spawn((
          LifeIcon(i),
          ImageNode::new(scene_assets.ship_icon.clone()),
          Visibility::Visible,
          Node{
            width: Val::Px(GAME_UI_FONT_SIZE),
            height: Val::Px(GAME_UI_FONT_SIZE),
            ..default()
          }
        ));
      }
    });
  });




}