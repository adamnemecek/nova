// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `engine` module contains core engine functionality such as setting up
//! the window and running ECS.
//!
//! The `window` module can create a platform-specific window for graphics and
//! input events. The state of the window is stored in the `Window` resource.
//!
//! The `context` module defines the `engine::Context`, the global state for the
//! engine. A context can be created from a `Window` or without one.
//!
//! The `init` module defines structures and functions that are used in the
//! initialization of an engine context. Until `engine::run` is called, the
//! context is still in init mode and can have new systems and processes
//! added.

mod components;
mod entities;
mod init;
mod resources;
mod window;

pub use self::components::*;
pub use self::entities::*;
pub use self::init::*;
pub use self::resources::*;
pub use self::window::*;

pub use specs::System;

use std::cell::RefCell;

pub struct Context<'a, 'b> {
  /// Specs world of the engine.
  world: specs::World,
  /// Current tick state of the engine.
  tick_state: Option<TickState<'a, 'b>>,
  /// Handle to the window created with `window::create_window`.
  pub(crate) window_handle: RefCell<Option<WindowHandle>>,
  /// Whether the engine will exit.
  exiting: bool,
}

struct TickState<'a, 'b> {
  /// Extensions added to the engine.
  extensions: Vec<Box<dyn Extension>>,
  /// Systems to dispatch each engine tick.
  systems: SystemDispatcher<'a, 'b>,
}

enum SystemDispatcher<'a, 'b> {
  Building(specs::DispatcherBuilder<'a, 'b>),
  Built(specs::Dispatcher<'a, 'b>),
}

pub trait Extension {
  // Invoked before an engine tick.
  fn before_tick(&mut self, _ctx: &mut Context) {}
  // Invoked during an engine tick before systems are dispatched.
  fn before_systems(&mut self, _ctx: &mut Context) {}
  // Invoked during an engine tick after systems are dispatched.
  fn after_systems(&mut self, _ctx: &mut Context) {}
  // Invoked after an engine tick.
  fn after_tick(&mut self, _ctx: &mut Context) {}
}

impl<'a, 'b> Context<'a, 'b> {
  pub fn new() -> Self {
    Context {
      world: specs::World::new(),
      window_handle: RefCell::new(None),
      tick_state: Some(TickState {
        extensions: Vec::new(),
        systems: SystemDispatcher::Building(specs::DispatcherBuilder::new()),
      }),
      exiting: false,
    }
  }
}

pub fn run_loop(ctx: &mut Context) {
  while !ctx.exiting {
    tick(ctx);
  }
}

/// Exits the engine tick loop started with `engine::run`.
pub fn exit_loop(ctx: &mut Context) {
  ctx.exiting = true;
}

pub fn tick(ctx: &mut Context) {
  let mut state = ctx
    .tick_state
    .take()
    .expect("engine::tick is already running");

  if let SystemDispatcher::Built(ref mut systems) = state.systems {
    for extension in &mut state.extensions {
      extension.before_tick(ctx);
    }

    // Update the window each tick if there is one.
    if ctx.window_handle.borrow().is_some() {
      update_window(ctx);
    }

    for extension in &mut state.extensions {
      extension.before_systems(ctx);
    }

    systems.dispatch(&mut ctx.world.res);

    for extension in &mut state.extensions {
      extension.after_systems(ctx);
    }

    for extension in &mut state.extensions {
      extension.after_tick(ctx);
    }
  } else {
    panic!("cannot call engine::tick before engine::init");
  }

  ctx.tick_state = Some(state);
}
