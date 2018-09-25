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

use std::cell::RefCell;
use std::mem;

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

pub struct Context<'a, 'b> {
  /// Specs world of the engine.
  world: specs::World,
  /// Current basic state of the engine.
  state: EngineState<'a, 'b>,
  /// Handle to the window created with `window::create_window`.
  pub(crate) window_handle: RefCell<Option<WindowHandle>>,
  /// Whether the engine will exit.
  exiting: bool,
}

enum EngineState<'a, 'b> {
  PreInit {
    extensions: Vec<Box<dyn Extension>>,
    systems: specs::DispatcherBuilder<'a, 'b>,
  },
  Init,
  Ready {
    extensions: Vec<Box<dyn Extension>>,
    systems: specs::Dispatcher<'a, 'b>,
  },
  Ticking,
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
      state: EngineState::PreInit {
        extensions: Vec::new(),
        systems: specs::DispatcherBuilder::new(),
      },
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
  let mut state = mem::replace(&mut ctx.state, EngineState::Ticking);

  match state {
    EngineState::Ready {
      ref mut extensions,
      ref mut systems,
    } => {
      for extension in extensions.iter_mut() {
        extension.before_tick(ctx);
      }

      // Update the window each tick if there is one.
      if ctx.window_handle.borrow().is_some() {
        update_window(ctx);
      }

      for extension in extensions.iter_mut() {
        extension.before_systems(ctx);
      }

      systems.dispatch(&mut ctx.world.res);

      for extension in extensions.iter_mut() {
        extension.after_systems(ctx);
      }

      for extension in extensions.iter_mut() {
        extension.after_tick(ctx);
      }
    }

    EngineState::Init { .. } | EngineState::PreInit { .. } => {
      panic!("cannot call engine::tick before engine::init");
    }

    EngineState::Ticking => {
      panic!("engine is already ticking");
    }
  };

  mem::replace(&mut ctx.state, state);
}
