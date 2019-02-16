// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod dispatch;

mod events;
mod resources;

use crate::assets;
use crate::clock;
use crate::el;
/*
#[cfg(not(feature = "headless"))]
use crate::graphics;
#[cfg(not(feature = "headless"))]
use crate::ui;
#[cfg(not(feature = "headless"))]
use crate::window;
*/

pub use self::events::*;
pub use self::resources::*;
pub use rayon::ThreadPool;

pub struct Engine {
  world: specs::World,
  thread_pool: ThreadPool,
  event_handlers: EventHandlers,
}

impl Engine {
  pub fn new() -> Self {
    let thread_pool = rayon::ThreadPoolBuilder::new()
      .build()
      .expect("Could not create thread pool");

    let mut engine = Engine {
      world: specs::World::new(),
      thread_pool,
      event_handlers: EventHandlers::new(),
    };

    el::setup(engine.resources_mut());

    engine.on_event(Event::ClockTimeUpdated, clock::UpdateTime::default());

    engine.world.res.insert(assets::OverlayFs::default());

    /*
    #[cfg(not(feature = "headless"))]
    {
      graphics::device::setup(engine.resources_mut());

      let update_window = window::setup(engine.resources_mut(), options.window);

      engine.on_event(Event::TickStarted, update_window);

      ui::setup(engine.resources_mut());

      let mut renderer = graphics::Renderer::new(engine.resources_mut());

      engine.on_event_fn(Event::TickEnding, {
        move |res, _| {
          renderer.render(res);
        }
      });
    }
    */

    engine
  }

  pub fn resources(&self) -> &Resources {
    &self.world.res
  }

  pub fn resources_mut(&mut self) -> &mut Resources {
    &mut self.world.res
  }

  pub fn add_element(&mut self, element: impl el::Element + 'static) {
    let res = self.resources();

    res.fetch_mut::<el::Hierarchy>().add_element(res, element);
  }

  pub fn on_event(
    &mut self,
    event: Event,
    mut dispatch: impl for<'a> dispatch::RunWithPool<'a> + 'static,
  ) {
    dispatch.setup(&mut self.world.res);

    self
      .event_handlers
      .add(event, EventHandler::RunWithPool(Box::new(dispatch)));
  }

  pub fn on_event_fn(
    &mut self,
    event: Event,
    fn_mut: impl FnMut(&mut Resources, &ThreadPool) + 'static,
  ) {
    self
      .event_handlers
      .add(event, EventHandler::FnMut(Box::new(fn_mut)));
  }

  /*
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
  */

  pub fn tick(&mut self) {
    self.world.maintain();

    self.run_event_handlers(Event::TickStarted);

    self
      .world
      .res
      .fetch_mut::<el::Hierarchy>()
      .deliver_messages(&self.world.res, &self.thread_pool);

    self.world.maintain();

    self.run_event_handlers(Event::ClockTimeUpdated);

    self
      .world
      .res
      .fetch_mut::<el::Hierarchy>()
      .build(&self.world.res, &self.thread_pool);

    self.world.maintain();

    self.run_event_handlers(Event::TickEnding);

    self.world.maintain();
  }

  fn run_event_handlers(&mut self, event: Event) {
    self
      .event_handlers
      .run(event, &mut self.world.res, &self.thread_pool);
  }
}

/*
#[derive(Default)]
pub struct Options {
  pub window: window::Options,
}
*/

impl Default for Engine {
  fn default() -> Self {
    Engine::new()
  }
}
