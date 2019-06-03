// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::components;
use crate::entities::{self, Entity};
use crate::resources::Resources;
use crate::scheduler::{Runnable, Scheduler, ThreadPool};
use crossbeam::channel;
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct EngineHandle {
  mutex: Arc<Mutex<Engine>>,
  channel: channel::Sender<EngineMessage>,
}

impl EngineHandle {
  pub fn lock(&mut self) -> MutexGuard<Engine> {
    self.mutex.lock().expect("failed to lock engine mutex")
  }

  pub fn execute(&mut self, func: impl FnOnce(&mut Engine) + 'static) {
    self
      .channel
      .send(EngineMessage::ExecuteFunction(Box::new(func)))
      .expect("failed to send function on engine channel")
  }

  pub fn query<F, R>(&mut self, func: F) -> R
  where
    F: FnOnce(&mut Engine) -> R + Send + 'static,
    R: Send + 'static,
  {
    let (sender, receiver) = channel::bounded(0);

    self.execute(move |engine| {
      sender
        .send(func(engine))
        .expect("failed to send result of query")
    });

    receiver.recv().expect("failed to receive result of query")
  }
}

enum EngineMessage {
  ExecuteFunction(Box<dyn FnOnce(&mut Engine)>),
}

pub struct Engine {
  pub resources: Resources,
  pub thread_pool: ThreadPool,
  phases: [Scheduler; PHASES],
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
      phases: Default::default(),
      entity_buffer: Default::default(),
    };

    entities::set_up(&mut engine.resources);
    components::set_up(&mut engine.resources);

    engine
  }

  pub fn schedule(
    &mut self,
    phase: EnginePhase,
    runnable: impl for<'a> Runnable<'a> + Send + 'static,
  ) {
    self.phases[phase as usize].add(runnable);
  }

  pub fn schedule_seq(
    &mut self,
    phase: EnginePhase,
    runnable: impl for<'a> Runnable<'a> + 'static,
  ) {
    self.phases[phase as usize].add_seq(runnable);
  }

  pub fn tick(&mut self) {
    // Maintain entities in case of out-of-tick changes.
    entities::maintain(&mut self.resources, &mut self.entity_buffer);

    for phase in &mut self.phases {
      phase.run(&self.resources, &self.thread_pool);

      entities::maintain(&mut self.resources, &mut self.entity_buffer);
    }
  }
}

impl fmt::Debug for Engine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("Engine")
      .field("phases", &self.phases)
      .finish()
  }
}

#[repr(usize)]
pub enum EnginePhase {
  BeforeUpdate,
  Update,
  AfterUpdate,
}

const PHASES: usize = EnginePhase::AfterUpdate as usize + 1;
