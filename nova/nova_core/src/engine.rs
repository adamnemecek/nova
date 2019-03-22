// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::components;
use crate::entities::{self, Entity};
use crate::resources::Resources;
use crate::scheduler::{Runnable, Scheduler, ThreadPool};
use std::fmt;

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

    entities::setup(&mut engine.resources);
    components::setup(&mut engine.resources);

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
