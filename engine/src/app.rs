// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::engine::{self, Engine};
#[cfg(not(feature = "headless"))]
use crate::graphics;
#[cfg(not(feature = "headless"))]
use crate::window;

pub struct App {
  engine: Engine,
}

impl App {
  pub fn new(options: Options) -> Self {
    let mut engine = Engine::new();

    #[cfg(not(feature = "headless"))]
    {
      graphics::setup(engine.resources_mut());

      let update_window = window::setup(engine.resources_mut(), options.window);

      engine.add_dispatch(engine::Event::TickStarted, update_window);

      let mut renderer = graphics::render::Renderer::new(engine.resources_mut());

      engine.add_fn(engine::Event::TickEnding, {
        move |res, _| {
          renderer.render(res);
        }
      });
    }

    App { engine }
  }

  pub fn engine_mut(&mut self) -> &mut Engine {
    &mut self.engine
  }

  pub fn run(mut self) {
    let mut reader = {
      let mut events = self.engine.resources().fetch_mut::<window::Events>();

      events.channel_mut().register_reader()
    };

    loop {
      self.engine.tick();

      let events = self.engine.resources().fetch::<window::Events>();

      for event in events.channel().read(&mut reader) {
        if let window::Event::CloseRequested = event {
          return;
        }
      }

      std::thread::sleep(std::time::Duration::from_millis(10));
    }
  }
}

impl Default for App {
  fn default() -> Self {
    App::new(Options::default())
  }
}

#[derive(Default)]
pub struct Options {
  pub window: window::Options,
}
