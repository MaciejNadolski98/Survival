use bevy::prelude::*;

use crate::{character::{player_stats::PlayerStats, ChangeSaturation}, contributing_stats};

use super::Check;

pub struct FindFood {
  pub difficulty: i32,
}

impl Check for FindFood {
  fn new(difficulty: i32) -> Self {
    Self { difficulty }
  }

  fn difficulty(&self) -> i32 {
    self.difficulty
  }
  
  fn succeed(&mut self, world: &mut World) {
    self.difficulty += 1;
    world.trigger(ChangeSaturation::new(2));
  }
  
  fn success_message(&self) -> Option<String> {
    Some("You found some food!".to_string())
  }
}

contributing_stats!(FindFood, [instinct, sight]);
