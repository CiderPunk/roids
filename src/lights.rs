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
        brightness: 500.0,
        ..Default::default()
      });
  }
}

#[derive(Component)]
pub struct RotateLight {
  verical_offset:f32,
  distance: f32,
  rotation: f32,
  rate: f32,
}

fn rotate_lights(time: Res<Time>, query: Query<(&mut RotateLight, &mut Transform)>) {
  for (mut rotate, mut transform) in query {
    rotate.rotation += time.delta_secs() * rotate.rate;
    transform.translation = Vec3::new(
      rotate.distance * rotate.rotation.sin(),
      rotate.verical_offset,
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
      intensity: 11_700_000_000.0,
      range: 800.,
      //shadows_enabled: true,
      ..default()
    },
    Transform::from_translation(Vec3::new(100., -50., 100.)),
    RotateLight {
      verical_offset:-350.,
      distance: 200.,
      rotation: 0.,
      rate: 0.2,
    },
  ));

  commands.spawn((
    GameEntity,
    PointLight {
      color: WHITE.into(),
      intensity: 5_400_000_000.0,
      range: 800.,
      //shadows_enabled: true,
      ..default()
    },
    Transform::from_translation(Vec3::new(100., 250., 100.)),
    RotateLight {
      verical_offset:-150.,
      distance: 200.,
      rotation: PI,
      rate: 0.33,
    },
  ));
}
