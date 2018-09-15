// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ggez::conf::{WindowMode, WindowSetup};
use specs::prelude::*;
use std::env;
use std::error::Error;
use std::path::PathBuf;

use prelude::*;

pub struct Game<'a, 'b> {
  pub world: World,
  pub platform: platform::Engine,
  pub time: time::Engine,
  pub graphics: graphics::Engine,
  dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
  pub fn new() -> Result<Self, Box<dyn Error>> {
    // Create a new world for entities and resources.
    let mut world = World::new();

    world.register::<motion::Position>();

    // Create a new ggez context and winit events loop.
    let (ctx, events_loop) = {
      let mut builder = ggez::ContextBuilder::new("nova", "bfrydl")
        // Create a resizable window with vsync disabled.
        .window_mode(WindowMode::default().resizable(true))
        .window_setup(WindowSetup::default().title("nova").vsync(false));

      // Add the resources dir for development.
      if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);

        path.push("resources");
        builder = builder.add_resource_path(path);
      }

      builder.build()?
    };

    // Set up basic engines.
    let platform = platform::Engine::new(&mut world, ctx, events_loop);
    let time = time::Engine::new(&mut world);
    let graphics = graphics::Engine::new(&mut world);

    // Create a new dispatcher for systems.
    let dispatcher = {
      let builder = DispatcherBuilder::new();

      builder.build()
    };

    Ok(Game {
      world,
      platform,
      time,
      graphics,
      dispatcher,
    })
  }

  /// Runs the main engine loop until quit.
  pub fn run(mut self) -> Result<(), Box<dyn Error>> {
    while self.platform.ctx.continuing {
      // Update engines.
      self.platform.update(&self.world);
      self.time.update(&self.world, &mut self.platform);

      // Dispatch registered ECS systems.
      self.dispatcher.dispatch(&mut self.world.res);

      // Draw the frame.
      self.graphics.draw(&self.world, &mut self.platform)?;
    }

    Ok(())
  }
}
