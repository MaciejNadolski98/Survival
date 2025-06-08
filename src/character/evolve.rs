use bevy::{ecs::component::{ComponentHook, Mutable, StorageType}, prelude::*};

use crate::{character::{death_reason::EvolveOption, player_stats::PlayerStats, UI_FONT_SIZE}, new_game};

pub struct EvolvePlugin;

impl Plugin for EvolvePlugin {
  fn build(&self, _app: &mut App) {
  }
}

pub struct EvolveButton(pub EvolveOption);

impl Component for EvolveButton {
  const STORAGE_TYPE: StorageType = StorageType::Table;
  type Mutability = Mutable;

  fn on_add() -> Option<ComponentHook> {
    Some(|mut world, context| {
      let stat_increase = world.get::<Self>(context.entity).unwrap().0.clone();
      world
        .commands()
        .entity(context.entity)
        .with_children(|commands| {
          commands
            .spawn((
              Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
              },
              BackgroundColor(Color::linear_rgb(0.3, 0.3, 0.3)),
              Outline {
                width: Val::Px(1.0),
                ..default()
              },
              Pickable {
                should_block_lower: true,
                is_hoverable: true,
              },
            ))
            .observe(press_button)
            .with_child((
              Text::new(stat_increase.description()),
              Node {
                align_self: AlignSelf::Center,
                ..default()
              },
              TextColor::BLACK,
              TextFont {
                font_size: UI_FONT_SIZE,
                ..default()
              },
              Pickable::IGNORE,
            ));
        });
    })
  }
}

fn press_button(
  trigger: Trigger<Pointer<Pressed>>,
  visible_buttons: Query<&ChildOf>,
  parent_buttons: Query<(Entity, &EvolveButton)>,
  mut commands: Commands,
  mut stats: ResMut<PlayerStats>,
) {
  let button_entity = visible_buttons.get(trigger.target()).unwrap().0;

  parent_buttons.get(button_entity).unwrap().1.0.evolve(&mut stats);
  for (entity, _) in parent_buttons {
    commands.entity(entity).despawn();
  }
  commands.run_system_cached(new_game);
}

