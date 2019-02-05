// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::ThreadPool;
use crate::ecs;

#[repr(usize)]
pub enum Event {
  Ticked,
}

const EVENT_COUNT: usize = Event::Ticked as usize + 1;

pub enum EventHandler {
  FnMut(Box<dyn FnMut(&mut ecs::Resources, &ThreadPool)>),
  RunWithPool(Box<dyn for<'a> ecs::Dispatchable<'a>>),
}

impl EventHandler {
  fn run(&mut self, res: &mut ecs::Resources, pool: &ThreadPool) {
    match self {
      EventHandler::FnMut(inner) => inner(res, pool),
      EventHandler::RunWithPool(inner) => inner.run(res, pool),
    }
  }
}

pub(super) struct EventHandlers([Vec<EventHandler>; EVENT_COUNT]);

impl EventHandlers {
  pub fn new() -> Self {
    EventHandlers([Vec::new(); EVENT_COUNT])
  }

  pub fn add(&mut self, event: Event, handler: EventHandler) {
    self.0[event as usize].push(handler);
  }

  pub fn run(&mut self, event: Event, res: &mut ecs::Resources, pool: &ThreadPool) {
    for handler in &mut self.0[event as usize] {
      handler.run(res, pool);
    }
  }
}
