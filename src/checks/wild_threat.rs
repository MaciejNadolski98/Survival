use bevy::prelude::*;

use crate::{character::{death_reason::ReduceHealth, player_stats::PlayerStats}, contributing_stats, evolutions_on_death};

use super::Check;

pub struct WildThreat {
  pub difficulty: i32,
}

impl Check for WildThreat {
  fn new(difficulty: i32) -> Self {
    Self { difficulty }
  }

  fn difficulty(&self) -> i32 {
    self.difficulty
  }
  
  fn fail(&mut self, world: &mut World) {
    world.trigger(ReduceHealth::<Self>::new(1));
  }
  
  fn failure_message(&self) -> Option<String> {
    Some("A wild beast attacked you in the forest".to_string())
  }
}

contributing_stats!(WildThreat, [instinct, strength]);
evolutions_on_death!(WildThreat, [instinct, strength, sight]);
