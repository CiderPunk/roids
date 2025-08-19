use bevy::prelude::*;

use crate::{
  asset_loader::SceneAssets,
  bounds::BoundsWarp,
  bullet::ShootEvent,
  collision::Collider,
  game_manager::{GameEntity, GameState},
  health::Health,
  input::{InputEventAction, InputEventType, InputMovementEvent, InputTriggerEvent},
  movement::{Acceleration, Rotation, Velocity},
  scheduling::GameSchedule,
};

const PLAYER_START_TRANSLATION: Vec3 = Vec3::new(0., 0., 0.);
const PLAYER_ROTATION_SPEED: f32 = -5.0;
const ACCELERATION_MULTIPIER: f32 = 60.0;
const PLAYER_DAMPING: f32 = 3.;
const PLAYER_MAX_SPEED: f32 = 30.;
const PLAYER_SHOOT_DELAY: f32 = 0.2;
const PLAYER_BULLET_FORWARD_OFFSET: f32 = 2.5;
const PLAYER_BULLET_VELOCITY: f32 = 60.;
const PLAYER_BULLET_DAMAGE: f32 = -10.;
const PLAYER_BULLET_SCALE: f32 = 0.5;
const PLAYER_COLLLISION_RADIUS: f32 = 1.3;
const PLAYER_START_LIVES: u32 = 3;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::GameInit), create_player)
      .add_systems(OnEnter(GameState::Alive), create_ship)
      .add_systems(
        Update,
        (
          (update_player_movement, update_player_action, player_shoot)
            .in_set(GameSchedule::EntityUpdates),
          check_player_health.in_set(GameSchedule::PreDespawnEntities),
        ),
      );
  }
}

#[derive(Component, Default)]
#[require(Transform, Velocity, Acceleration, Rotation)]
pub struct PlayerShip {
  shoot: bool,
  shield: bool,
  next_shoot_time: f32,
}

#[derive(Component, Default)]
pub struct Player {
  lives: u32,
  score: u32,
}

fn check_player_health(
  query: Query<&Health, With<PlayerShip>>,
  player: Single<&Player>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  for health in query {
    if health.value <= 0. {
      info!("Player dead");
      if player.lives > 0 {
        next_state.set(GameState::Dead);
      } else {
        next_state.set(GameState::GameOver);
      }
    }
  }
}

fn create_player(query: Query<Entity, With<Player>>, mut commands: Commands) {
  //delete old player
  for entity in query {
    commands.entity(entity).despawn();
  }

  info!("Create player");
  commands.spawn(Player {
    lives: PLAYER_START_LIVES,
    score: 0,
  });
}

fn create_ship(
  mut commands: Commands,
  scene_assets: Res<SceneAssets>,
  mut player: Single<&mut Player>,
) {
  player.lives -= 1;
  info!("Create ship");
  commands.spawn((
    GameEntity,
    PlayerShip { ..default() },
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
    Collider {
      radius: PLAYER_COLLLISION_RADIUS,
      damage: 0.,
    },
    Health {
      value: 10.,
      max: 10.,
      last_hurt_by: None,
    },
  ));

  /*.with_child((

      SpotLight {
        intensity: 500_000_000.0, // lumens
        color: Color::WHITE,
        shadows_enabled: false,
        inner_angle: PI /8. * 0.85,
        outer_angle: PI / 8.,
        range:50.,
        ..default()
      },
      Transform::from_translation(Vec3::ZERO),
    ));
  */
}

fn update_player_movement(
  //mut commands:Commands,
  mut ev_input_movement_event: EventReader<InputMovementEvent>,
  ship: Single<(&GlobalTransform, &mut Acceleration, &mut Rotation), With<PlayerShip>>,
) {
  let (transform, mut acceleration, mut rotation) = ship.into_inner();
  for InputMovementEvent { direction } in ev_input_movement_event.read() {
    rotation.y = direction.x * PLAYER_ROTATION_SPEED;
    acceleration.acceleration = transform.forward() * ACCELERATION_MULTIPIER * direction.y.max(0.);
  }
}

fn update_player_action(
  mut ev_input_trigger_event: EventReader<InputTriggerEvent>,
  ship: Single<&mut PlayerShip>,
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
  query: Single<(Entity, &mut PlayerShip, &GlobalTransform, &Velocity)>,
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
