// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::ecs::{self, Dispatchable};
use crate::log;

#[cfg(feature = "graphics")]
use crate::graphics;

#[cfg(feature = "window")]
use crate::window;

pub use rayon::ThreadPool;

pub struct Engine {
  res: ecs::Resources,
  thread_pool: ThreadPool,
  on_tick: Vec<Box<dyn for<'a> Dispatchable<'a>>>,
  log: log::Logger,
}

impl Engine {
  pub fn new(options: Options) -> Self {
    let log = log::Logger::new("nova::Engine");

    let thread_pool = rayon::ThreadPoolBuilder::new()
      .build()
      .expect("Could not create thread pool");

    let mut engine = Engine {
      res: ecs::setup(),
      thread_pool,
      on_tick: Vec::new(),
      log,
    };

    #[cfg(feature = "graphics")]
    {
      if options.graphics {
        let gpu = graphics::setup();

        engine
          .log
          .info("Initialized graphics.")
          .with("backend", graphics::backend::NAME)
          .with("adapter", gpu.device().adapter_info());

        engine.res.insert(gpu);
      }
    }

    #[cfg(feature = "window")]
    {
      if let Some(options) = options.window {
        let (handle, events_loop) = window::setup(options);

        engine.on_tick(ecs::seq![
          window::PollEvents { events_loop },
          window::StopEngineOnCloseRequest::default(),
        ]);

        engine.log.info("Initialized window.");

        engine.res.insert(handle);
      }
    }

    engine
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

  pub fn start(mut self) {
    self.res.entry().or_insert_with(Stop::default).requested = false;

    loop {
      self.tick();

      if self.res.get_mut::<Stop>().unwrap().requested {
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

pub struct Options {
  #[cfg(feature = "graphics")]
  graphics: bool,
  #[cfg(feature = "window")]
  window: Option<window::Options>,
}

impl Default for Options {
  fn default() -> Self {
    Options {
      #[cfg(feature = "graphics")]
      graphics: true,
      #[cfg(feature = "window")]
      window: Some(Default::default()),
    }
  }
}

#[derive(Default)]
pub struct Stop {
  pub requested: bool,
}
