use bevy::{math::{bounding::Aabb2d, VectorSpace}, prelude::*, window::{WindowResized, WindowResolution}};

use crate::{bounds::Bounds, game_manager::GameState, player::PlayerShip, scheduling::GameSchedule};

pub const CAMERA_START_LOCATION: Vec3 = Vec3::new(0., 160., 0.);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<CameraBoundsChangeEvent>()
      .add_systems(Startup, spawn_camera)
      .add_systems(Update, (on_resize, update_camera_bounds, follow_player.in_set(GameSchedule::EntityUpdates) ))
      .add_systems(OnEnter(GameState::StartScreen), reset_camera);
  }
}

#[derive(Event)]
pub struct CameraBoundsChangeEvent;


#[derive(Component)]
pub struct GameCamera{ 
  limits:Aabb2d,
}

fn reset_camera(
  mut camera_transform:Single<&mut Transform, With<GameCamera>>,
){
  camera_transform.translation = CAMERA_START_LOCATION;

}

fn  follow_player(
  player_query:Query<&GlobalTransform, With<PlayerShip>>,
  mut camera_single:Single<(&GameCamera, &mut Transform)>,
){
  let (game_camera, mut camera_transform) = camera_single.into_inner(); 
  for player_transform in player_query{
    let player_pos = Vec2::new(player_transform.translation().x, player_transform.translation().z);
    let camera_pos = game_camera.limits.closest_point(player_pos);
    camera_transform.translation.x = camera_pos.x;
    camera_transform.translation.z = camera_pos.y;
  }
}


fn spawn_camera(mut commands: Commands) {
  commands.spawn((
    GameCamera{ 
      limits: Aabb2d::new(Vec2::ZERO, Vec2::ZERO), 
    },
    Camera3d::default(),
    Camera {
      order: 0,
      ..default()
    },
    Transform::from_translation(CAMERA_START_LOCATION).looking_at(Vec3::ZERO, Vec3::Z),
  ));
}

fn get_point_on_world_plane(camera: &Camera, camera_transform:&GlobalTransform, screen_coord:Vec2) -> Result<Vec3, BevyError>{
  let Ok(ray) = camera.viewport_to_world(camera_transform, screen_coord) else{  panic!("Cannot get ray for {screen_coord}"); };
  let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)) else{  panic!("Cannot get intersection for {screen_coord}" ); }; 
  Ok(ray.get_point(distance))
}



fn on_resize(
  mut ev_resize_reader: EventReader<WindowResized>,
  mut ev_bound_change_writer: EventWriter<CameraBoundsChangeEvent>,
){
  if !ev_resize_reader.is_empty(){
    ev_resize_reader.clear();
    info!("Resize event");
    ev_bound_change_writer.write(CameraBoundsChangeEvent);
  }
}

fn update_camera_bounds(
  mut ev_bounds_reader:EventReader<CameraBoundsChangeEvent>,
  camera_query:Single<(&Camera, &GlobalTransform, &mut GameCamera)>,
  bounds:Single<&Bounds>,
  window:Single<&Window>,
){
  if !ev_bounds_reader.is_empty(){
    ev_bounds_reader.clear();
    let (camera, camera_transform, mut game_camera) = camera_query.into_inner();
    info!("Recalculating camera limits");
    let Ok(mid) = get_point_on_world_plane(camera, camera_transform, Vec2::new(window.width() * 0.5, window.height() * 0.5)) else{ panic!("Failed getting mid") };
    let Ok(top_left) = get_point_on_world_plane(camera, camera_transform,Vec2::new(0.,0.)) else{  panic!("Failed getting top left") };
    let Ok(bottom_right) = get_point_on_world_plane(camera, camera_transform,Vec2::new(window.width(),window.height())) else{  panic!("Failed getting bottom right")};

    let visible_top_left = top_left - mid;
    let visible_bottom_right = bottom_right - mid;


    info!("visible tl:{:} br:{:}", visible_top_left, visible_bottom_right);


    let bounds_limit_tl = (bounds.half_size - visible_top_left).max(Vec3::new(0.,0., 0.));
    let bounds_limit_br = (-bounds.half_size - visible_bottom_right).min(Vec3::new(0.,0., 0.));

    game_camera.limits = Aabb2d { max: Vec2::new(bounds_limit_tl.x, bounds_limit_tl.z), min: Vec2::new(bounds_limit_br.x, bounds_limit_br.z) };
    

    info!("limits updated tl:{:} br:{:}", bounds_limit_tl, bounds_limit_br);
  }
}

