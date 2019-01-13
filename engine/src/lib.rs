// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(async_await, await_macro, const_fn, drain_filter, futures_api)]

// TODO: Remove when RLS supports it.
extern crate derive_more;
extern crate specs;
extern crate specs_derive;

use std::cell::RefCell;
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

  pub fn execute<R>(&self, func: impl FnOnce(&Context) -> R) -> R {
    let mut engine = self.0.borrow();

    func(&mut engine)
  }

  pub fn execute_mut<R>(&self, func: impl FnOnce(&mut Context) -> R) -> R {
    let mut engine = self.0.borrow_mut();

    func(&mut engine)
  }

  pub fn tick(&self) {
    process::tick_all(&self);

    self.execute_mut(Context::maintain)
  }
}

pub fn init() -> EngineHandle {
  let _ = log::set_as_default();
  let engine = EngineHandle::new(Context::new());

  process::init(&engine);

  engine
}
