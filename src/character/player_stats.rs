use std::collections::HashMap;

use bevy::prelude::*;

use crate::character::{event_log::{display_tooltip, hide_tooltip, Tooltip}, UI_FONT_SIZE};

pub struct PlayerStatsPlugin;

impl Plugin for PlayerStatsPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<PlayerStats>()
      .add_systems(Startup, spawn_stats_panel)
      .add_systems(Update, update_stats);
  }
}

const STAT_DESCRIPTIONS: [(&'static str, &'static str); 9] = [
  (
    "intelligence",
    "Contributes to [AncientInscriptions]\nGained from [Chasm]"
  ),
  (
    "instinct",
    "Increases Saturation; Contributes to [FindFood, WildThreat, Climbing]\nGained from [WildThreat, Starvation]"
  ),
  (
    "focus",
    "Increases low rolls\nGained from [Chasm, Exhaustion]",
  ),
  (
    "luck",
    "Increases high rolls\nGained from [FallingRock]",
  ),
  (
    "strength",
    "Increases Health and Stamina; Contributes to [WildThreat, Climbing]\nGained from [WildThreat]"
  ),
  (
    "dexterity",
    "Increases Stamina; Contributes to [FallingRock]\nGained from [FallingRock]"
  ),
  (
    "sight",
    "Contributes to [Mirage, FindFood]\nGained from [Chasm, WildThreat]"
  ),
  (
    "endurance",
    "Contributes to [Mirage]; Increases Health and Saturation\nGained from [Exhaustion, Starvation]",
  ),
  (
    "regeneration",
    "Increases Stamina regeneration; Contributes to [Restoration]\nGained from [Exhaustion, Starvation]"
  )
];

#[derive(Resource, Default, Clone, Copy, Reflect)]
pub struct PlayerStats {
  // Contributes to [AncientInscriptions]
  // Gained from [Chasm]
  pub intelligence: i32,

  // Increases Saturation; Contributes to [FindFood, WildThreat, Climbing]
  // Gained from [WildThreat, Starvation]
  pub instinct: i32,

  // Increases low rolls
  // Gained from [Chasm, Exhaustion]
  pub focus: i32,

  // Increases high rolls
  // Gained from [FallingRock]
  pub luck: i32,

  // Increases Health and Stamina; Contributes to [WildThreat, Climbing]
  // Gained from [WildThreat]
  pub strength: i32,

  // Increases Stamina; Contributes to [FallingRock]
  // Gained from [FallingRock]
  pub dexterity: i32,

  // Contributes to [Mirage, FindFood]
  // Gained from [Chasm, WildThreat]
  pub sight: i32,

  // Contributes to [Mirage]; Increases Health and Saturation
  // Gained from [Exhaustion, Starvation]
  pub endurance: i32,

  // Increases Stamina regeneration; Contributes to [Restoration]
  // Gained from [Exhaustion, Starvation]
  pub regeneration: i32,
}

#[derive(Component)]
pub struct PlayerStatsPanel;

fn spawn_stats_panel(
  mut commands: Commands,
  player_stats: Res<PlayerStats>,
) {
  let descriptions = HashMap::from(STAT_DESCRIPTIONS);
  commands
    .spawn((
      Name::new("Stats panel"),
      Node {
        width: Val::Percent(20.0),
        height: Val::Percent(80.0),
        left: Val::Percent(1.0),
        top: Val::Percent(10.0),
        flex_direction: FlexDirection::Column,
        ..default()
      },
      BackgroundColor(Color::WHITE),
      PlayerStatsPanel,
    ))
    .with_children(|commands| {
      let field_count = player_stats.field_len();
      for i in 0..field_count {
        let field_name = player_stats.name_at(i).unwrap();
        let field_value: i32 = *player_stats.field_at(i).unwrap().try_downcast_ref().unwrap();

        commands.spawn((
          Node {
            height: Val::Percent(100.0 / field_count as f32),
            width: Val::Percent(100.0),
            ..default()
          },
          related!(Tooltip[(
            Node {
              position_type: PositionType::Absolute,
              ..default()
            },
            Text::new(
              descriptions[field_name]
            ),
            TextFont {
              font_size: UI_FONT_SIZE,
              ..default()
            },
            TextColor(Color::BLACK),
            BackgroundColor(Color::WHITE),
            Outline { width: Val::Px(1.0), offset: Val::Px(0.0), color: Color::BLACK },
            Pickable::IGNORE,
            ZIndex(1),
          )]),
        ))
        .observe(display_tooltip)
        .observe(hide_tooltip)
        .with_children(|commands| {
          commands
            .spawn((
              Node {
                width: Val::Percent(50.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                ..default()
              },
              Pickable::IGNORE,
            ))
            .with_child((
              Text::new(field_name),
              TextFont {
                font_size: UI_FONT_SIZE,
                ..default()
              },
              TextColor::from(Color::BLACK),
              Node {
                left: Val::Percent(10.0),
                ..default()
              },
              Pickable::IGNORE,
            ));
          commands
            .spawn((
              Node {
                width: Val::Percent(50.0),
                right: Val::Percent(0.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::RowReverse,
                ..default()
              },
              Pickable::IGNORE,
            ))
            .with_child((
              Text::new(field_value.to_string()),
              TextFont {
                font_size: UI_FONT_SIZE,
                ..default()
              },
              TextColor::from(Color::BLACK),
              Node {
                right: Val::Percent(10.0),
                ..default()
              },
              PlayerStatDisplay {
                stat: field_name.to_string(),
              },
              Pickable::IGNORE,
            ));
        });
      }
    });
}

#[derive(Component)]
struct PlayerStatDisplay {
  stat: String,
}

fn update_stats(
  player_stats: Res<PlayerStats>,
  player_stat_displays: Query<(&mut Text, &PlayerStatDisplay)>,
) {
  if !player_stats.is_changed() {
    return;
  }

  for (mut text, PlayerStatDisplay { stat }) in player_stat_displays {
    let field: i32 = *player_stats.field(stat).unwrap().try_downcast_ref().unwrap();
    *text = Text::new(field.to_string());
  }
}
