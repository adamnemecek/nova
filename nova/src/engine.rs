// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod events;

use crate::assets;
use crate::clock;
use crate::ecs::{self, Dispatchable};
#[cfg(not(feature = "headless"))]
use crate::graphics;
#[cfg(not(feature = "headless"))]
use crate::window;

pub use self::events::*;
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

    engine.add_dispatch(Event::ClockTimeUpdated, clock::UpdateTime::default());

    engine.resources.insert(assets::OverlayFs::default());

    #[cfg(not(feature = "headless"))]
    {
      graphics::device::setup(engine.resources_mut());

      let update_window = window::setup(engine.resources_mut(), options.window);

      engine.add_dispatch(Event::TickStarted, update_window);

      let mut renderer = graphics::render::Renderer::new(engine.resources_mut());

      engine.add_fn(Event::TickEnding, {
        move |res, _| {
          renderer.render(res);
        }
      });
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

  #[cfg(not(feature = "headless"))]
  pub fn run(mut self) {
    let mut reader = {
      let mut events = self.resources.fetch_mut::<window::Events>();

      events.channel_mut().register_reader()
    };

    loop {
      self.tick();

      let events = self.resources.fetch::<window::Events>();

      for event in events.channel().read(&mut reader) {
        if let window::Event::CloseRequested = event {
          return;
        }
      }
    }
  }

  pub fn tick(&mut self) {
    self.run_event_handlers(Event::TickStarted);

    ecs::maintain(&mut self.resources);

    self.run_event_handlers(Event::ClockTimeUpdated);

    ecs::maintain(&mut self.resources);

    self.run_event_handlers(Event::TickEnding);

    ecs::maintain(&mut self.resources);
  }

  fn run_event_handlers(&mut self, event: Event) {
    self
      .event_handlers
      .run(event, &mut self.resources, &self.thread_pool);
  }
}

#[derive(Default)]
pub struct Options {
  pub window: window::Options,
}
