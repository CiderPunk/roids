use bevy::prelude::*;

use crate::{
  asset_loader::SceneAssets, bounds::BoundsWarp, bullet::ShootEvent, collision::Collider, game::PauseState, game_manager::GameState, health::Health, input::{InputEventAction, InputEventType, InputMovementEvent, InputTriggerEvent}, movement::{Acceleration, Rotation, Velocity}
};

const PLAYER_START_TRANSLATION: Vec3 = Vec3::new(0., 0., 0.);
const PLAYER_ROTATION_SPEED: f32 = -5.0;
const ACCELERATION_MULTIPIER: f32 = 60.0;
const PLAYER_DAMPING: f32 = 3.;
const PLAYER_MAX_SPEED: f32 = 30.;
const PLAYER_SHOOT_DELAY: f32 = 0.5;
const PLAYER_BULLET_FORWARD_OFFSET: f32 = 2.5;
const PLAYER_BULLET_VELOCITY: f32 = 60.;
const PLAYER_BULLET_DAMAGE: f32 = -10.;
const PLAYER_BULLET_SCALE: f32 = 0.5;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::GameInit), create_player)
      .add_systems(
        Update,
        (update_player_movement, update_player_action, player_shoot)
          .chain()
          .run_if(in_state(PauseState::Running)),
      );
  }
}

#[derive(Component, Default)]
#[require(Transform, Velocity, Acceleration, Rotation)]
pub struct Player {
  shoot: bool,
  shield: bool,
  next_shoot_time: f32,
}

fn create_player(mut commands: Commands, scene_assets: Res<SceneAssets>) {
  commands.spawn((
    Player { ..default() },
    SceneRoot(scene_assets.ship.clone()),
    Transform::from_translation(PLAYER_START_TRANSLATION),
    Velocity(Vec3::new(0., 0., 1.)),
    Acceleration {
      acceleration: Vec3::ZERO,
      max_speed: PLAYER_MAX_SPEED,
      damping: PLAYER_DAMPING,
      min_speed: 2.0,
    },
    BoundsWarp(true),
    Collider{ radius: 5., damage: 0. },
    Health{ value: 10., max: 10., last_hurt_by: None },
  ));
}

fn update_player_movement(
  //mut commands:Commands,
  mut ev_input_movement_event: EventReader<InputMovementEvent>,
  ship: Single<(&GlobalTransform, &mut Acceleration, &mut Rotation), With<Player>>,
) {
  let (transform, mut acceleration, mut rotation) = ship.into_inner();
  for InputMovementEvent { direction } in ev_input_movement_event.read() {
    rotation.y = direction.x * PLAYER_ROTATION_SPEED;
    acceleration.acceleration = transform.forward() * ACCELERATION_MULTIPIER * direction.y.max(0.);
  }
}

fn update_player_action(
  mut ev_input_trigger_event: EventReader<InputTriggerEvent>,
  ship: Single<&mut Player>,
) {
  let mut player = ship.into_inner();
  for InputTriggerEvent { action, input_type } in ev_input_trigger_event.read() {
    if *action == InputEventAction::Shoot {
      player.shoot = *input_type == InputEventType::Pressed;
    }
    if *action == InputEventAction::Shield {
      player.shield = *input_type == InputEventType::Pressed;
    }
  }
}

fn player_shoot(
  query: Single<(Entity, &mut Player, &GlobalTransform, &Velocity)>,
  time: Res<Time>,
  mut ev_shoot_event: EventWriter<ShootEvent>,
) {
  let (player_entity, mut player, transform, velocity) = query.into_inner();

  player.next_shoot_time -= time.delta_secs();

  if player.next_shoot_time < 0. {
    if player.shoot {
      let forward = transform.forward();
      ev_shoot_event.write(ShootEvent::new(
        true,
        transform.translation() + (forward * PLAYER_BULLET_FORWARD_OFFSET),
        (forward * PLAYER_BULLET_VELOCITY) + velocity.0,
        PLAYER_BULLET_DAMAGE,
        PLAYER_BULLET_SCALE,
        player_entity,
      ));
      player.next_shoot_time += PLAYER_SHOOT_DELAY;
    } else {
      player.next_shoot_time = 0.;
    }
  }
}
