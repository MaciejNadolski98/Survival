use bevy::prelude::*;

use crate::{character::{player_stats::PlayerStats, ChangeStamina}, contributing_stats};

use super::Check;

pub struct Climbing {
  pub difficulty: i32,
}

impl Check for Climbing {
  fn new(difficulty: i32) -> Self {
    Self { difficulty }
  }

  fn difficulty(&self) -> i32 {
    self.difficulty
  }

  fn fail(&mut self, world: &mut World) {
    world.trigger(ChangeStamina::new(-1));
  }
  
  fn failure_message(&self) -> Option<String> {
    Some("You lost extra stamina trying to climb the mountain".to_string())
  }
}

contributing_stats!(Climbing, [instinct, strength]);
