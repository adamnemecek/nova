// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(async_await, await_macro, const_fn, drain_filter, futures_api)]

// TODO: Remove when RLS supports it.
extern crate derive_more;
extern crate specs;
extern crate specs_derive;

use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;

pub mod ecs;
pub mod log;
pub mod process;
pub mod time;

mod context;

pub use self::context::*;

#[derive(Clone)]
pub struct EngineHandle(Rc<RefCell<Context>>);

impl EngineHandle {
  pub(crate) fn new(ctx: Context) -> Self {
    EngineHandle(Rc::new(RefCell::new(ctx)))
  }

  pub fn execute(&self, func: impl FnOnce(&Context)) {
    let mut engine = self.0.borrow();

    func(&mut engine);
  }

  pub fn execute_mut(&self, func: impl FnOnce(&mut Context)) {
    let mut engine = self.0.borrow_mut();

    func(&mut engine);
  }
}

pub fn start<F>(main: impl FnOnce(EngineHandle) -> F)
where
  F: Future<Output = ()> + 'static,
{
  let _ = log::set_as_default();

  let engine = EngineHandle::new(Context::new());

  let mut rate_limiter = time::RateLimiter::new();
  let mut process_executor = process::Executor::new(&engine);

  process::spawn_fn(&engine, main);

  loop {
    rate_limiter.begin();

    engine.execute_mut(|ctx| {
      let mut clock = ctx.fetch_resource_mut::<time::Clock>();
      let settings = ctx.fetch_resource_mut::<time::Settings>();

      clock.tick(&settings);
    });

    process_executor.tick();

    engine.execute_mut(|ctx| {
      ctx.maintain();
    });

    rate_limiter.wait_until(time::Duration::from_hz(60));
  }
}
