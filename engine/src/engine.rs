// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod events;
mod options;

use crate::ecs::{self, Dispatchable};

#[cfg(feature = "graphics")]
use crate::graphics;

#[cfg(feature = "window")]
use crate::window;

pub use self::events::*;
pub use self::options::*;
pub use rayon::ThreadPool;

pub struct Engine {
  resources: ecs::Resources,
  thread_pool: ThreadPool,
  event_handlers: EventHandlers,
}

impl Engine {
  pub fn new(options: Options) -> Self {
    let thread_pool = rayon::ThreadPoolBuilder::new()
      .build()
      .expect("Could not create thread pool");

    let mut engine = Engine {
      resources: ecs::setup(),
      thread_pool,
      event_handlers: EventHandlers::new(),
    };

    #[cfg(feature = "graphics")]
    {
      if options.graphics {
        graphics::setup(&mut engine.resources);
      }
    }

    #[cfg(feature = "window")]
    {
      if let Some(window_options) = options.window {
        let update = window::setup(&mut engine.resources, window_options);

        engine.add_dispatch(Event::Ticked, update);
      }
    }

    engine
  }

  pub fn resources(&self) -> &ecs::Resources {
    &self.resources
  }

  pub fn resources_mut(&mut self) -> &mut ecs::Resources {
    &mut self.resources
  }

  pub fn add_dispatch(
    &mut self,
    event: Event,
    mut dispatch: impl for<'a> Dispatchable<'a> + 'static,
  ) {
    dispatch.setup(&mut self.resources);

    self
      .event_handlers
      .add(event, EventHandler::RunWithPool(Box::new(dispatch)));
  }

  pub fn add_fn(
    &mut self,
    event: Event,
    fn_mut: impl FnMut(&mut ecs::Resources, &ThreadPool) + 'static,
  ) {
    self
      .event_handlers
      .add(event, EventHandler::FnMut(Box::new(fn_mut)));
  }

  pub fn tick(&mut self) {
    self
      .event_handlers
      .run(Event::Ticked, &mut self.resources, &self.thread_pool);

    ecs::maintain(&mut self.resources);
  }

  pub fn run(mut self) {
    self
      .resources
      .entry()
      .or_insert_with(Exit::default)
      .requested = false;

    loop {
      self.tick();

      if self.resources.get_mut::<Exit>().unwrap().requested {
        break;
      }

      std::thread::sleep(std::time::Duration::from_millis(10));
    }
  }
}

impl Default for Engine {
  fn default() -> Engine {
    Engine::new(Options::default())
  }
}

#[derive(Default)]
pub struct Exit {
  pub requested: bool,
}
