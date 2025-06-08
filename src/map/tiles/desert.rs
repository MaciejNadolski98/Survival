use bevy::prelude::*;

use crate::{character::player_stats::PlayerStats, checks::{mirage::Mirage, Check}, map::{tiles::{wrap_tile, Tile, TileBuilder}, HexCoord}};

use super::TileTrait;

pub struct Desert {
  image: Handle<Image>,
  mirage: Mirage,
}

impl TileTrait for Desert {
  fn tile_name(&self) -> String {
    Self::name()
  }

  fn on_enter(
    &mut self,
    world: &mut World,
  ) {
    let player_stats = *world.resource::<PlayerStats>();
    self.mirage.check(world, &player_stats);
  }

  fn on_rest(&mut self, _world: &mut World) {}
  
  fn image(&self) -> Handle<Image> {
    self.image.clone()
  }

  fn base_resting(&self) -> i32 {
    -2
  }

  fn terrain_difficulty(&self) -> u32 {
    2
  }

  fn scale(&self) -> f32 {
    0.8
  }
}

impl TileBuilder for Desert {
  fn name() -> String {
    "desert".into()
  }

  fn new(coord: HexCoord, server: AssetServer) -> Tile {
    wrap_tile(Self {
      image: server.load(format!("{}.png", Self::name())),
      mirage: Mirage::new(2 + coord.check_modifier()),
    })
  }
}
