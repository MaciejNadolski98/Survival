use bevy::prelude::*;

use crate::{character::{death_reason::ReduceHealth, player_stats::PlayerStats}, evolutions_on_death, map::coords::HexCoord, HEIGHT, WIDTH};

pub mod death_reason;
mod evolve;
pub mod player_stats;
mod event_log;

use death_reason::DeathReasonPlugin;
use player_stats::PlayerStatsPlugin;
use event_log::EventLogPlugin;

pub use event_log::{Log, gray_out_logs};

pub struct CharacterPlugin;

pub const BAR_HEIGHT: f32 = HEIGHT / 20.0;
pub const BAR_WIDTH: f32 = WIDTH / 3.1;

pub const UI_FONT_SIZE: f32 = 15.0;

impl Plugin for CharacterPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins((DeathReasonPlugin, PlayerStatsPlugin, EventLogPlugin))
      .init_resource::<PlayerPosition>()
      .init_resource::<Health>()
      .add_event::<IncreaseHealth>()
      .init_resource::<Stamina>()
      .add_event::<ChangeStamina>()
      .init_resource::<Saturation>()
      .add_event::<ChangeSaturation>()
      .add_systems(Startup, (spawn_health_bar, spawn_stamina_bar, spawn_saturation_bar))
      .add_systems(Update, (update_health_bar, update_stamina_bar, update_saturation_bar))
      .add_observer(increase_health)
      .add_observer(change_stamina)
      .add_observer(change_saturation);
  }
}


#[derive(Resource, Default)]
pub struct PlayerPosition(pub HexCoord);

#[derive(Resource)]
pub struct Health {
  pub current: i32,
  pub max: i32,
}

impl From<&PlayerStats> for Health {
  fn from(stats: &PlayerStats) -> Self {
    let value = 3 + stats.endurance + stats.strength;
    Self {
      current: value,
      max: value,
    }
  }
}

impl FromWorld for Health {
  fn from_world(world: &mut World) -> Self {
    world.resource::<PlayerStats>().into()
  }
}

#[derive(Event)]
pub struct IncreaseHealth {
  value: u32,
}

impl IncreaseHealth {
  pub fn new(value: u32) -> Self {
    Self {
      value
    }
  }
}

fn increase_health(
  trigger: Trigger<IncreaseHealth>,
  mut health: ResMut<Health>,
) {
  health.current += trigger.value as i32;
  if health.current > health.max {
    health.current = health.max;
  }
}

#[derive(Resource)]
pub struct Stamina {
  pub current: i32,
  pub max: i32,
}

impl From<&PlayerStats> for Stamina {
  fn from(stats: &PlayerStats) -> Self {
    let value = 3 + stats.strength + stats.dexterity;
    Self {
      current: value,
      max: value,
    }
  }
}

impl FromWorld for Stamina {
  fn from_world(world: &mut World) -> Self {
    world.resource::<PlayerStats>().into()
  }
}

#[derive(Event)]
pub struct ChangeStamina {
  delta: i32,
}

impl ChangeStamina {
  pub fn new(delta: i32) -> Self {
    Self {
      delta
    }
  }
}

#[derive(Event)]
pub struct Exhaustion;

evolutions_on_death!(Exhaustion, [focus, endurance, regeneration]);

fn change_stamina(
  trigger: Trigger<ChangeStamina>,
  mut stamina: ResMut<Stamina>,
  mut commands: Commands,
) {
  let delta = trigger.delta;

  if -delta > stamina.current {
    stamina.current = 0;
    commands.trigger(ReduceHealth::<Exhaustion>::new(1));
    return;
  }

  stamina.current += delta;
  if stamina.current > stamina.max {
    stamina.current = stamina.max;
  }
}

#[derive(Resource)]
pub struct Saturation {
  pub current: i32,
  pub max: i32,
}

impl From<&PlayerStats> for Saturation {
  fn from(stats: &PlayerStats) -> Self {
    let value = 3 + stats.endurance + stats.instinct;
    Self {
      current: value,
      max: value,
    }
  }
}

impl FromWorld for Saturation {
  fn from_world(world: &mut World) -> Self {
    world.resource::<PlayerStats>().into()
  }
}

#[derive(Event)]
pub struct ChangeSaturation {
  delta: i32,
}

impl ChangeSaturation {
  pub fn new(delta: i32) -> Self {
    Self {
      delta
    }
  }
}

#[derive(Event)]
pub struct Starvation;

evolutions_on_death!(Starvation, [endurance, regeneration, instinct]);

fn change_saturation(
  trigger: Trigger<ChangeSaturation>,
  mut saturation: ResMut<Saturation>,
  mut commands: Commands,
) {
  let delta = trigger.delta;

  if -delta > saturation.current {
    saturation.current = 0;
    commands.trigger(ReduceHealth::<Starvation>::new(1));
    return;
  }

  saturation.current += delta;
  if saturation.current > saturation.max {
    saturation.current = saturation.max;
  }
}

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthIndicator;

#[derive(Component)]
pub struct HealthText;

