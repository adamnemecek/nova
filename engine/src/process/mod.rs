// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::EngineHandle;
use std::future::Future;
use std::pin::Pin;

mod atomic_wake;
mod executor;
mod next_tick;

pub use self::executor::*;
pub use self::next_tick::*;

#[derive(Default)]
pub struct Processes {
  pending: Vec<Process>,
}

impl Processes {
  pub fn start(&mut self, process: impl Future<Output = ()> + 'static) {
    self.pending.push(Process(Box::pin(process)));
  }
}

struct Process(Pin<Box<dyn Future<Output = ()>>>);

// A `Process` is temporarily stored in the `Processes` resource until it is
// taken by an `Executor` on the next frame. Because the `Process` isn't
// accessed or modified in the meantime, it can be safely treated as
// `Send + Sync` even though it is neither.
unsafe impl Send for Process {}
unsafe impl Sync for Process {}

pub fn spawn_fn<F>(engine: &EngineHandle, process: impl FnOnce(EngineHandle) -> F)
where
  F: Future<Output = ()> + 'static,
{
  let future = process(engine.clone());

  spawn(&engine, future);
}

pub fn spawn(engine: &EngineHandle, future: impl Future<Output = ()> + 'static) {
  engine.execute(|ctx| {
    Processes::start(&mut ctx.fetch_resource_mut(), future);
  });
}
