use bevy::prelude::*;

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<HealthEvent>()
      .add_systems(Update, apply_health_changes);
  }
}

#[derive(Event)]
pub struct HealthEvent {
  pub entity: Entity,
  pub inflictor: Option<Entity>,
  pub health_adjustment: f32,
}

impl HealthEvent {
  pub fn new(entity: Entity, inflictor: Option<Entity>, health_adjustment: f32) -> Self {
    Self {
      entity,
      inflictor,
      health_adjustment,
    }
  }
}

#[derive(Component, Default)]
pub struct Health {
  pub value: f32,
  pub max: f32,
  pub last_hurt_by: Option<Entity>,
}

fn apply_health_changes(
  mut ev_health_reader: EventReader<HealthEvent>,
  mut query: Query<&mut Health>,
) {
  for HealthEvent {
    entity,
    inflictor,
    health_adjustment,
  } in ev_health_reader.read()
  {
    let Ok(mut health) = query.get_mut(*entity) else {
      continue;
    };
    if health.value >= 0. {
      if *health_adjustment < 0. && inflictor.is_some() {
        health.last_hurt_by = *inflictor;
      }
      health.value = (health.value + health_adjustment).min(health.max);
    }
  }
}
