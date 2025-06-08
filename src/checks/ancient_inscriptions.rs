use bevy::prelude::*;
use rand::random;

use crate::{character::{player_stats::PlayerStats, PlayerPosition}, contributing_stats, map::tiles::FountainPosition};

use super::Check;

pub struct AncientInscriptions {
  pub difficulty: i32,
  pub deciphered: Option<String>,
}

impl Check for AncientInscriptions {
  fn new(difficulty: i32) -> Self {
    Self { difficulty, deciphered: None }
  }

  fn difficulty(&self) -> i32 {
    self.difficulty
  }
  
  fn succeed(&mut self, world: &mut World) {
    self.deciphered = Some(generate_message(world));
  }

  fn failure_message(&self) -> Option<String> {
    Some("You cannot decipher the message".into())
  }
  
  fn success_message(&self) -> Option<String> {
    Some("You deciphered a message!".into())
  }
}

contributing_stats!(AncientInscriptions, [intelligence]);

fn generate_message(world: &mut World) -> String {
  let player_position = world.resource::<PlayerPosition>().0;
  let fountain_position = world.resource::<FountainPosition>().0;

  let relative_position = fountain_position - player_position;
  let direction = relative_position.to_cartesian().normalize();

  let options = [
    ("NORTH", Vec2::Y),
    ("EAST", Vec2::X),
    ("SOUTH", Vec2::NEG_Y),
    ("WEST", Vec2::NEG_X),
  ]
    .map(|(dir, cardinal)| (dir, cardinal.dot(direction)))
    .map(|(dir, dot)| (dir, dot.exp()));
  let sum = options.iter().fold(0.0, |acc, (_, value)| acc + value);
  let mut r = random::<f32>() * sum;
  
  let mut last_dir = "";
  for (dir, weight) in options {
    last_dir = dir;
    r -= weight;
    if r < 0.0 {
      break;
    }
  }
  format!("The ancient inscriptions suggest your destination is {last_dir}")
}
