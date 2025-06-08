// #![feature(trace_macros)]
// trace_macros!(false);

use bevy::{
  input::common_conditions::input_toggle_active, 
  prelude::*, 
  window::WindowResolution
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_defer::AsyncPlugin;

pub mod character;
pub mod map;
pub mod utils;
pub mod checks;

use character::CharacterPlugin;

use crate::{character::{death_reason::IsDead, player_stats::PlayerStats, Health, PlayerPosition, Saturation, Stamina}, map::{coords::HexCoord, MapPlugin}};

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

fn main() {
  App::new()
    .add_plugins(AsyncPlugin::default_settings())
    .add_plugins(
      DefaultPlugins
      .set(WindowPlugin {
        primary_window: Some(Window {
          title: "Survival".to_string(),
          resolution: WindowResolution::new(WIDTH, HEIGHT),
          ..Default::default()
        }),
        ..default()
      })
      .set(ImagePlugin::default_nearest())
    )
    .add_plugins((CharacterPlugin, MapPlugin))
    .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
    .add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
    )
    .add_systems(Startup, (spawn_camera, new_game))
    .run();
}

#[derive(Component)]
pub struct MyCamera;

fn spawn_camera(mut commands: Commands) {
  commands.spawn((
    Camera2d,
    MyCamera,
  ));
}

fn new_game(
  mut player_position: ResMut<PlayerPosition>,
  mut health: ResMut<Health>,
  mut stamina: ResMut<Stamina>,
  mut saturation: ResMut<Saturation>,
  player_stats: Res<PlayerStats>,
  mut is_dead: ResMut<IsDead>,
) {
  is_dead.0 = false;
  player_position.0 = HexCoord::new(0, 0);
  *health = player_stats.as_ref().into();
  *stamina = player_stats.as_ref().into();
  *saturation = player_stats.as_ref().into();
}
