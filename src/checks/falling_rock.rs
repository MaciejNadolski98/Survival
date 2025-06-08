use bevy::prelude::*;

use crate::{character::{death_reason::ReduceHealth, player_stats::PlayerStats}, contributing_stats, evolutions_on_death};

use super::Check;

pub struct FallingRock {
  pub difficulty: i32,
}

impl Check for FallingRock {
  fn new(difficulty: i32) -> Self {
    Self { difficulty }
  }

  fn difficulty(&self) -> i32 {
    self.difficulty
  }
  
  fn fail(&mut self, world: &mut World) {
    world.trigger(ReduceHealth::<Self>::new(3));
  }
  
  fn failure_message(&self) -> Option<String> {
    Some("A boulder fell on you and crushed you".to_string())
  }
}

contributing_stats!(FallingRock, [dexterity]);
evolutions_on_death!(FallingRock, [dexterity, luck]);
