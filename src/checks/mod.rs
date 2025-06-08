use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::character::{player_stats::PlayerStats, Log};

pub mod find_food;
pub mod wild_threat;
pub mod ancient_inscriptions;
pub mod climbing;
pub mod falling_rock;
pub mod mirage;
pub mod restoration;

pub trait Check: RollResult {
  fn new(difficulty: i32) -> Self;

  fn check(&mut self, world: &mut World, player_stats: &PlayerStats) {
    let (check_summary, success) = Self::sum(self.difficulty(), player_stats);
    if success {
      if let Some(message) = self.success_message() {
        world.send_event(Log { text: message, expanded_text: check_summary });
      }
      self.succeed(world);
    } else {
      if let Some(message) = self.failure_message() {
        world.send_event(Log { text: message, expanded_text: check_summary });
      }
      self.fail(world);
    }
  }

  fn succeed(&mut self, _world: &mut World) {}
  fn fail(&mut self, _world: &mut World) {}

  fn success_message(&self) -> Option<String> { None }
  fn failure_message(&self) -> Option<String> { None }

  fn difficulty(&self) -> i32;
}

pub trait RollResult {  
  fn sum(difficulty: i32, player: &PlayerStats) -> (String, bool);
}

impl<M: Modifiers> RollResult for M {
  fn sum(difficulty: i32, player: &PlayerStats) -> (String, bool) {
    let (modifier_formula, modifier_value) = Self::sum_modifiers(player);
    let (roll_formula, roll_value) = roll(player);
    let total_value = modifier_value + roll_value;
    let difficulty = difficulty;
    let (success_formula, success) = success(total_value, difficulty);
    (
      format!("{success_formula}: {modifier_formula} + {roll_formula} vs {difficulty}"),
      success,
    )
  }
}
fn roll(player: &PlayerStats) -> (String, i32) {
  let mut rng = thread_rng();
  let (min, max) = (1 + player.focus, 6 + player.luck);
  let roll_value = if min < max { rng.gen_range(min..=max) } else { max };
  (
    format!("(roll: {roll_value})"), 
    roll_value as i32
  )
}

fn success(total_value: i32, difficulty: i32) -> (String, bool) {
  if total_value >= difficulty {
    ("success".to_string(), true)
  } else {
    ("failure".to_string(), false)
  }
}

pub trait Modifiers {
  fn sum_modifiers(player: &PlayerStats) -> (String, i32);
}

#[macro_export]
macro_rules! contributing_stats {
  ($name:ident, [$($field:ident),+]) => {
    use crate::checks::Modifiers;

    impl Modifiers for $name {
      fn sum_modifiers(player: &PlayerStats) -> (String, i32) {
        let mut formula = String::new();
        let mut total = 0;
        $(
          if !formula.is_empty() {
            formula.push_str(" + ");
          }
          formula.push_str("(");
          formula.push_str(stringify!($field));
          formula.push_str(": ");
          formula.push_str(&format!(
            "{}", player.$field
          ));
          formula.push_str(")");
          total += player.$field;
        )+
        (formula, total)
      }
    }
  };
}
