// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Context, Process};
use crate::prelude::*;

/// State of the engine during initialization.
#[derive(Default)]
pub(super) struct State<'a> {
  /// List of processes to run during the game loop.
  pub(super) processes: Vec<Box<dyn Process>>,
  /// Systems to dispatch early in the game loop.
  pub(super) early_systems: specs::DispatcherBuilder<'a, 'a>,
  /// Systems to dispatch in the game loop.
  pub(super) systems: specs::DispatcherBuilder<'a, 'a>,
  /// Systems to dispatch late in the game loop.
  pub(super) late_systems: specs::DispatcherBuilder<'a, 'a>,
}

/// Adds a process to the engine that will be run during the game loop.
pub fn add_process(ctx: &mut Context, process: impl Process + 'static) {
  let setup = ctx
    .init_state
    .as_mut()
    .expect("cannot add processes when engine is already running");

  setup.processes.push(Box::new(process));
}

/// Adds a system to the engine that will be dispatched early in the game
/// loop.
pub fn add_system_early<'a, T>(
  ctx: &mut Context<'a>,
  system: T,
  name: &'static str,
  dependencies: &[&'static str],
) where
  for<'b> T: System<'b> + Send + 'a,
{
  let setup = ctx
    .init_state
    .as_mut()
    .expect("cannot add systems when engine is already running");

  setup.early_systems.add(system, name, dependencies);
}

/// Adds a system to the engine that will be dispatched in the game loop.
pub fn add_system<'a, T>(
  ctx: &mut Context<'a>,
  system: T,
  name: &'static str,
  dependencies: &[&'static str],
) where
  for<'b> T: System<'b> + Send + 'a,
{
  let setup = ctx
    .init_state
    .as_mut()
    .expect("cannot add systems when engine is already running");

  setup.systems.add(system, name, dependencies);
}

/// Adds a system to the engine that will be dispatched late the game loop.
pub fn add_system_late<'a, T>(
  ctx: &mut Context<'a>,
  system: T,
  name: &'static str,
  dependencies: &[&'static str],
) where
  for<'b> T: System<'b> + Send + 'a,
{
  let setup = ctx
    .init_state
    .as_mut()
    .expect("cannot add systems when engine is already running");

  setup.late_systems.add(system, name, dependencies);
}
