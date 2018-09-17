// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate ggez;
extern crate nova_engine;
extern crate specs;

use nova_engine::prelude::*;
use std::error::Error;

/// Main entry point of the program.
pub fn main() -> Result<(), Box<dyn Error>> {
  let mut core = Core::new(core::context::build("nova", "bfrydl"));
  let mut dispatch = DispatcherBuilder::default();

  input::setup(&mut core, &mut dispatch);
  graphics::setup(&mut core, &mut dispatch);
  stage::setup(&mut core, &mut dispatch);

  unstable::setup(&mut core, &mut dispatch);

  setup(&mut core)?;

  let mut dispatcher = dispatch.build();

  let mut stage_renderer = stage::Renderer::default();
  let mut fps_display = core::FpsDisplay::default();

  // Run the main event loop.
  while core.is_running() {
    core.tick();

    dispatcher.dispatch(&mut core.world.res);

    stage_renderer.draw(&mut core);
    fps_display.draw(&mut core);
  }

  Ok(())
}

fn setup<'a, 'b>(core: &mut Core) -> Result<(), Box<dyn Error>> {
  let hero = unstable::actor::load(core, "/hero-f")?;

  unstable::actor::load(core, "/004-fire-salamander")?;

  // Set the camera target to the hero actor.
  core
    .world
    .write_resource::<stage::Camera>()
    .set_target(hero);

  core
    .world
    .write_storage()
    .insert(hero, unstable::InputControlled)?;

  Ok(())
}
