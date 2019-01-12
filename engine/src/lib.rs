// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(futures_api, drain_filter, arbitrary_self_types, const_fn)]

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
  pub fn execute<R>(&self, func: impl FnOnce(&Context) -> R) -> R {
    let mut engine = self.0.borrow();

    func(&mut engine)
  }

  pub fn execute_mut<R>(&self, func: impl FnOnce(&mut Context) -> R) -> R {
    let mut engine = self.0.borrow_mut();

    func(&mut engine)
  }
}

pub fn start<F>(main: impl FnOnce(EngineHandle) -> F)
where
  F: Future<Output = ()> + 'static,
{
  let mut engine = Context {
    world: specs::World::new(),
  };

  log::setup(&mut engine);
  time::setup(&mut engine);

  let engine = EngineHandle(Rc::new(RefCell::new(engine)));

  let mut process_executor = process::Executor::new(&engine);
  let mut rate_limiter = time::RateLimiter::new();

  let main = main(engine.clone());

  engine.execute(|ctx| process::spawn(ctx, main));

  loop {
    rate_limiter.begin();

    engine.execute(time::tick);
    process_executor.tick();
    engine.execute_mut(Context::maintain);

    rate_limiter.wait_for_full_duration(time::Duration::from_hz(60));
  }
}
