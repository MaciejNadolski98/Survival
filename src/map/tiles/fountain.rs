use bevy::prelude::*;

use crate::{character::{death_reason::{IsDead, TotalDeaths}, Log}, map::{tiles::{wrap_tile, Tile, TileBuilder}, HexCoord}};

use super::TileTrait;

pub struct Fountain {
  image: Handle<Image>,
}

impl TileTrait for Fountain {
  fn tile_name(&self) -> String {
    Self::name()
  }

  fn on_enter(
    &mut self,
    world: &mut World,
  ) {
    let total_deaths = world.resource::<TotalDeaths>().0;
    world.send_event(Log { text: "You win!".to_string(), expanded_text: format!("total deaths: {total_deaths}")});
    // Block interaction with the world:
    world.resource_mut::<IsDead>().0 = true;
  }

  fn on_rest(&mut self, _world: &mut World) {}
  
  fn image(&self) -> Handle<Image> {
    self.image.clone()
  }

  fn scale(&self) -> f32 {
    0.8
  }
}

impl TileBuilder for Fountain {
  fn name() -> String {
    "fountain".into()
  }

  fn new(_coord: HexCoord, server: AssetServer) -> Tile {
    wrap_tile(Self {
      image: server.load(format!("{}.png", Self::name())),
    })
  }
}
