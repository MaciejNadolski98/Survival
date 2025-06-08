use bevy::prelude::*;

use crate::{character::player_stats::PlayerStats, checks::{climbing::Climbing, falling_rock::FallingRock, Check}, map::{tiles::{wrap_tile, Tile, TileBuilder}, HexCoord}};

use super::TileTrait;

pub struct Mountain {
  image: Handle<Image>,
  climbing: Climbing,
  falling_rock: FallingRock,
}

impl TileTrait for Mountain {
  fn tile_name(&self) -> String {
    Self::name()
  }

  fn on_enter(
    &mut self,
    world: &mut World,
  ) {
    let player_stats = *world.resource::<PlayerStats>();
    self.climbing.check(world, &player_stats);
    self.falling_rock.check(world, &player_stats);
  }

  fn on_rest(&mut self, _world: &mut World) {}
  
  fn image(&self) -> Handle<Image> {
    self.image.clone()
  }

  fn base_resting(&self) -> i32 {
    3
  }

  fn terrain_difficulty(&self) -> u32 {
    2
  }

  fn scale(&self) -> f32 {
    0.8
  }
}

impl TileBuilder for Mountain {
  fn name() -> String {
    "mountain".into()
  }

  fn new(coord: HexCoord, server: AssetServer) -> Tile {
    wrap_tile(Self {
      image: server.load(format!("{}.png", Self::name())),
      climbing: Climbing::new(2 + coord.check_modifier()),
      falling_rock: FallingRock::new(2 + coord.check_modifier()),
    })
  }
}
