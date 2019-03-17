// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod dispatch;

pub use rayon::ThreadPool;

use crate::clock;
use crate::ecs::entities::{self, Entities, Entity};
use crate::ecs::Resources;
use crate::scheduler::{Runnable, Scheduler};
use std::fmt;

const EVENT_COUNT: usize = EngineEvent::TickEnding as usize + 1;

#[repr(usize)]
pub enum EngineEvent {
  TickStarted,
  ClockTimeUpdated,
  TickEnding,
}

pub struct Engine {
  pub resources: Resources,
  pub thread_pool: ThreadPool,
  schedulers: [Scheduler; EVENT_COUNT],
  entity_buffer: Vec<Entity>,
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
      resources: Resources::new(),
      thread_pool,
      schedulers: Default::default(),
      entity_buffer: Default::default(),
    };

    engine.resources.insert(Entities::default());

    clock::Time::setup(&mut engine.resources);

    engine
  }

  pub fn on_event(&mut self, event: EngineEvent, runnable: impl Runnable + Send + 'static) {
    self.schedulers[event as usize].add(runnable);
  }

  pub fn on_event_seq(&mut self, event: EngineEvent, runnable: impl Runnable + 'static) {
    self.schedulers[event as usize].add_seq(runnable);
  }

  pub fn tick(&mut self, delta_time: clock::DeltaTime) {
    entities::maintain(&mut self.resources, &mut self.entity_buffer);

    self.run_scheduler(EngineEvent::TickStarted);

    entities::maintain(&mut self.resources, &mut self.entity_buffer);

    clock::Time::update(&mut self.resources.fetch_mut(), delta_time);

    self.run_scheduler(EngineEvent::ClockTimeUpdated);

    entities::maintain(&mut self.resources, &mut self.entity_buffer);

    self.run_scheduler(EngineEvent::TickEnding);

    entities::maintain(&mut self.resources, &mut self.entity_buffer);
  }

  fn run_scheduler(&mut self, event: EngineEvent) {
    self.schedulers[event as usize].run(&self.resources, &self.thread_pool);
  }
}

impl fmt::Debug for Engine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("Engine").field("schedulers", &self.schedulers).finish()
  }
}
