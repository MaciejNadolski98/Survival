use bevy::prelude::*;

use crate::{character::{player_stats::PlayerStats, IncreaseHealth}, contributing_stats};

use super::Check;

pub struct Restoration {
  pub difficulty: i32,
}

impl Check for Restoration {
  fn new(difficulty: i32) -> Self {
    Self { difficulty }
  }

  fn difficulty(&self) -> i32 {
    self.difficulty
  }
  
  fn succeed(&mut self, world: &mut World) {
    world.trigger(IncreaseHealth::new(1));
  }
  
  fn success_message(&self) -> Option<String> {
    Some("You healed 1 hp while resting".to_string())
  }
}

contributing_stats!(Restoration, [regeneration]);
