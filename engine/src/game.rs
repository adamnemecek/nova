// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use specs::prelude::*;

use prelude::*;

pub struct Game<'a, 'b> {
  pub world: World,
  pub platform: platform::Engine,
  pub time: time::Engine,
  pub graphics: graphics::Engine,
  dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
  pub fn new() -> Self {
    // Create a new world for entities and resources.
    let mut world = World::new();

    world.register::<motion::Position>();

    // Set up basic engines.
    let platform = platform::Engine::new(&mut world);
    let time = time::Engine::new(&mut world);
    let graphics = graphics::Engine::new(&mut world);

    // Create a new dispatcher for systems.
    let dispatcher = {
      let builder = DispatcherBuilder::new();

      builder.build()
    };

    Game {
      world,
      platform,
      time,
      graphics,
      dispatcher,
    }
  }

  /// Runs the main engine loop until quit.
  pub fn run(mut self) {
    while self.platform.ctx.continuing {
      // Update engines.
      self.platform.update(&self.world);
      self.time.update(&self.world, &mut self.platform);

      // Dispatch registered ECS systems.
      self.dispatcher.dispatch(&mut self.world.res);

      // Draw the frame.
      self
        .graphics
        .draw(&self.world, &mut self.platform)
        .expect("drawing failed");
    }
  }
}
