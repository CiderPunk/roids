use std::f32::consts::PI;

use bevy::{
  asset::RenderAssetUsages, mesh::{Indices, MeshTag, PrimitiveTopology}, platform::collections::HashMap, prelude::*, render::render_resource::{AsBindGroup, ShaderType}, shader::ShaderRef
};

use rand::Rng;

use crate::{
  asset_loader::AssetsLoading,
  game_manager::GameEntity,
  movement::Velocity,
};
pub struct EffectSpritePlugin;
const EFFECT_SPRITE_SHADER_PATH: &str = "shaders/animated_uv.wgsl";

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectSpriteType {
  Splosion,
  Ricochet,
}

struct EffectSpriteDef {
  effect_type: EffectSpriteType,
  texture_path: &'static str,
  frame_count: u32,
  horizontal_frames: u32,
  vertical_frames: u32,
  frame_rate: f32,
}

//Our effect sprite params
const EFFECTS: [EffectSpriteDef; 3] = [
  EffectSpriteDef {
    effect_type: EffectSpriteType::Splosion,
    texture_path: "sprites/splosion.png",
    frame_count: 17,
    horizontal_frames: 4,
    vertical_frames: 4,
    frame_rate: 15.,
  },  
  EffectSpriteDef {
    effect_type: EffectSpriteType::Splosion,
    texture_path: "sprites/splosion2.png",
    frame_count: 17,
    horizontal_frames: 4,
    vertical_frames: 4,
    frame_rate: 15.,
  },
  EffectSpriteDef {
    effect_type: EffectSpriteType::Ricochet,
    texture_path: "sprites/ricochet.png",
    frame_count: 8,
    horizontal_frames: 4,
    vertical_frames: 2,
    frame_rate: 30.,
  },
];

struct Effect {
  materials: Vec<Handle<EffectSpriteMaterial>>,
  animation_time: f32,
  //init_time:f32,
}

#[derive(Resource, Default)]
struct EffectCollection(HashMap<EffectSpriteType, Effect>);

impl Plugin for EffectSpritePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<EffectCollection>()
      .add_plugins(MaterialPlugin::<EffectSpriteMaterial>::default())
      .add_message::<EffectSpriteMessage>()
      .add_systems(Startup, init_effect_sprite)
      .add_systems(Update, (spawn_effect_sprites, cleanup_effect_sprites));
  }
}

#[derive(Message)]
pub struct EffectSpriteMessage {
  translation: Vec3,
  scale: f32,
  velocity: Vec3,
  effect: EffectSpriteType,
}

impl EffectSpriteMessage {
  pub fn new(translation: Vec3, scale: f32, velocity: Vec3, effect: EffectSpriteType) -> Self {
    Self {
      translation,
      scale,
      velocity,
      effect,
    }
  }
}

#[derive(Resource)]
struct EffectQuad(Handle<Mesh>);

#[derive(Component)]
struct EffectSprite {
  timer: Timer,
}

#[derive(Default, Clone, Copy, AsBindGroup, Debug, ShaderType)]
pub struct EffectSpriteSettings {
  frame_rate: f32,
  frames_wide: f32,
  frames_deep: f32,
  frame_count: f32,
  //init_time:f32,
  //filler:Vec3,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct EffectSpriteMaterial {
  #[uniform(0)]
  settings: EffectSpriteSettings,
  #[texture(1)]
  #[sampler(2)]
  texture_atlas: Option<Handle<Image>>,
  alpha_mode: AlphaMode,
}

impl Material for EffectSpriteMaterial {
  fn vertex_shader() -> ShaderRef {
    EFFECT_SPRITE_SHADER_PATH.into()
  }

  fn fragment_shader() -> ShaderRef {
    EFFECT_SPRITE_SHADER_PATH.into()
  }
  fn alpha_mode(&self) -> AlphaMode {
    self.alpha_mode
  }
}