fn spawn_health_bar(
  mut commands: Commands,
) {
  commands.spawn((
    HealthBar,
    Name::new("Health Bar"),
    Transform::from_translation(Vec3::new(-WIDTH / 3.0, HEIGHT / 2.0 - BAR_HEIGHT / 2.0, 0.0)),
    Visibility::Visible,
    related!(Children[
      (
        Name::new("Health background"),
        Sprite::from_color(Color::linear_rgb(0.5, 0.5, 0.5), Vec2::new(BAR_WIDTH, BAR_HEIGHT)),
      ),
      (
        HealthIndicator,
        Name::new("Health indicator"),
        Sprite::from_color(Color::linear_rgb(1.0, 0.0, 0.0), Vec2::new(BAR_WIDTH, BAR_HEIGHT)),
      ),
      (
        HealthText,
        Name::new("Health text"),
        Text2d::new(""),
        TextColor::BLACK,
      ),
    ])
  ));
}

fn update_health_bar(
  health: Res<Health>,
  mut health_indicator: Query<(&mut Sprite, &mut Transform), With<HealthIndicator>>,
  mut health_text: Query<&mut Text2d, With<HealthText>>,
) {
  if let Ok((mut sprite, mut transform)) = health_indicator.single_mut() {
    let ratio = health.current as f32 / health.max as f32;
    sprite.custom_size = Some(Vec2::new(BAR_WIDTH * ratio, BAR_HEIGHT));
    transform.translation = Vec3::new(-BAR_WIDTH / 2.0 + ratio * BAR_WIDTH / 2.0, 0.0, 0.0);
  }

  if let Ok(mut text) = health_text.single_mut() {
    *text = Text2d::new(format!("{}/{}", health.current, health.max))
  }
}

#[derive(Component)]
pub struct StaminaBar;

#[derive(Component)]
pub struct StaminaIndicator;

#[derive(Component)]
pub struct StaminaText;

fn spawn_stamina_bar(
  mut commands: Commands,
) {
  commands.spawn((
    StaminaBar,
    Name::new("Stamina Bar"),
    Transform::from_translation(Vec3::new(0.0, HEIGHT / 2.0 - BAR_HEIGHT / 2.0, 0.0)),
    Visibility::Visible,
    related!(Children[
      (
        Name::new("Stamina background"),
        Sprite::from_color(Color::linear_rgb(0.5, 0.5, 0.5), Vec2::new(BAR_WIDTH, BAR_HEIGHT)),
      ),
      (
        StaminaIndicator,
        Name::new("Stamina indicator"),
        Sprite::from_color(Color::linear_rgb(1.0, 1.0, 0.0), Vec2::new(BAR_WIDTH, BAR_HEIGHT)),
      ),
      (
        StaminaText,
        Name::new("Stamina text"),
        Text2d::new(""),
        TextColor::BLACK,
      ),
    ])
  ));
}

fn update_stamina_bar(
  stamina: Res<Stamina>,
  mut stamina_indicator: Query<(&mut Sprite, &mut Transform), With<StaminaIndicator>>,
  mut stamina_text: Query<&mut Text2d, With<StaminaText>>,
) {
  if let Ok((mut sprite, mut transform)) = stamina_indicator.single_mut() {
    let ratio = stamina.current as f32 / stamina.max as f32;
    sprite.custom_size = Some(Vec2::new(BAR_WIDTH * ratio, BAR_HEIGHT));
    transform.translation = Vec3::new(-BAR_WIDTH / 2.0 + ratio * BAR_WIDTH / 2.0, 0.0, 0.0);
  }

  if let Ok(mut text) = stamina_text.single_mut() {
    *text = Text2d::new(format!("{}/{}", stamina.current, stamina.max))
  }
}


#[derive(Component)]
pub struct SaturationBar;

#[derive(Component)]
pub struct SaturationIndicator;

#[derive(Component)]
pub struct SaturationText;

fn spawn_saturation_bar(
  mut commands: Commands,
) {
  commands.spawn((
    SaturationBar,
    Name::new("Saturation Bar"),
    Transform::from_translation(Vec3::new(WIDTH / 3.0, HEIGHT / 2.0 - BAR_HEIGHT / 2.0, 0.0)),
    Visibility::Visible,
    related!(Children[
      (
        Name::new("Saturation background"),
        Sprite::from_color(Color::linear_rgb(0.5, 0.5, 0.5), Vec2::new(BAR_WIDTH, BAR_HEIGHT)),
      ),
      (
        SaturationIndicator,
        Name::new("Saturation indicator"),
        Sprite::from_color(Color::linear_rgb(0.0, 1.0, 0.0), Vec2::new(BAR_WIDTH, BAR_HEIGHT)),
      ),
      (
        SaturationText,
        Name::new("Saturation text"),
        Text2d::new(""),
        TextColor::BLACK,
      ),
    ])
  ));
}

fn update_saturation_bar(
  saturation: Res<Saturation>,
  mut saturation_indicator: Query<(&mut Sprite, &mut Transform), With<SaturationIndicator>>,
  mut saturation_text: Query<&mut Text2d, With<SaturationText>>,
) {
  if let Ok((mut sprite, mut transform)) = saturation_indicator.single_mut() {
    let ratio = saturation.current as f32 / saturation.max as f32;
    sprite.custom_size = Some(Vec2::new(BAR_WIDTH * ratio, BAR_HEIGHT));
    transform.translation = Vec3::new(-BAR_WIDTH / 2.0 + ratio * BAR_WIDTH / 2.0, 0.0, 0.0);
  }

  if let Ok(mut text) = saturation_text.single_mut() {
    *text = Text2d::new(format!("{}/{}", saturation.current, saturation.max))
  }
}
