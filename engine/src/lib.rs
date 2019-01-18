// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(async_await, await_macro, const_fn, drain_filter, futures_api)]

// TODO: Remove when RLS supports it.
extern crate crossbeam;
extern crate derive_more;
extern crate specs;
extern crate specs_derive;
extern crate winit;

pub mod assets;
pub mod ecs;
pub mod log;
pub mod math;
pub mod time;
pub mod window;

pub mod events {
  pub use shrev::{Event, EventChannel as Channel, EventIterator, ReaderId};
}

pub mod thread {
  pub use rayon::{ThreadPool, ThreadPoolBuilder};

  use crate::time;

  pub fn create_pool() -> ThreadPool {
    ThreadPoolBuilder::new()
      .build()
      .expect("Could not create ThreadPool")
  }

  pub fn sleep(duration: time::Duration) {
    std::thread::sleep(duration.into())
  }
}
