use bevy::prelude::*;

use crate::game_manager::PauseState;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSchedule {
  HealthAdjust,
  ReadUserInput,
  PreEntityUpdates,
  EntityUpdates,
  CollisionDetection,
  DespawnEntities,
  PreDespawnEntities,
  PostEntityUpdates,
}

pub struct SchedulingPlugin;

impl Plugin for SchedulingPlugin {
  fn build(&self, app: &mut App) {
    app
      .configure_sets(
        Update,
        (
          GameSchedule::HealthAdjust,
          GameSchedule::PreDespawnEntities,
          GameSchedule::DespawnEntities,
          GameSchedule::ReadUserInput,
          GameSchedule::PreEntityUpdates,
          GameSchedule::EntityUpdates,
          GameSchedule::PostEntityUpdates,
        )
          .chain()
          .run_if(in_state(PauseState::Running)),
      )
      .configure_sets(
        PostUpdate,
        GameSchedule::CollisionDetection
          .after(TransformSystems::Propagate)
          .run_if(in_state(PauseState::Running)),
      );
  }
}
