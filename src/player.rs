use bevy::prelude::*;
use crate::{
  asset_loader::SceneAssets, bounds::{BoundsWarp, InBounds}, bullet::ShootMessage, collision::{Collider, CollisionFlags}, effect_sprite::{EffectSpriteMessage, EffectSpriteType}, game_manager::{GameEntity, GameState}, health::Health, input::{InputEventAction, InputEventType, InputMovementMessage, InputTriggerMessage}, movement::{Acceleration, Damping, Rotation, Velocity}, scheduling::GameSchedule, shaders::ShaderMaterials
};

const PLAYER_START_TRANSLATION: Vec3 = Vec3::new(0., 0., 0.);
const PLAYER_ROTATION_SPEED: f32 = -5.0;
const ACCELERATION_MULTIPIER: f32 = 60.0;
const PLAYER_DAMPING: f32 = 0.02;
const PLAYER_MIN_SPEED: f32 = 0.2;
const PLAYER_MAX_SPEED: f32 = 30.;
const PLAYER_SHOOT_DELAY: f32 = 0.2;
const PLAYER_BULLET_FORWARD_OFFSET: f32 = 2.5;
const PLAYER_BULLET_VELOCITY: f32 = 60.;
const PLAYER_BULLET_DAMAGE: f32 = -10.;
const PLAYER_BULLET_SCALE: f32 = 0.5;
const PLAYER_COLLLISION_RADIUS: f32 = 1.3;
const PLAYER_START_LIVES: u32 = 3;
const PLAYER_SPAWN_INVINCIBLE_TIME: f32 = 3.;
const PLAYER_SHIELD_SIZE: f32 = 3.5;
const PLAYER_SHIELD_REPULSE_FORCE: f32 = 300.;
const PLAYER_BULLET_TIME_TO_LIVE: f32 = 3.0;
    

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app

    .add_message::<ScoreMessage>()
    .add_message::<LifeEvent>()
    .add_systems(OnEnter(GameState::GameInit), create_player)
    .add_systems(OnEnter(GameState::Alive), create_ship)
    .add_systems(
      Update,
      (
        (update_player_movement, update_player_action, player_shoot).in_set(GameSchedule::ActionUserInput),
        (update_score, update_invulnerable, create_shield, update_shield, animate_flame).in_set(GameSchedule::EntityUpdates),
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
  pub lives: u32,
  pub score: u32,
}

#[derive(Message)]
pub struct ScoreMessage{
  score:u32,
}

impl ScoreMessage{
  pub fn new(score:u32)->Self{
    Self{ score }
  }
}

#[derive(Message)]
pub struct LifeEvent{
  pub lives:u32,
}

impl LifeEvent{
  pub fn new(lives:u32)->Self{
    Self{ lives }
  }
}


#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Invulnerable{
  pub duration:Timer,
}


#[derive(Component)]
pub struct Shield{
  owner:Entity,
  pub repulse_force:f32,
}

#[derive(Component)]
struct FlameMarker;


fn update_score(
  mut player: Single<&mut Player>,
  mut msg_score_reader:MessageReader<ScoreMessage>
){
  for score_event in msg_score_reader.read(){
    player.score += score_event.score;
  }
}

fn check_player_health(
  query: Query<(&Health, &Velocity, &GlobalTransform),With<PlayerShip>>,
  player: Single<&Player>,
  mut next_state: ResMut<NextState<GameState>>,
  mut ev_effect_writer:MessageWriter<EffectSpriteMessage>,
) {
  for (health, velocity, transform) in query {
    if health.value <= 0. {
      ev_effect_writer.write(EffectSpriteMessage::new(transform.translation(), 16., velocity.0, EffectSpriteType::Splosion));
      info!("Player dead");
      if player.lives > 0 {
        next_state.set(GameState::Dead);
      } else {
        next_state.set(GameState::GameOver);
      }
    }
  }
}

