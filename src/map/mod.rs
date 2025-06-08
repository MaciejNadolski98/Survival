use bevy::{platform::collections::{HashMap, HashSet}, prelude::*};

pub mod coords;
pub mod tiles;

pub use coords::HexCoord;
use tiles::{Tile, TilePlugin};

use crate::{character::PlayerPosition, map::tiles::{generate_tile, spawn_tile, wrap_tile, Forest, TileBuilder, TileMarker}};
pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(TilePlugin)
      .init_resource::<GameMap>()
      .add_systems(Update, update_tiles.run_if(changed::<PlayerPosition>))
      .add_systems(Startup, update_tiles);
  }
}

const VIEW_DISTANCE: u32 = 2;

fn changed<R: Resource>(
  resource: Res<R>,
) -> bool {
  resource.is_changed()
}


#[derive(Resource)]
pub struct GameMap {
  tiles: HashMap<HexCoord, Tile>,
  visited_tiles: HashSet<HexCoord>,
}

impl FromWorld for GameMap {
  fn from_world(world: &mut World) -> Self {
    let server = world.resource::<AssetServer>();
    let starting_coord = HexCoord::new(0, 0);
    Self {
      tiles: HashMap::from([(starting_coord, wrap_tile(Forest::new(starting_coord, server.clone())))]),
      visited_tiles: HashSet::from([starting_coord]),
    }
  }
}

impl GameMap {
  pub fn get(&mut self, coord: HexCoord, server: AssetServer) -> Tile {
    if let Some(tile) = self.tiles.get(&coord) {
      tile.clone()
    } else {
      let tile = generate_tile(coord, server);
      self.tiles.insert(coord, tile.clone());
      tile
    }
  }
}

fn update_tiles(
  tiles: Query<Entity, With<TileMarker>>,
  mut map: ResMut<GameMap>,
  position: Res<PlayerPosition>,
  mut commands: Commands,
  server: Res<AssetServer>,
) {
  for tile in tiles {
    commands.entity(tile).despawn();
  }

  for coord in position.0.within_distance(VIEW_DISTANCE) {
    commands.run_system_cached_with(
      spawn_tile, 
      (
        coord, 
        map.get(coord, server.clone()), 
        map.visited_tiles.contains(&coord),
      )
    );
  }
}
