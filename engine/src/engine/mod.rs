// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

pub mod window;

pub use self::window::Window;

mod component;
mod context;
mod process;
mod resource;
mod setup;

pub use self::component::*;
pub use self::context::*;
pub use self::process::*;
pub use self::resource::*;
pub use self::setup::*;

/// Runs the engine loop until `engine::exit` is called.
pub fn run(ctx: &mut Context) {
  let setup = ctx.setup.take().expect("engine context is already running");

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
