use bevy::prelude::*;
use crate::character::player_stats::PlayerStats;
use crate::checks::find_food::FindFood;
use crate::checks::wild_threat::WildThreat;
use crate::checks::Check;
use crate::map::tiles::{wrap_tile, Tile, TileBuilder};
use crate::map::HexCoord;

use super::TileTrait;

pub struct Forest {
  wild_threat: WildThreat,
  find_food: FindFood,
  image: Handle<Image>,
}

impl TileTrait for Forest {
  fn tile_name(&self) -> String {
    Self::name()
  }

  fn on_enter(
    &mut self,
    world: &mut World,
  ) {
    let player_stats = *world.resource::<PlayerStats>();

    self.wild_threat.check(world, &player_stats);
    self.find_food.check(world, &player_stats);
  }

  fn on_rest(&mut self, world: &mut World) {
    let player_stats = *world.resource::<PlayerStats>();

    self.find_food.check(world, &player_stats);
  }
  
  fn image(&self) -> Handle<Image> {
    self.image.clone()
  }
}

impl TileBuilder for Forest {
  fn name() -> String {
    "forest".into()
  }

  fn new(coord: HexCoord, server: AssetServer) -> Tile {
    wrap_tile(Self {
      wild_threat: WildThreat::new(2 + coord.check_modifier()),
      find_food: FindFood::new(2 + coord.check_modifier()),
      image: server.load(format!("{}.png", Self::name())),
    })
  }
}
