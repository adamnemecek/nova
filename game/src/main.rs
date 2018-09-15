// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate ggez;
extern crate nova_engine;
extern crate specs;

use nova_engine::prelude::*;
use std::error::Error;
use std::sync::Arc;

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
    core.update();

    dispatcher.dispatch(&mut core.world.res);

    stage_renderer.draw(&mut core);
    fps_display.draw(&mut core);
  }

  Ok(())
}

fn setup<'a, 'b>(core: &mut Core) -> Result<(), Box<dyn Error>> {
  // Add a character to the world.
  {
    let atlas = graphics::Atlas::load(core, "/004-fire-salamander/atlas")?;

    core
      .world
      .create_entity()
      .with(graphics::Sprite {
        atlas: Arc::new(atlas),
        cell: 0,
      })
      .with(stage::Position(Point3::new(100.0, 100.0, 0.0)))
      .with(stage::Render)
      .build();
  }

  // Add another character to the world.
  {
    let atlas = graphics::Atlas::load(core, "/hero-f/atlas")?;

    let entity = core
      .world
      .create_entity()
      .with(graphics::Sprite {
        atlas: Arc::new(atlas),
        cell: 7,
      })
      .with(stage::Position(Point3::new(164.0, 164.0, 0.0)))
      .with(stage::Render)
      .with(unstable::Character {
        state: unstable::character::State::Idle,
        speed: 64.0,
      })
      .with(unstable::movement::Controlled)
      .build();

    // Set the camera target to the hero character.
    core
      .world
      .write_resource::<stage::Camera>()
      .set_target(entity);
  }

  Ok(())
}
