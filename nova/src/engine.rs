// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod events;
mod resources;

use crate::assets;
use crate::clock;
use crate::ecs::{self, Dispatchable};
#[cfg(not(feature = "headless"))]
use crate::graphics;
#[cfg(not(feature = "headless"))]
use crate::ui;
#[cfg(not(feature = "headless"))]
use crate::window;

pub use self::events::*;
pub use self::resources::*;
pub use rayon::ThreadPool;

pub struct Engine {
  world: specs::World,
  thread_pool: ThreadPool,
  event_handlers: EventHandlers,
}

impl Engine {
  pub fn new(options: Options) -> Self {
    let thread_pool = rayon::ThreadPoolBuilder::new()
      .build()
      .expect("Could not create thread pool");

    let mut engine = Engine {
      world: specs::World::new(),
      thread_pool,
      event_handlers: EventHandlers::new(),
    };

    engine.add_dispatch(Event::ClockTimeUpdated, clock::UpdateTime::default());

    engine.world.res.insert(assets::OverlayFs::default());

    #[cfg(not(feature = "headless"))]
    {
      graphics::device::setup(engine.resources_mut());

      let update_window = window::setup(engine.resources_mut(), options.window);

      engine.add_dispatch(Event::TickStarted, update_window);

      ui::setup(engine.resources_mut());

      let mut renderer = graphics::Renderer::new(engine.resources_mut());

      engine.add_fn(Event::TickEnding, {
        move |res, _| {
          renderer.render(res);
        }
      });
    }

    engine
  }

  pub fn resources(&self) -> &Resources {
    &self.world.res
  }

  pub fn resources_mut(&mut self) -> &mut Resources {
    &mut self.world.res
  }

  #[deprecated]
  pub fn create_entity(&mut self) -> ecs::EntityBuilder {
    self.world.create_entity()
  }

  pub fn add_dispatch(
    &mut self,
    event: Event,
    mut dispatch: impl for<'a> Dispatchable<'a> + 'static,
  ) {
    dispatch.setup(&mut self.world.res);

    self
      .event_handlers
      .add(event, EventHandler::RunWithPool(Box::new(dispatch)));
  }

  pub fn add_fn(
    &mut self,
    event: Event,
    fn_mut: impl FnMut(&mut Resources, &ThreadPool) + 'static,
  ) {
    self
      .event_handlers
      .add(event, EventHandler::FnMut(Box::new(fn_mut)));
  }

  #[cfg(not(feature = "headless"))]
  pub fn run(mut self) {
    let mut reader = {
      let mut events = self.world.res.fetch_mut::<window::Events>();

      events.channel_mut().register_reader()
    };

    loop {
      self.tick();

      let events = self.world.res.fetch::<window::Events>();

      for event in events.channel().read(&mut reader) {
        if let window::Event::CloseRequested = event {
          return;
        }
      }
    }
  }

  pub fn tick(&mut self) {
    self.run_event_handlers(Event::TickStarted);
    self.run_event_handlers(Event::ClockTimeUpdated);
    self.run_event_handlers(Event::TickEnding);
  }

  fn run_event_handlers(&mut self, event: Event) {
    self
      .event_handlers
      .run(event, &mut self.world.res, &self.thread_pool);

    self.world.maintain();
  }
}

#[derive(Default)]
pub struct Options {
  pub window: window::Options,
}
