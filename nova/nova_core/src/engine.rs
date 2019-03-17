// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::clock;
use crate::ecs;
use crate::ecs::entities::{self, Entity};
use crate::ecs::resources::Resources;
use crate::scheduler::{Runnable, Scheduler};
use crate::ThreadPool;
use std::fmt;

const PHASES: usize = EnginePhase::AfterUpdate as usize + 1;

#[repr(usize)]
pub enum EnginePhase {
  BeforeUpdate,
  Update,
  AfterUpdate,
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

    ecs::setup(&mut engine);

    clock::Time::setup(&mut engine.resources);

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

  pub fn tick(&mut self, delta_time: clock::DeltaTime) {
    // Maintain entities in case of out-of-tick changes.
    entities::maintain(&mut self.resources, &mut self.entity_buffer);

    self.run_phase(EnginePhase::BeforeUpdate);

    clock::Time::update(&mut self.resources.fetch_mut(), delta_time);

    self.run_phase(EnginePhase::Update);
    self.run_phase(EnginePhase::AfterUpdate);
  }

  fn run_phase(&mut self, phase: EnginePhase) {
    self.phases[phase as usize].run(&self.resources, &self.thread_pool);

    entities::maintain(&mut self.resources, &mut self.entity_buffer);
  }
}

impl fmt::Debug for Engine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("Engine")
      .field("phases", &self.phases)
      .finish()
  }
}