fn update_invulnerable(
  mut commands:Commands,
  query:Query<(Entity, &mut Invulnerable)>,
  shield_query:Query<(Entity, &Shield)>,
  time:Res<Time>,
){
  for (entity, mut invulnerable) in query{
    invulnerable.duration.tick(time.delta());
    if invulnerable.duration.just_finished(){

      commands.entity(entity).remove::<Invulnerable>();
      for (shield_entity, shield) in shield_query{
        if shield.owner == entity{
          commands.entity(shield_entity).despawn();
        }
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
  query:Query<&PlayerShip>,
  mut commands: Commands,
  scene_assets: Res<SceneAssets>,
  mut player: Single<&mut Player>,
  mut lives_writer: MessageWriter<LifeEvent>,
) {


  if !query.is_empty(){
    return;
  }
  player.lives -= 1;
  lives_writer.write(LifeEvent::new(player.lives));
  info!("Create ship");

commands.spawn((
    Invulnerable{ duration: Timer::from_seconds(PLAYER_SPAWN_INVINCIBLE_TIME, TimerMode::Once) } ,
    GameEntity,
    PlayerShip { ..default() },
    SceneRoot(scene_assets.ship.clone()),
    Transform::from_translation(PLAYER_START_TRANSLATION),
    Velocity(Vec3::new(0., 0., 1.)),
    Acceleration {
      acceleration: Vec3::ZERO,
      max_speed: PLAYER_MAX_SPEED,
    },
    Damping{
        amount: PLAYER_DAMPING,
        min_speed: PLAYER_MIN_SPEED,
    },
    BoundsWarp::default(),
    InBounds,
    Collider {
      collison_group: CollisionFlags::Player,
      collision_mask: CollisionFlags::Enemy | CollisionFlags::Asteroid,
      owner: None,
      radius: PLAYER_COLLLISION_RADIUS,
      damage: -10.,
    },
    Health {
      value: 10.,
      max: 10.,
      last_hurt_by: None,
    },
  ))
  .with_child((
    FlameMarker,
    SceneRoot(scene_assets.flame.clone()),
    Visibility::Hidden,
  ));
}

fn create_shield(
  mut commands: Commands,
  query: Query<Entity, Added<Invulnerable>>,
  scene_assets: Res<SceneAssets>,
  shaders: Res<ShaderMaterials>,
) {
  for entity in query.iter() {
    commands.spawn((
      GameEntity,
      Shield{ owner: entity, repulse_force: PLAYER_SHIELD_REPULSE_FORCE},
      Mesh3d(scene_assets.ship_shield.clone()),
      MeshMaterial3d(shaders.shield.clone()),
      Transform::from_scale(Vec3::splat(PLAYER_SHIELD_SIZE)),
      Collider{ 
        collison_group: CollisionFlags::Player,
        collision_mask: CollisionFlags::Enemy | CollisionFlags::Asteroid,
        owner:Some(entity),
        radius: PLAYER_SHIELD_SIZE,
        damage: 0.
      },
    ));
  }
}

fn update_shield(
  shield_query:Query<(&Shield, &mut Transform)>,
  owner_query:Query<&GlobalTransform>,
){
  for (shield, mut transform) in shield_query{
    if let Ok(owner_transform) = owner_query.get(shield.owner){
      transform.translation = owner_transform.translation();
    }
  }
}



fn update_player_movement(
  //mut commands:Commands,
  mut input_movement_reader: MessageReader<InputMovementMessage>,
  ship: Single<(&GlobalTransform, &mut Acceleration, &mut Rotation), With<PlayerShip>>,
  flame_visibility: Single<&mut Visibility, With<FlameMarker>>,
) {
  let (transform, mut acceleration, mut rotation) = ship.into_inner();
  let mut flame = flame_visibility.into_inner();
  for InputMovementMessage { direction } in input_movement_reader.read() {
    rotation.y = direction.x * PLAYER_ROTATION_SPEED;
    acceleration.acceleration = transform.forward() * ACCELERATION_MULTIPIER * direction.y.max(0.);
    if direction.y > 0.{
      *flame = Visibility::Visible;
    }
    else{
      *flame = Visibility::Hidden;
    }
  }

}

fn animate_flame(
  flame: Single<&mut Transform, With<FlameMarker>>,
  time:Res<Time>,
){
  let mut transform = flame.into_inner();
  transform.scale = Vec3::splat(0.5 + (time.elapsed_secs() * 20.).sin().abs() * 0.5);
}


fn update_player_action(
  mut input_trigger_reader: MessageReader<InputTriggerMessage>,
  ship: Single<&mut PlayerShip>,
) {
  let mut player = ship.into_inner();
  for InputTriggerMessage { action, input_type } in input_trigger_reader.read() {
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
  mut ev_shoot_event: MessageWriter<ShootMessage>,
) {
  let (player_entity, mut player, transform, velocity) = query.into_inner();

  player.next_shoot_time -= time.delta_secs();

  if player.next_shoot_time < 0. {
    if player.shoot {
      let forward = transform.forward();
      ev_shoot_event.write(ShootMessage::new(
        true,
        transform.translation() + (forward * PLAYER_BULLET_FORWARD_OFFSET),
        (forward * PLAYER_BULLET_VELOCITY) + velocity.0,
        PLAYER_BULLET_DAMAGE,
        PLAYER_BULLET_SCALE,
        player_entity,
        PLAYER_BULLET_TIME_TO_LIVE,
      ));
      player.next_shoot_time += PLAYER_SHOOT_DELAY;
    } else {
      player.next_shoot_time = 0.;
    }
  }
}
