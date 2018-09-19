// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate nova;
extern crate specs;

use nova::prelude::*;
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

  // Run the main event loop.
  while core.is_running() {
    core.tick();
    dispatcher.dispatch(&mut core.world.res);
    stage::render(&mut core);
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

  let _monster =
    stage::actors::build_entity(Arc::new(monster_template), core.world.create_entity())
      .with(stage::Position {
        point: Point3::new(32.0, 24.0, 0.0),
      })
      .build();

  // Set the camera target to the hero.
  core
    .world
    .write_resource::<stage::Camera>()
    .set_target(hero);

  // Set the hero to be input controlled.
  core
    .world
    .write_storage()
    .insert(hero, unstable::InputControlled)?;

  // Set the object shadow texture.
  core
    .world
    .write_resource::<stage::objects::render::Settings>()
    .shadow_texture = core
    .world
    .read_resource::<core::Assets>()
    .load(&PathBuf::from("circle.png"))
    .ok()
    .map(Arc::new);

  Ok(())
}
