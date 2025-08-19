use std::f32::consts::PI;

use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{
  game_manager::{GameEntity, GameState},
  scheduling::GameSchedule,
};

pub struct LightPlugin;

impl Plugin for LightPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::GameInit), spawn_lights)
      .add_systems(Update, rotate_lights.in_set(GameSchedule::EntityUpdates))
      .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 50.0,
        ..Default::default()
      });
  }
}

#[derive(Component)]
pub struct RotateLight {
  distance: f32,
  rotation: f32,
  rate: f32,
}

fn rotate_lights(time: Res<Time>, query: Query<(&mut RotateLight, &mut Transform)>) {
  for (mut rotate, mut transform) in query {
    rotate.rotation += time.delta_secs() * rotate.rate;
    transform.translation = Vec3::new(
      rotate.distance * rotate.rotation.sin(),
      -50.,
      rotate.distance * rotate.rotation.cos(),
    );
  }
}

fn spawn_lights(mut commands: Commands) {
  info!("spawning lights!");
  commands.spawn((
    GameEntity,
    PointLight {
      color: WHITE.into(),
      intensity: 1_700_000_000.0,
      range: 500.,
      //shadows_enabled: true,
      ..default()
    },
    Transform::from_translation(Vec3::new(100., -50., 100.)),
    RotateLight {
      distance: 200.,
      rotation: 0.,
      rate: 0.2,
    },
  ));

  commands.spawn((
    GameEntity,
    PointLight {
      color: WHITE.into(),
      intensity: 400_000_000.0,
      range: 500.,
      //shadows_enabled: true,
      ..default()
    },
    Transform::from_translation(Vec3::new(100., -50., 100.)),
    RotateLight {
      distance: 200.,
      rotation: PI,
      rate: 0.33,
    },
  ));
}
