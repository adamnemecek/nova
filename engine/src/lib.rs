// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(async_await, await_macro, const_fn, drain_filter, futures_api)]

pub mod assets;
pub mod ecs;
pub mod engine;
#[cfg(feature = "graphics")]
pub mod graphics;
pub mod log;
pub mod math;
pub mod utils;
#[cfg(feature = "window")]
pub mod window;

pub mod events {
  pub use shrev::{Event, EventChannel as Channel, EventIterator, ReaderId};
}

pub use self::engine::Engine;
