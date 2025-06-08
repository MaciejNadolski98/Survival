use bevy::prelude::*;

use crate::{character::death_reason::ReduceHealth, evolutions_on_death, map::{tiles::{wrap_tile, Tile, TileBuilder}, HexCoord}};

use super::TileTrait;

pub struct Chasm {
  image: Handle<Image>,
}

impl TileTrait for Chasm {
  fn tile_name(&self) -> String {
    Self::name()
  }

  fn on_enter(
    &mut self,
    world: &mut World,
  ) {
    world.trigger(ReduceHealth::<Self>::new(999));
  }

  fn on_rest(&mut self, _world: &mut World) {}
  
  fn image(&self) -> Handle<Image> {
    self.image.clone()
  }

  fn scale(&self) -> f32 {
    0.8
  }
}

evolutions_on_death!(Chasm, [intelligence, focus, sight]);

impl TileBuilder for Chasm {
  fn name() -> String {
    "chasm".into()
  }

  fn new(_coord: HexCoord, server: AssetServer) -> Tile {
    wrap_tile(Self {
      image: server.load(format!("{}.png", Self::name())),
    })
  }
}
