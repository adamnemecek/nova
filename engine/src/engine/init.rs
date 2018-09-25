// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Context, Extension, SystemDispatcher};
use crate::prelude::*;

/// Adds an extension to the engine.
pub fn add_extension(ctx: &mut Context, extension: impl Extension + 'static) {
  let tick_state = ctx
    .tick_state
    .as_mut()
    .expect("cannot add extensions during engine::tick");

  if let SystemDispatcher::Built(_) = tick_state.systems {
    panic!("cannot add extensions after engine::init");
  }

  tick_state.extensions.push(Box::new(extension));
}

/// Adds a system to the engine that is dispatched each engine tick.
pub fn add_system<T>(
  ctx: &mut Context,
  system: T,
  name: &'static str,
  dependencies: &[&'static str],
) where
  for<'a> T: System<'a> + Send + 'static,
{
  let tick_state = ctx
    .tick_state
    .as_mut()
    .expect("cannot add systems during engine::tick");

  if let SystemDispatcher::Building(ref mut systems) = tick_state.systems {
    systems.add(system, name, dependencies);
  } else {
    panic!("cannot add systems after engine::init");
  }
}

pub fn init(ctx: &mut Context) {
  let mut tick_state = ctx
    .tick_state
    .take()
    .expect("engine is already initialized");

  if let SystemDispatcher::Building(systems) = tick_state.systems {
    tick_state.systems = SystemDispatcher::Built(systems.build());
  } else {
    panic!("engine is already initialized");
  }

  ctx.tick_state = Some(tick_state);
}
