use bevy::prelude::*;
use crate::character::player_stats::PlayerStats;
use crate::character::Log;
use crate::checks::ancient_inscriptions::AncientInscriptions;
use crate::checks::Check;
use crate::map::tiles::{wrap_tile, Tile, TileBuilder};
use crate::map::HexCoord;

use super::TileTrait;

pub struct Plain {
  ancient_inscriptions: AncientInscriptions,
  image: Handle<Image>,
}

impl TileTrait for Plain {
  fn tile_name(&self) -> String {
    Self::name()
  }

  fn on_enter(
    &mut self,
    world: &mut World,
  ) {
    if let Some(deciphered) = &self.ancient_inscriptions.deciphered {
      world.send_event(Log { text: deciphered.clone(), expanded_text: "Nothing changed...".into() });
    }
  }

  fn on_rest(&mut self, world: &mut World) {
    if let Some(deciphered) = &self.ancient_inscriptions.deciphered {
      world.send_event(Log { text: deciphered.clone(), expanded_text: "Nothing changed...".into() });
    } else {
      let player_stats = *world.resource::<PlayerStats>();
      self.ancient_inscriptions.check(world, &player_stats);
      if let Some(deciphered) = &self.ancient_inscriptions.deciphered {
        world.send_event(Log { text: deciphered.clone(), expanded_text: "I wonder if it's right...".into() });
      }
    }
  }
  
  fn image(&self) -> Handle<Image> {
    self.image.clone()
  }

  fn base_resting(&self) -> i32 {
    2
  }
  
  fn scale(&self) -> f32 {
    0.8
  }
}



impl TileBuilder for Plain {
  fn name() -> String {
    "plain".into()
  }

  fn new(coord: HexCoord, server: AssetServer) -> Tile {
    wrap_tile(Self {
      ancient_inscriptions: AncientInscriptions::new(2 + coord.check_modifier()),
      image: server.load(format!("{}.png", Self::name())),
    })
  }
}
