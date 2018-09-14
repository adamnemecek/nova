// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate ggez;
extern crate nova_engine;
extern crate specs;

use nova_engine::{core, graphics, sprites, Engine};
use specs::prelude::*;
use std::sync::Arc;

/// Main entry point of the program.
pub fn main() -> Result<(), Box<std::error::Error>> {
  // Initialize the enigne.
  let mut engine = Engine::new()?;

  // Add a character to the world.
  {
    let atlas = Arc::new(sprites::Atlas::new(
      &mut engine.ctx,
      "/004-fire-salamander/atlas",
    )?);

    engine
      .world
      .create_entity()
      .with(core::Position {
        x: 100.0,
        y: 100.0,
        z: 0.0,
      })
      .with(sprites::Sprite { atlas, frame: 0 })
      .with(graphics::Drawable)
      .build();

    let atlas = Arc::new(sprites::Atlas::new(&mut engine.ctx, "/hero-f/atlas")?);

    engine
      .world
      .create_entity()
      .with(core::Position {
        x: 164.0,
        y: 164.0,
        z: 0.0,
      })
      .with(sprites::Sprite { atlas, frame: 7 })
      .with(graphics::Drawable)
      .build();
  }

  // Run the main event loop.
  engine.run()
}
