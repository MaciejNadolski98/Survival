use std::{marker::PhantomData, sync::Arc};

use bevy::prelude::*;

use crate::{character::{evolve::EvolveButton, player_stats::PlayerStats, Exhaustion, Health, Log, Starvation}, checks::{falling_rock::FallingRock, wild_threat::WildThreat}, map::tiles::Chasm};

macro_rules! all_death_reasons {
  ($($type:ty),+) => {
    all_death_reasons!(@internal $($type),+)
  };
  (@internal $head:ty, $($tail:ty),+) => {
    ($head, all_death_reasons!(@internal $($tail),+))
  };
  (@internal $type:ty) => {
    $type
  };
}

pub type AllDeathReasons = all_death_reasons!(WildThreat, Exhaustion, Starvation, Chasm, FallingRock);

pub struct DeathReasonPlugin;

impl Plugin for DeathReasonPlugin {
  fn build(&self, app: &mut App) {
    AllDeathReasons::register(app);
    app
      .init_resource::<IsDead>()
      .init_resource::<TotalDeaths>();
  }
}

trait DeathReasons {
  fn register(app: &mut App);
}

impl<T: DeathReason> DeathReasons for T {
  fn register(app: &mut App) {
    app
      .add_event::<ReduceHealth<T>>()
      .add_observer(on_health_reduce::<T>)
      .add_observer(on_death::<T>);
  }
}

impl<T1: DeathReasons, T2: DeathReasons> DeathReasons for (T1, T2) {
  fn register(app: &mut App) {
    T1::register(app);
    T2::register(app);
  }
}

pub trait DeathReason: Send + Sync + 'static {
  fn evolve_options() -> [EvolveOption; 2] {
    [Self::evolve_option(), Self::evolve_option()]
  }

  fn evolve_option() -> EvolveOption {
    EvolveOption([Self::stat_increase(), Self::stat_increase()])
  }

  fn stat_increase() -> StatIncrease;

  fn name() -> String;
}

#[derive(Clone)]
pub struct EvolveOption([StatIncrease; 2]);

impl EvolveOption {
  pub fn description(&self) -> String {
    let [StatIncrease { field: field1, .. }, StatIncrease { field: field2, .. }] = &self.0;
    if field1 == field2 {
      format!("{}++", field1)
    } else {
      format!("{}+ {}+", field1, field2)
    }
  }

  pub fn evolve(&self, player_stats: &mut PlayerStats) {
    (self.0[0].apply.as_ref())(player_stats);
    (self.0[1].apply.as_ref())(player_stats);
  }
}

#[derive(Clone)]
pub struct StatIncrease {
  pub field: String,
  pub apply: Arc<Box<dyn Fn(&mut PlayerStats) + Send + Sync + 'static>>,
}

#[macro_export]
macro_rules! evolutions_on_death {
  ($name:ident, [$($field:ident),+]) => {
    impl crate::character::death_reason::DeathReason for $name {
      fn stat_increase() -> crate::character::death_reason::StatIncrease {
        const COUNT: usize = [$(stringify!($field)),+].len();
        #[allow(unused_variables)]
        let r = rand::random::<usize>() % COUNT;
        {
          evolutions_on_death!(@internal r, 0, $($field),+)
        }
      }

      fn name() -> String {
        stringify!($name).to_string()
      }
    }
  };

  // Recursive case
  (@internal $r:expr, $idx:expr, $head_field:ident, $($tail_fields:ident),+) => {
    if $r == $idx {
      crate::character::death_reason::StatIncrease {
        field: stringify!($head_field).to_string(),
        apply: std::sync::Arc::new(Box::new(|stats: &mut crate::character::player_stats::PlayerStats| {
          stats.$head_field += 1;
        })),
      }
    } else {
      evolutions_on_death!(@internal $r, $idx + 1, $($tail_fields),+)
    }
  };

  // Base case
  (@internal $r:expr, $idx:expr, $last_field:ident) => {
    crate::character::death_reason::StatIncrease {
      field: stringify!($last_field).to_string(),
      apply: std::sync::Arc::new(Box::new(|stats: &mut crate::character::player_stats::PlayerStats| {
        stats.$last_field += 1;
      })),
    }
  };
}

#[derive(Event)]
pub struct ReduceHealth<S: DeathReason> {
  amount: i32,
  _marker: PhantomData<S>,
}

impl<S: DeathReason> ReduceHealth<S> {
  pub fn new(damage:i32) -> ReduceHealth<S> {
    Self {
      amount: damage,
      _marker: PhantomData::<S>,
    }
  }
}

fn on_health_reduce<R: DeathReason>(
  trigger: Trigger<ReduceHealth<R>>,
  mut health: ResMut<Health>,
  mut commands: Commands,
) {
  health.current -= trigger.amount;
  if health.current <= 0 {
    commands.trigger(Death::<R>::new());
  }
}

#[derive(Event)]
pub struct Death<R: DeathReason>(PhantomData<R>);

impl<R: DeathReason> Death<R> {
  fn new() -> Self { Self(PhantomData::<R>) }
}

#[derive(Resource, Default)]
pub struct IsDead(pub bool);

#[derive(Resource, Default)]
pub struct TotalDeaths(pub u32);

fn on_death<R: DeathReason>(
  _trigger: Trigger<Death<R>>,
  mut commands: Commands,
  mut is_dead: ResMut<IsDead>,
  mut total_deaths: ResMut<TotalDeaths>,
) {
  if is_dead.0 { 
    return; 
  }
  is_dead.0 = true;
  total_deaths.0 += 1;

  commands.send_event(Log { 
    text: "You died!".to_string(), 
    expanded_text: format!("death reason: {}", R::name()),
  });
  let options = R::evolve_options();

  commands.spawn((
    EvolveButton(options[0].clone()),
    Node {
      position_type: PositionType::Absolute,
      left: Val::Percent(20.0),
      top: Val::Percent(40.0),
      width: Val::Percent(20.0),
      height: Val::Percent(20.0),
      ..default()
    },
    ZIndex(2),
  ));
  commands.spawn((
    EvolveButton(options[1].clone()),
    Node {
      position_type: PositionType::Absolute,
      right: Val::Percent(20.0),
      top: Val::Percent(40.0),
      width: Val::Percent(20.0),
      height: Val::Percent(20.0),
      ..default()
    },
    ZIndex(2),
  ));
}
