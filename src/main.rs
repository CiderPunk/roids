mod asset_loader;
mod bounds;
mod bullet;
mod camera;
mod collision;
mod effect_sprite;
mod game_manager;
mod health;
mod input;
mod lights;
mod modal_screen;
mod movement;
mod pause_screen;
mod player;
mod roid;
mod scheduling;
mod starfield;
mod game_ui;

use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowCloseRequested};

use crate::{
  asset_loader::AssetLoaderPlugin, bounds::BoundsPlugin, bullet::BulletPlugin, camera::CameraPlugin, collision::CollisionPlugin, effect_sprite::EffectSpritePlugin, game_manager::{GameManagerPlugin, GameState}, game_ui::GameUiPlugin, health::HealthPlugin, input::GameInputPlugin, lights::LightPlugin, modal_screen::ModalScreenPlugin, movement::MovementPlugin, pause_screen::PauseScreenPlugin, player::PlayerPlugin, roid::RoidPlugin, scheduling::SchedulingPlugin, starfield::StarfieldPlugin
};


const APP_NAME: &str = "Roids";

fn main() {
  run_game();
}

pub fn run_game() {
  App::new()
    .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)))
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: APP_NAME.into(),
            name: Some(APP_NAME.into()),
            fit_canvas_to_parent: true,
            visible: true,
            ..default()
          }),
          ..default()
        })
        //prevent meta check issues on itch.io
        .set(AssetPlugin {
          meta_check: AssetMetaCheck::Never,
          watch_for_changes_override: Some(true),
          ..default()
        }),
    )
    .add_plugins((
      AssetLoaderPlugin,
      GameManagerPlugin,
      LightPlugin,
      CameraPlugin,
      GameInputPlugin,
      PlayerPlugin,
      MovementPlugin,
      BulletPlugin,
      BoundsPlugin,
      RoidPlugin,
      CollisionPlugin,
      HealthPlugin,
      PauseScreenPlugin,
    ))
    .add_plugins((
      ModalScreenPlugin,
      SchedulingPlugin,
      StarfieldPlugin,
      EffectSpritePlugin,
      GameUiPlugin,
    ))
    .add_systems(PreUpdate, shutdown_detect)
    //.add_systems(PreUpdate, test_sphere)
    .run();
}

fn _test_sphere(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let material = materials.add(StandardMaterial {
    base_color: Color::WHITE,
    ..Default::default()
  });
  commands.spawn((
    Mesh3d(meshes.add(Sphere::new(10.).mesh().uv(32, 18))),
    MeshMaterial3d(material),
    Transform::from_translation(Vec3::ZERO),
  ));
}

fn shutdown_detect(
  mut ev_windows_close_reader: EventReader<WindowCloseRequested>,
  mut next_state:ResMut<NextState<GameState>>,
) {
  for _ in ev_windows_close_reader.read() {
    info!("shutting down");
    next_state.set(GameState::Shutdown);

  }
}
