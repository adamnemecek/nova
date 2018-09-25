// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Context;

/// One of the phases of the engine loop.
#[derive(Eq, PartialEq)]
pub enum LoopPhase {
  Early,
  Normal,
  Late,
}

/// Runs the engine loop until `engine::exit` is called.
pub fn run(ctx: &mut Context) {
  let init_state = ctx
    .init_state
    .take()
    .expect("engine context is already running");

  let mut extensions = init_state.extensions;
  let mut systems = init_state.systems.build();

  // Run the engine loop.
  while !ctx.exiting {
    for extension in &mut extensions {
      extension.before_tick(ctx);
    }

    // Update the window each loop if there is one.
    if ctx.window_handle.borrow().is_some() {
      super::update_window(ctx);
    }

    for extension in &mut extensions {
      extension.before_systems(ctx);
    }

    systems.dispatch(&mut ctx.world.res);

    for extension in &mut extensions {
      extension.after_systems(ctx);
    }

    for extension in &mut extensions {
      extension.after_tick(ctx);
    }
  }
}

/// Exits the engine loop started with `engine::run`.
pub fn exit(ctx: &mut Context) {
  ctx.exiting = true;
}