fn init_effect_sprite(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<EffectSpriteMaterial>>,
  mut loading: ResMut<AssetsLoading>,
  mut effect_collection: ResMut<EffectCollection>,
  asset_server: Res<AssetServer>,
  //time:Res<Time>,
) {
  let quad = meshes.add(create_quad());
  //let quad = meshes.add(Sphere::new(2.).mesh().uv(32, 18));
  commands.insert_resource(EffectQuad(quad));

  for effect in EFFECTS {
    let texture: Handle<Image> = asset_server.load(effect.texture_path);
    loading.0.push(texture.clone().untyped());
    //let init_time = time.elapsed_secs_wrapped();
    let material = materials.add(EffectSpriteMaterial {
      texture_atlas: Some(texture.clone()),
      alpha_mode: AlphaMode::Blend,
      settings: EffectSpriteSettings {
        frame_rate: effect.frame_rate,
        frame_count: effect.frame_count as f32,
        frames_deep: effect.vertical_frames as f32,
        frames_wide: effect.horizontal_frames as f32,
      },
    });

    let effect = effect_collection.0.entry(effect.effect_type).or_insert(
      Effect {
        materials: Vec::new(),
        animation_time: (effect.frame_count + 1) as f32 / effect.frame_rate,
      });
    effect.materials.push(material);  
    
  }
}

fn create_quad() -> Mesh {
  Mesh::new(PrimitiveTopology::TriangleList,
    RenderAssetUsages::default(),
  )
  .with_inserted_attribute(
    Mesh::ATTRIBUTE_POSITION,
    vec![[-1., 0., -1.], [1., 0., -1.], [1., 0., 1.], [-1., 0., 1.]],
  )
  .with_inserted_attribute(
    Mesh::ATTRIBUTE_UV_0,
    vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]],
  )
  .with_inserted_attribute(
    Mesh::ATTRIBUTE_NORMAL,
    vec![
      [0.0, 1.0, 0.0],
      [0.0, 1.0, 0.0],
      [0.0, 1.0, 0.0],
      [0.0, 1.0, 0.0],
    ],
  )
  .with_inserted_indices(Indices::U32(vec![0, 2, 1, 0, 3, 2]))
}

fn spawn_effect_sprites(
  mut commands: Commands,
  mut msg_effect_reader: MessageReader<EffectSpriteMessage>,
  mesh: Res<EffectQuad>,
  effects: Res<EffectCollection>,
  time: Res<Time>,
) {
  let mut rng = rand::rng();
  for sprite in msg_effect_reader.read() {
    let Some(effect) = effects.0.get(&sprite.effect) else {
      continue;
    };

    let transform = Transform::from_translation(sprite.translation + Vec3::new(0., rng.random_range(0.0 .. 0.3), 0.))
      .with_scale(Vec3::splat(sprite.scale))
      //.with_rotation(Quat::from_euler(EulerRot::XZX, PI * -0.5, rng.random_range(-1. .. 1.) * PI, 0.));
      .with_rotation(Quat::from_rotation_y(rng.random_range(-1. ..1.) * PI));
    let offset: f32 = time.elapsed_secs_wrapped();


    let material_id = rng.random_range(0 .. effect.materials.len());
    commands.spawn((
      GameEntity,
      EffectSprite {
        timer: Timer::from_seconds(effect.animation_time, TimerMode::Once),
      },
      Mesh3d(mesh.0.clone()),
      MeshMaterial3d(effect.materials[material_id].clone()),
      Velocity(sprite.velocity),
      transform,
      MeshTag(offset.to_bits()),
    ));
  }
}

fn cleanup_effect_sprites(
  mut commands: Commands,
  mut query: Query<(Entity, &mut EffectSprite)>,
  time: Res<Time>,
) {
  for (entity, mut sprite) in &mut query {
    sprite.timer.tick(time.delta());
    if sprite.timer.just_finished() {
      commands.entity(entity).despawn();
    }
  }
}
