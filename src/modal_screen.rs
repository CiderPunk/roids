use std::f32::consts::PI;

use bevy::{color::palettes::css::WHITE, prelude::*};
use rand::Rng;

use crate::{asset_loader::SceneAssets, game_manager::GameState, input::{InputEventAction, InputEventType, InputTriggerEvent}, movement::Rotation};


const FONT_SIZE_HUGE: f32 = 190.;
const FONT_SIZE_MEDIUM: f32 = 60.;

pub struct ModalScreenPlugin;

impl Plugin for ModalScreenPlugin{
  fn build(&self, app: &mut bevy::app::App) {
    app
      .init_state::<ModalState>()
      .add_systems(OnEnter(GameState::GameOver), show_game_over_screen)
      .add_systems(OnEnter(GameState::Dead), show_dead_screen)
      .add_systems(OnEnter(GameState::StartScreen), show_start_screen)
      .add_systems(OnExit(ModalState::Open), remove_modal_screen)
      .add_systems(Update,  update_modal_screen.run_if(in_state(ModalState::Open)),
      );
  }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum ModalState {
  #[default]
  Closed,
  Open,
}


#[derive(Component)]
struct ModalScreenElement;

fn show_dead_screen(
  mut next_modal_state: ResMut<NextState<ModalState>>,
  mut commands: Commands, 
  scene_assets: Res<SceneAssets>
){
  next_modal_state.set(ModalState::Open);
  info!("show dead screen");
  commands.spawn((
    ModalScreenElement,
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
      Text::new("DEADS"),
      TextFont {
        font: scene_assets.font.clone(),
        font_size: FONT_SIZE_MEDIUM,
        ..default()
      },
    ));
  });
}

fn remove_modal_screen(
  mut commands:Commands,
  query:Query<Entity, With<ModalScreenElement>>,
){
  for entity in query{
    commands.entity(entity).despawn();
  }

}


fn show_game_over_screen(
  mut next_modal_state:ResMut<NextState<ModalState>>,
  mut commands: Commands, 
  scene_assets: Res<SceneAssets>
){
  next_modal_state.set(ModalState::Open);
  info!("show game over screen");
  commands.spawn((
    ModalScreenElement,
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
      Text::new("Game Over"),
      TextFont {
        font: scene_assets.font.clone(),
        font_size: FONT_SIZE_MEDIUM,
        ..default()
      },
    ));
  });
}



fn show_start_screen(
  mut next_modal_state:ResMut<NextState<ModalState>>,
  mut commands: Commands, 
  scene_assets: Res<SceneAssets>
) {
  next_modal_state.set(ModalState::Open);

  info!("show start screen");
  
commands.spawn((
    ModalScreenElement,
    Text::new("'ROIDS"),
    TextFont {
      font: scene_assets.font.clone(),
      font_size: FONT_SIZE_HUGE,
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
    ModalScreenElement,
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


  let mut rng = rand::rng();
  let rotation = Vec3::new(
    rng.random_range(-0.1 .. 0.1),
    rng.random_range(-0.1 .. 0.1),
    rng.random_range(-0.1 ..0.1),
  );
  let rotation2 = Vec3::new(
    rng.random_range(-0.1 .. 0.1),
    rng.random_range(-0.1 .. 0.1),
    rng.random_range(-0.1 ..0.1),
  );
  commands.spawn((
    ModalScreenElement,
    Transform::from_translation(Vec3::new(-16.,130.,1.)).with_scale(Vec3::splat(8.0))
      .with_rotation(Quat::from_euler(EulerRot::XYZ,  rng.random_range(0. .. PI* 2.), rng.random_range(0. .. PI * 2.), rng.random_range(0. .. PI * 2.))),
    SceneRoot(scene_assets.roid1.clone()),
    Rotation(rotation),
  ));

  commands.spawn((
    ModalScreenElement,
    Transform::from_translation(Vec3::new(12.,90.,-22.)).with_scale(Vec3::splat(6.0))
      .with_rotation(Quat::from_euler(EulerRot::XYZ,  rng.random_range(0. .. PI* 2.), rng.random_range(0. .. PI * 2.), rng.random_range(0. .. PI * 2.))),
    SceneRoot(scene_assets.roid1.clone()),
    Rotation(rotation2),
  ));

  commands.spawn((
    ModalScreenElement,
    PointLight {
      color: WHITE.into(),
      intensity: 1700_000_000.0,
      range: 500.,
      //shadows_enabled: true,
      ..default()
    },
    Transform::from_translation(Vec3::new(30., 40., 40.)),

  ));
}


fn update_modal_screen(
  mut ev_input_event: EventReader<InputTriggerEvent>,
  state: Res<State<GameState>>,
  mut next_state: ResMut<NextState<GameState>>,
  mut next_modal_state: ResMut<NextState<ModalState>>,
    
) {
  
  for InputTriggerEvent { action, input_type } in ev_input_event.read() {
    if *input_type == InputEventType::Pressed && *action == InputEventAction::Shoot {
      next_modal_state.set(ModalState::Closed);
      match state.get(){
        GameState::StartScreen => next_state.set(GameState::GameInit),
        GameState::Dead => next_state.set(GameState::Alive),
        GameState::GameOver => next_state.set(GameState::StartScreen),
       _ => ()
      }
    }
  }
}

