use bevy::{prelude::*, window::PrimaryWindow};

use crate::{character::UI_FONT_SIZE, WIDTH};

const MAX_ITEMS: u32 = 10;

pub struct EventLogPlugin;

impl Plugin for EventLogPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<Log>()
      .register_type::<Tooltip>()
      .register_type::<TooltipOf>()
      .add_systems(Startup, spawn_event_log)
      .add_systems(Update, log_events);
  }
}

#[derive(Event, Clone)]
pub struct Log {
  pub text: String,
  pub expanded_text: String,
}

#[derive(Component)]
pub struct EventLogPanel;

fn spawn_event_log(
  mut commands: Commands,
) {
  commands
    .spawn((
      Name::new("Event log panel"),
      Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(27.0),
        height: Val::Percent(80.0),
        right: Val::Percent(1.0),
        top: Val::Percent(10.0),
        ..default()
      },
      BackgroundColor(Color::BLACK),
      EventLogPanel,
    ));
}

fn log_events(
  mut reader: EventReader<Log>,
  mut commands: Commands,
) {
  for log in reader.read() {
    commands.run_system_cached_with(push_log, log.clone());
  }
}

#[derive(Component, Default)]
struct LogEntry {
  idx: u32,
}

fn push_log(
  log: In<Log>,
  panel: Query<Entity, With<EventLogPanel>>,
  children: Query<&Children>,
  mut nodes: Query<(&mut Node, &mut LogEntry)>,
  mut commands: Commands,
) {
  let panel = panel.single().unwrap();
  if let Ok(children) = children.get(panel) {
    for &child in children {
      let (mut node, mut entry) = nodes.get_mut(child).unwrap();
      entry.idx += 1;
      if entry.idx >= MAX_ITEMS {
        commands.entity(child).despawn();
        continue;
      }
      node.bottom = Val::Percent(100.0 * entry.idx as f32 / MAX_ITEMS as f32);
    }
  }

  commands
    .entity(panel)
    .with_children(|commands| {
      commands
        .spawn((
          Node {
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0 / MAX_ITEMS as f32),
            bottom: Val::Percent(0.0),
            ..default()
          },
          Outline { width: Val::Px(1.0), offset: Val::Px(0.0), color: Color::BLACK },
          BackgroundColor(Color::WHITE),
          LogEntry::default(),
          related!(Tooltip[(
            Node {
              position_type: PositionType::Absolute,
              ..default()
            },
            Text::new(log.expanded_text.clone()),
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
        .with_child((
          Node {
            left: Val::Percent(1.0),
            ..default()
          },
          Text::new(log.text.clone()),
          TextColor(Color::BLACK),
          TextFont {
            font_size: UI_FONT_SIZE,
            ..default()
          },
          Pickable::IGNORE,
        ));
    });
}

#[derive(Component, Reflect)]
#[require(
  Visibility::Hidden,
  Transform,
  Node {
    position_type: PositionType::Relative,
    width: Val::Percent(30.0),
    height: Val::Percent(10.0),
    ..default()
  },
)]
#[relationship(relationship_target = Tooltip)]
pub struct TooltipOf {
  #[relationship]
  element: Entity,
}

#[derive(Component, Reflect)]
#[relationship_target(relationship = TooltipOf)]
pub struct Tooltip {
  #[relationship]
  tooltip: Entity,
}

pub fn display_tooltip(
  trigger: Trigger<Pointer<Move>>,
  tooltip: Query<&Tooltip>,
  mut query: Query<(&mut Visibility, &mut Node, &ComputedNode)>,
  q_window: Query<&Window, With<PrimaryWindow>>,
) {
  let window = q_window.single().unwrap();
  let Some(cursor_position) = window.cursor_position() else { return; };
  let tooltip = tooltip.get(trigger.target).unwrap().tooltip;
  let (mut visibility, mut node, computed_node) = query.get_mut(tooltip).unwrap();
  *visibility = Visibility::Visible;

  if cursor_position.x < WIDTH / 2.0 {
    node.left = Val::Px(cursor_position.x);
    node.top = Val::Px(cursor_position.y);
  } else {
    node.left = Val::Px(cursor_position.x - computed_node.size.x * computed_node.inverse_scale_factor());
    node.top = Val::Px(cursor_position.y);
  }
}

pub fn hide_tooltip(
  trigger: Trigger<Pointer<Out>>,
  tooltip: Query<&Tooltip>,
  mut query: Query<&mut Visibility>,
) {
  let tooltip = tooltip.get(trigger.target).unwrap().tooltip;
  let mut visibility = query.get_mut(tooltip).unwrap();
  *visibility = Visibility::Hidden;
}

pub fn gray_out_logs(
  panel: Query<Entity, With<EventLogPanel>>,
  children: Query<&Children>,
  mut colors: Query<&mut BackgroundColor>,
  mut commands: Commands
) {
  let panel = panel.single().unwrap();
  if let Ok(children) = children.get(panel) {
    for &child in children {
      let mut color = colors.get_mut(child).unwrap();
      if let Some(new_color) = gray_out(color.0) {
        color.0 = new_color;
      } else {
        commands.entity(child).despawn();
      }
    }
  }
}

fn gray_out(color: Color) -> Option<Color> {
  let Srgba { red, green, blue, alpha } = color.to_srgba();
  let (red, green, blue) = (red - 0.2, green - 0.2, blue - 0.2);
  if red < 0.0 || green < 0.0 || blue < 0.0 {
    None
  } else {
    Some(Color::srgba(red, green, blue, alpha))
  }
}
