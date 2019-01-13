// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(async_await, await_macro, const_fn, drain_filter, futures_api)]

// TODO: Remove when RLS supports it.
extern crate derive_more;
extern crate specs;
extern crate specs_derive;

pub mod ecs;
mod handle;
pub mod log;
pub mod tasks;
pub mod time;

pub use self::handle::*;

pub fn create() -> EngineHandle {
  let _ = log::set_as_default();
  let engine = EngineHandle::new(ecs::Context::new());

  tasks::init(&engine);
  time::clocks::init(&engine);

  engine
}

pub fn tick(engine: &EngineHandle, delta_time: time::Duration) {
  tasks::tick_all(engine);

  ecs::maintain(engine);

  time::clocks::tick(engine, delta_time);
}
