// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `engine` module contains core engine functionality such as setting up
//! the window and running ECS.
//!
//! The `window` module defines the `Window`, a cross-platform abstraction
//! around a GUI window which is needed for input events and for graphics.
//!
//! The `context` module defines the `engine::Context`, the global state for the
//! engine. A context can be created from a `Window` or without one.
//!
//! The `setup` module defines structures and functions that are used in the
//! initial setup of an engine context. Until `engine::run` is called, the
//! context is still in “set up” mode and can have new systems and processes
//! added.
//!
//! The `process` module defines the `Process` trait, which describes an
//! _engine process_, a custom part of the main engine loop.
//!
//! The `resource` and `component` modules support ECS.

use super::*;

pub mod window;

pub use self::window::Window;

pub mod init;

mod component;
mod context;
mod process;
mod resource;

pub use self::component::*;
pub use self::context::*;
pub use self::process::*;
pub use self::resource::*;

/// Runs the engine loop until `engine::exit` is called.
///
/// Each iteration of the main engine loop is split into three sequential
/// phases: early, regular, and late. In each phase, first the systems for that
/// phase are dispatched, then processes are updated in the order they were
/// added.
///
/// “Early” systems and processes implement “inputs” to the engine. For example:
/// time, player input, or updates from a multiplayer server.
///
/// “Late” systems and processes implement “outputs” from the engine. For
/// example: sending updates _to_ a multiplayer server, calculating UI layout,
/// or drawing graphics on the screen.
///
/// The majority of systems and processes are neither early nor late, and they
/// implement most game logic.
///
/// Systems are run in parallel in each of the three phases. After the systems
/// run in each phase, all processes are updated in sequence.
pub fn run(ctx: &mut Context) {
  let setup = ctx
    .init_state
    .take()
    .expect("engine context is already running");

  let mut processes = setup.processes;
  let mut early_systems = setup.early_systems.build();
  let mut systems = setup.systems.build();
  let mut late_systems = setup.late_systems.build();

  // Run the engine loop.
  while !ctx.exiting {
    // Update the window each tick if there is one.
    if let Some(window) = ctx.window.borrow_mut().as_mut() {
      window.update();

      if window.is_closing() {
        ctx.exiting = true;
      }
    }

    // Run all systems and processes.
    early_systems.dispatch(&mut ctx.world.res);

    for process in &mut processes {
      process.early_update(ctx);
    }

    systems.dispatch(&mut ctx.world.res);

    for process in &mut processes {
      process.update(ctx);
    }

    late_systems.dispatch(&mut ctx.world.res);

    for process in &mut processes {
      process.late_update(ctx);
    }
  }
}

/// Creates a new entity builder that will build an entity in the engine
/// context.
pub fn create_entity<'a>(ctx: &'a mut Context) -> EntityBuilder<'a> {
  ctx.world.create_entity()
}

/// Exits the engine loop started with `engine::run`.
pub fn exit(ctx: &mut Context) {
  ctx.exiting = true;
}
