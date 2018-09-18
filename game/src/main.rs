// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate nova_engine;
extern crate specs;

use nova_engine::prelude::*;
use std::error::Error;
use std::path::PathBuf;

/// Main entry point of the program.
pub fn main() -> Result<(), Box<dyn Error>> {
  let mut core = Core::new("nova", "bfrydl");
  let mut dispatch = DispatcherBuilder::default();

  input::setup(&mut core, &mut dispatch);
  graphics::setup(&mut core, &mut dispatch);
  stage::setup(&mut core, &mut dispatch);

  unstable::setup(&mut core, &mut dispatch);

  setup(&mut core)?;

  let mut dispatcher = dispatch.build();
  let mut stage_renderer = stage::rendering::Renderer::default();

  // Run the main event loop.
  while core.is_running() {
    core.tick();
    dispatcher.dispatch(&mut core.world.res);
    stage_renderer.draw(&mut core);
  }

  Ok(())
}

fn setup<'a, 'b>(core: &mut Core) -> Result<(), Box<dyn Error>> {
  let (hero_template, monster_template) = {
    let assets = core.world.read_resource::<core::Assets>();

    (
      assets.load::<stage::actors::Template>(&PathBuf::from("hero-f/actor.yml"))?,
      assets.load::<stage::actors::Template>(&PathBuf::from("004-fire-salamander/actor.yml"))?,
    )
  };

  let hero =
    stage::actors::build_entity(Arc::new(hero_template), core.world.create_entity()).build();

  stage::actors::build_entity(Arc::new(monster_template), core.world.create_entity()).build();

  // Set the camera target to the hero.
  core
    .world
    .write_resource::<stage::rendering::Camera>()
    .set_target(hero);

  // Set the hero to be input controlled.
  core
    .world
    .write_storage()
    .insert(hero, unstable::InputControlled)?;

  Ok(())
}
