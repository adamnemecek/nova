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
  // Initialize the game.
  let mut game = Game::new()?;

  // Add a character to the world.
  {
    let atlas = Arc::new(graphics::Atlas::new(
      &mut game.platform.ctx,
      "/004-fire-salamander/atlas",
    )?);

    game
      .world
      .create_entity()
      .with(motion::Position {
        x: 100.0,
        y: 100.0,
        z: 0.0,
      })
      .with(graphics::Sprite { atlas, cell: 0 })
      .with(graphics::Drawable)
      .build();

    let atlas = Arc::new(graphics::Atlas::new(
      &mut game.platform.ctx,
      "/hero-f/atlas",
    )?);

    game
      .world
      .create_entity()
      .with(motion::Position {
        x: 164.0,
        y: 164.0,
        z: 0.0,
      })
      .with(graphics::Sprite { atlas, cell: 7 })
      .with(graphics::Drawable)
      .build();
  }

  // Run the main event loop.
  game.run()
}
