// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate ggez;
extern crate nova_engine;
extern crate specs;

use nova_engine::prelude::*;
use specs::prelude::*;
use std::error::Error;
use std::sync::Arc;

/// Main entry point of the program.
pub fn main() -> Result<(), Box<dyn Error>> {
  let mut core = Core::new(core::context::build("nova", "bfrydl"));
  let mut stage = Stage::new(&mut core);
  let mut fps_display = core::FpsDisplay::default();

  // Add a character to the world.
  {
    let atlas = Arc::new(graphics::Atlas::load(
      &mut core,
      "/004-fire-salamander/atlas",
    )?);

    core
      .world
      .create_entity()
      .with(graphics::Sprite { atlas, cell: 0 })
      .with(stage::Position {
        x: 100.0,
        y: 100.0,
        z: 0.0,
      })
      .with(stage::Drawable)
      .build();

    let atlas = Arc::new(graphics::Atlas::load(&mut core, "/hero-f/atlas")?);

    core
      .world
      .create_entity()
      .with(graphics::Sprite { atlas, cell: 7 })
      .with(stage::Position {
        x: 164.0,
        y: 164.0,
        z: 0.0,
      })
      .with(stage::Drawable)
      .build();
  }

  // Run the main event loop.
  while core.is_running() {
    core.update();
    fps_display.update(&core);

    stage.draw(&mut core);
    fps_display.draw(&mut core);
  }

  Ok(())
}
