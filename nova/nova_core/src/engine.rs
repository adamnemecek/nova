// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod dispatch;

mod events;
mod resources;

use crate::clock;
use crate::el;

pub use self::events::*;
pub use self::resources::*;
pub use rayon::ThreadPool;

pub struct Engine {
  world: specs::World,
  thread_pool: ThreadPool,
  event_handlers: EventHandlerList,
}

impl Default for Engine {
  fn default() -> Self {
    Engine::new()
  }
}

impl Engine {
  pub fn new() -> Self {
    let thread_pool = rayon::ThreadPoolBuilder::new()
      .build()
      .expect("Could not create thread pool");

    let mut engine = Engine {
      world: specs::World::new(),
      thread_pool,
      event_handlers: EventHandlerList::new(),
    };

    clock::Time::setup(engine.resources_mut());
    el::setup(&mut engine);

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

    self.event_handlers
      .add(event, EventHandler::RunWithPool(Box::new(dispatch)));
  }

  pub fn on_event_fn(
    &mut self,
    event: Event,
    fn_mut: impl FnMut(&mut Resources, &ThreadPool) + 'static,
  ) {
    self.event_handlers
      .add(event, EventHandler::FnMut(Box::new(fn_mut)));
  }

  pub fn tick(&mut self, delta_time: clock::DeltaTime) {
    self.world.maintain();

    self.run_event_handlers(Event::TickStarted);

    el::Hierarchy::deliver_messages(&mut self.world.res.fetch_mut(), &self.world.res);

    self.world.maintain();

    clock::Time::update(&mut self.world.res.fetch_mut(), delta_time);

    self.run_event_handlers(Event::ClockTimeUpdated);

    el::Hierarchy::build(&mut self.world.res.fetch_mut(), &self.world.res);

    self.world.maintain();

    self.run_event_handlers(Event::TickEnding);

    self.world.maintain();
  }

  fn run_event_handlers(&mut self, event: Event) {
    self.event_handlers
      .run(event, &mut self.world.res, &self.thread_pool);
  }
}
