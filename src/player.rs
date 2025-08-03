use bevy::{math::VectorSpace, prelude::*};

use crate::{asset_loader::SceneAssets, game::PauseState, game_manager::GameState, input::{InputMovementEvent, InputTriggerEvent}, movement::{Acceleration, Rotation, Velocity}};

const START_TRANSLATION: Vec3 = Vec3::new(0.,0.,0.);

pub struct PlayerPlugin;



impl Plugin for PlayerPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::GameInit), create_player)
      .add_systems(Update, update_player_movement.run_if(in_state(PauseState::Running)));
  }
}




#[derive(Component, Default)]
#[require(Transform, Velocity, Acceleration, Rotation)]
pub struct Player{

}


fn create_player(mut commands: Commands, scene_assets: Res<SceneAssets>){

commands.spawn((
    Player { ..default() },
    SceneRoot(scene_assets.ship.clone()),
    Transform::from_translation(START_TRANSLATION),
    Velocity(Vec3::new(0.,0.,1.)),
    Acceleration{
        acceleration: Vec3::ZERO,
        max_speed: 20.,
        damping: 0.,
    }
  ));

}

fn update_player_movement(
  //mut commands:Commands, 
  mut ev_input_movement_event:EventReader<InputMovementEvent>,
  mut player:Single<(&GlobalTransform, &mut Acceleration, &mut Rotation),With<Player>>
){

  let (global_trasform, mut acceleration, mut rotation) = player.into_inner();
  for InputMovementEvent{ direction } in ev_input_movement_event.read(){
    rotation.y = direction.x * 10.;


  }


}