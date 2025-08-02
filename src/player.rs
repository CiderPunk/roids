use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, game_manager::GameState, movement::Velocity};

const START_TRANSLATION: Vec3 = Vec3::new(0.,0.,0.);

pub struct PlayerPlugin;



impl Plugin for PlayerPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::GameInit), create_player)
      .add_systems(Update, update_player.run_if(in_state(GameState::Playing)));
  }
}




#[derive(Component, Default)]
#[require(Transform)]
pub struct Player{

}


fn create_player(mut commands: Commands, scene_assets: Res<SceneAssets>){

commands.spawn((
    Player { ..default() },
    SceneRoot(scene_assets.ship.clone()),
    Transform::from_translation(START_TRANSLATION),
    Velocity(Vec3::new(10.,0.,5.))
  ));

}

fn update_player(mut commands:Commands, query:Query<&GlobalTransform, With<Player>>){

}