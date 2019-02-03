// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::ecs::{self, Dispatchable};
#[cfg(feature = "graphics")]
use crate::graphics;
#[cfg(feature = "window")]
use crate::window;
use std::sync::Arc;

pub struct App {
  res: ecs::Resources,
  thread_pool: rayon::ThreadPool,
  on_tick: Vec<Box<dyn for<'a> Dispatchable<'a>>>,
  #[cfg(feature = "graphics")]
  gpu: graphics::Gpu,
  #[cfg(feature = "window")]
  window: Arc<window::Window>,
}

impl App {
  pub fn new(options: Options) -> Self {
    let thread_pool = rayon::ThreadPoolBuilder::new()
      .build()
      .expect("Could not create thread pool");
    #[cfg(feature = "window")]
    let (window, events_loop) = window::create(options.window);

    let mut app = App {
      res: ecs::setup(),
      thread_pool,
      on_tick: Vec::new(),
      #[cfg(feature = "graphics")]
      gpu: graphics::Gpu::new(),
      #[cfg(feature = "window")]
      window,
    };

    #[cfg(feature = "window")]
    app.on_tick(window::PollEvents { events_loop });

    app
  }

  pub fn on_tick(&mut self, mut handler: impl for<'a> Dispatchable<'a> + 'static) {
    handler.setup(&mut self.res);

    self.on_tick.push(Box::new(handler));
  }

  pub fn tick(&mut self) {
    for handler in &mut self.on_tick {
      handler.run(&self.res, &self.thread_pool);
    }
  }

  pub fn run(mut self) {
    loop {
      self.tick();
    }
  }
}

impl Default for App {
  fn default() -> App {
    App::new(Options::default())
  }
}

#[derive(Default)]
pub struct Options {
  #[cfg(feature = "window")]
  window: window::Options,
}
