// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::dispatch;
use super::{Resources, ThreadPool};
use std::iter;

#[repr(usize)]
pub enum EngineEvent {
  TickStarted,
  ClockTimeUpdated,
  TickEnding,
}

const EVENT_COUNT: usize = EngineEvent::TickEnding as usize + 1;

pub enum EventHandler {
  FnMut(Box<dyn FnMut(&mut Resources, &ThreadPool)>),
  RunWithPool(Box<dyn for<'a> dispatch::RunWithPool<'a>>),
}

impl EventHandler {
  fn run(&mut self, res: &mut Resources, pool: &ThreadPool) {
    match self {
      EventHandler::FnMut(inner) => inner(res, pool),
      EventHandler::RunWithPool(inner) => inner.run(res, pool),
    }
  }
}

pub(crate) struct EventHandlerList(Vec<Vec<EventHandler>>);

impl EventHandlerList {
  pub fn new() -> Self {
    EventHandlerList(iter::repeat_with(Vec::new).take(EVENT_COUNT).collect())
  }

  pub fn add(&mut self, event: EngineEvent, handler: EventHandler) {
    self.0[event as usize].push(handler);
  }

  pub fn run(&mut self, event: EngineEvent, res: &mut Resources, pool: &ThreadPool) {
    for handler in &mut self.0[event as usize] {
      handler.run(res, pool);
    }
  }
}
