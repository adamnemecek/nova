// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::ecs::{self, Dispatchable};
#[cfg(feature = "graphics")]
use crate::graphics;
#[cfg(feature = "window")]
use crate::window;
use std::sync::Arc;

pub use rayon::ThreadPool;

pub struct Engine {
  res: ecs::Resources,
  thread_pool: ThreadPool,
  on_tick: Vec<Box<dyn for<'a> Dispatchable<'a>>>,
  #[cfg(feature = "graphics")]
  gpu: graphics::Gpu,
  #[cfg(feature = "window")]
  window: Arc<window::Window>,
}

impl Engine {
  pub fn new(options: Options) -> Self {
    let thread_pool = rayon::ThreadPoolBuilder::new()
      .build()
      .expect("Could not create thread pool");

    #[cfg(feature = "window")]
    let (window, events_loop) = window::create(options.window);

    let mut app = Engine {
      res: ecs::setup(),
      thread_pool,
      on_tick: Vec::new(),
      #[cfg(feature = "graphics")]
      gpu: graphics::Gpu::new(),
      #[cfg(feature = "window")]
      window,
    };

    #[cfg(feature = "window")]
    app.on_tick(ecs::seq![
      window::PollEvents { events_loop },
      window::ExitEngineLoopOnCloseRequest::default(),
    ]);

    app
  }

  pub fn on_tick(&mut self, mut dispatch: impl for<'a> Dispatchable<'a> + 'static) {
    dispatch.setup(&mut self.res);

    self.on_tick.push(Box::new(dispatch));
  }

  pub fn tick(&mut self) {
    for dispatchable in &mut self.on_tick {
      dispatchable.run(&self.res, &self.thread_pool);
    }

    ecs::maintain(&mut self.res);
  }

  pub fn run_loop(mut self) {
    self.res.entry().or_insert_with(LoopExit::default).requested = false;

    loop {
      self.tick();

      if self.res.get_mut::<LoopExit>().unwrap().requested {
        break;
      }
    }
  }
}

impl Default for Engine {
  fn default() -> Engine {
    Engine::new(Options::default())
  }
}

#[derive(Default)]
pub struct Options {
  #[cfg(feature = "window")]
  window: window::Options,
}

#[derive(Default)]
pub struct LoopExit {
  pub requested: bool,
}
