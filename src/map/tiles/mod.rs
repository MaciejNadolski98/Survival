use std::sync::{Arc, Mutex};
use rand::{thread_rng, seq::SliceRandom};

use bevy::prelude::*;

pub mod forest;
pub mod desert;
pub mod fountain;
pub mod chasm;
pub mod plain;
pub mod mountain;

pub use forest::Forest;
pub use desert::Desert;
pub use fountain::Fountain;
pub use chasm::Chasm;
pub use plain::Plain;
pub use mountain::Mountain;

use crate::{character::{death_reason::IsDead, gray_out_logs, player_stats::PlayerStats, ChangeSaturation, ChangeStamina, PlayerPosition}, checks::{restoration::Restoration, Check}, map::{coords::{HexCoord, HEX_CIRCUMRADIUS, MARGIN}, GameMap}};

pub struct TilePlugin;

impl Plugin for TilePlugin{
  fn build(&self, app: &mut App) {
    app
      .add_plugins(MeshPickingPlugin)
      .init_resource::<FountainPosition>()
      .add_systems(Startup, spawn_fountain);
  }
}

pub trait TileTrait: Send + Sync + 'static {
  fn tile_name(&self) -> String;

  fn on_enter(&mut self, world: &mut World);
  fn on_rest(&mut self, world: &mut World);

  fn image(&self) -> Handle<Image>;

  fn terrain_difficulty(&self) -> u32 {
    1
  }

  fn base_resting(&self) -> i32 {
    1
  }

  fn scale(&self) -> f32 {
    1.0
  }
}

pub type Tile = Arc<Mutex<dyn TileTrait>>;

impl TileTrait for Tile {
  fn tile_name(&self) -> String {
    self.lock().unwrap().tile_name()
  }

  fn on_enter(&mut self, world: &mut World) {
    self.lock().unwrap().on_enter(world)
  }

  fn on_rest(&mut self, world: &mut World) {
    self.lock().unwrap().on_rest(world)
  }

  fn image(&self) -> Handle<Image> {
    self.lock().unwrap().image()
  }

  fn terrain_difficulty(&self) -> u32 {
    self.lock().unwrap().terrain_difficulty()
  }

  fn base_resting(&self) -> i32 {
    self.lock().unwrap().base_resting()
  }

  fn scale(&self) -> f32 {
    self.lock().unwrap().scale()
  }
}

pub trait TileBuilder {
  fn name() -> String;
  fn new(coord: HexCoord, server: AssetServer) -> Tile;
}

pub fn wrap_tile<T: TileTrait>(value: T) -> Tile {
  Arc::new(Mutex::new(value))
}

pub fn generate_tile(coord: HexCoord, server: AssetServer) -> Tile {
  [
    wrap_tile(Forest::new(coord, server.clone())),
    wrap_tile(Desert::new(coord, server.clone())),
    wrap_tile(Chasm::new(coord, server.clone())),
    wrap_tile(Plain::new(coord, server.clone())),
    wrap_tile(Mountain::new(coord, server.clone())),
  ].choose(&mut thread_rng()).unwrap().clone()
}

pub type SpawnTypeInput = In<(HexCoord, Tile, bool)>;

pub fn spawn_tile(
  input: SpawnTypeInput,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  player_position: Res<PlayerPosition>,
) {
  let (coord, tile, visited) = input.0;
  let relative_coord = coord - player_position.0;
  let default_material = if visited { materials.add(Color::linear_rgb(0.4, 0.4, 0.4)) } else { materials.add(Color::BLACK) };
  let mut entity_commands = commands
    .spawn((
      Mesh2d(meshes.add(RegularPolygon::new(HEX_CIRCUMRADIUS - MARGIN, 6))),
      MeshMaterial2d(default_material.clone()),
      Transform::from_translation((relative_coord.to_cartesian(), 0.0).into()),
      TileMarker,
      Interaction::None,
      Pickable {
        should_block_lower: true,
        is_hoverable: true,
      },
      coord,
      related!(Children[
        (
          Sprite::from_image(tile.image()),
          Transform::from_scale(Vec3::splat(0.15 * tile.scale())),
          Pickable::IGNORE,
        )
      ])
    ));
  if coord.distance(player_position.0) <= 1 {
    entity_commands
      .observe(update_material_on::<Pointer<Over>>(materials.add(Color::WHITE)))
      .observe(update_material_on::<Pointer<Out>>(default_material.clone()));
  }
  if coord.distance(player_position.0) == 1 {
    entity_commands
      .observe(on_press.pipe(move_player));
  }
  if coord == player_position.0 {
    entity_commands
      .observe(on_press.pipe(rest));
  }
}

fn update_material_on<E: Event>(new_material: Handle<ColorMaterial>) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial2d<ColorMaterial>>) {
  move |trigger, mut materials| {
    if let Ok(mut material) = materials.get_mut(trigger.target()) {
      material.0 = new_material.clone();
    }
  }
}

fn on_press(trigger: Trigger<Pointer<Pressed>>, query: Query<&HexCoord>) -> HexCoord {
  *query.get(trigger.target()).unwrap()
}

fn move_player(
  target: In<HexCoord>, 
  mut player_position: ResMut<PlayerPosition>,
  mut map: ResMut<GameMap>,
  server: Res<AssetServer>,
  mut commands: Commands,
  is_dead: Res<IsDead>,
) {
  if is_dead.0 {
    return;
  }
  commands.run_system_cached(next_turn);

  player_position.0 = *target;
  let tile = map.get(*target, server.clone());
  map.visited_tiles.insert(*target);
  commands.run_system_cached_with(on_enter, tile);
}

fn on_enter(
  mut tile: In<Tile>,
  world: &mut World,
) {
  world.trigger(ChangeStamina::new(-(tile.terrain_difficulty() as i32)));
  tile.on_enter(world);
}

fn rest(
  target: In<HexCoord>, 
  mut commands: Commands,
  is_dead: Res<IsDead>,
) {
  if is_dead.0 {
    return;
  }
  commands.run_system_cached(next_turn);
  commands.run_system_cached_with(on_rest, *target);
}

fn on_rest(
  coord: In<HexCoord>,
  world: &mut World,
) {
  let server = world.resource::<AssetServer>().clone();
  let mut map = world.resource_mut::<GameMap>();
  let mut tile = map.get(*coord, server.clone());
  let stats = *world.resource::<PlayerStats>();
  world.trigger(ChangeStamina::new(tile.base_resting() + stats.regeneration));
  Restoration::new(2 + coord.check_modifier()).check(world, &stats);

  tile.on_rest(world);
}

fn next_turn(
  mut commands: Commands
) {
  commands.trigger(ChangeSaturation::new(-1));
  commands.run_system_cached(gray_out_logs);
}

#[derive(Component)]
pub struct TileMarker;

const FOUNTAIN_DISTANCE: u32 = 15;

#[derive(Resource, Default)]
pub struct FountainPosition(pub HexCoord);

fn spawn_fountain(
  mut map: ResMut<GameMap>,
  server: Res<AssetServer>,
  mut fountain_position: ResMut<FountainPosition>,
) {
  let random_coord = HexCoord::random_in_distance(FOUNTAIN_DISTANCE);
  map.tiles.insert(random_coord, wrap_tile(Fountain::new(random_coord, server.clone())));
  fountain_position.0 = random_coord;
}
