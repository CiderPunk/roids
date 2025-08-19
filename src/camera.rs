use bevy::{math::VectorSpace, prelude::*, window::{WindowResized, WindowResolution}};

use crate::{bounds::Bounds, game_manager::GameState, player::PlayerShip, scheduling::GameSchedule};

pub const CAMERA_LOCATION: Vec3 = Vec3::new(0., 160., 0.);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<CameraBoundsChangeEvent>()
      .add_systems(Startup, spawn_camera)
      .add_systems(Update, (on_resize, update_camera_bounds, follow_player.in_set(GameSchedule::EntityUpdates) ));
  }
}

#[derive(Event)]
pub struct CameraBoundsChangeEvent;


#[derive(Component)]
pub struct GameCamera{ 
  half_bounds:Vec3,
}

fn  follow_player(
  player_query:Query<&GlobalTransform, With<PlayerShip>>,

  mut camera_single:Single<(&GameCamera, &mut Transform)>,
){
  let (game_camera, mut camera_transform) = camera_single.into_inner(); 
  for player_transform in player_query{
    
  }
}


fn spawn_camera(mut commands: Commands) {
  commands.spawn((
    GameCamera{ 
      half_bounds:Vec3::ZERO, 
    },
    Camera3d::default(),
    Camera {
      order: 0,
      ..default()
    },
    Transform::from_translation(CAMERA_LOCATION).looking_at(Vec3::ZERO, Vec3::Z),
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
    //let Ok(top_left) = get_point_on_world_plane(camera, camera_transform,Vec2::new(0.,0.)) else{  panic!("Failed getting top left") };
    let Ok(bottom_right) = get_point_on_world_plane(camera, camera_transform,Vec2::new(window.width(),window.height())) else{  panic!("Failed getting bottom right")};

    let visible_bottom_right = bottom_right - mid;
    let bounds_limit = bounds.half_size - visible_bottom_right;
    game_camera.half_bounds = Vec3::new(bounds_limit.x.min(0.), 0., bounds_limit.z.min(0.));
    info!("Calculated bounds {:} ", bounds.half_size);
  }
}

