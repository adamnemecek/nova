// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Context, Extension, TickState};
use crate::prelude::*;
use std::mem;

/// Adds an extension to the engine.
pub fn add_extension(ctx: &mut Context, extension: impl Extension + 'static) {
  match ctx.tick_state {
    TickState::PreInit {
      ref mut extensions, ..
    } => {
      extensions.push(Box::new(extension));
    }

    _ => {
      panic!("cannot add extensions after engine::init");
    }
  }
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
  match ctx.tick_state {
    TickState::PreInit {
      ref mut systems, ..
    } => {
      systems.add(system, name, dependencies);
    }

    _ => {
      panic!("cannot add systems after engine::init");
    }
  }
}

pub fn init(ctx: &mut Context) {
  let mut state = mem::replace(&mut ctx.tick_state, TickState::Init);

  match state {
    TickState::PreInit {
      systems,
      extensions,
    } => {
      let systems = systems.build();

      state = TickState::Ready {
        extensions,
        systems,
      };
    }

    _ => {
      panic!("engine::init has already been called");
    }
  };

  mem::replace(&mut ctx.tick_state, state);
}
