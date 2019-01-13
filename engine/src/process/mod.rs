// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod atomic_wake;
mod next_tick;
mod process_list;

use self::atomic_wake::*;
pub use self::next_tick::*;
pub use self::process_list::*;
use super::EngineHandle;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::LocalWaker;
use std::task::Poll;

pub fn spawn(engine: &EngineHandle, future: impl Future<Output = ()> + 'static) {
  engine.execute(|ctx| {
    let mut processes = ctx.fetch_resource_mut::<ProcessList>();

    processes.spawn(future);
  });
}

pub fn spawn_fn<F>(engine: &EngineHandle, func: impl FnOnce(EngineHandle) -> F)
where
  F: Future<Output = ()> + 'static,
{
  spawn(engine, func(engine.clone()));
}

pub(crate) fn init(engine: &EngineHandle) {
  engine.execute_mut(|ctx| {
    ctx.ensure_resource::<ProcessList>();
  });
}

pub(crate) fn tick_all(engine: &EngineHandle) {
  let mut processes = engine.execute_mut(|ctx| {
    let process_list = ctx.get_resource::<ProcessList>();

    process_list.acquire()
  });

  processes.drain_filter(|process| {
    if !process.is_awake() {
      return false;
    }

    match process.poll() {
      Poll::Ready(_) => true,
      Poll::Pending => false,
    }
  });

  engine.execute_mut(|ctx| {
    let process_list = ctx.get_resource::<ProcessList>();

    process_list.release(processes);
  })
}

struct Process {
  future: Pin<Box<dyn Future<Output = ()>>>,
  wake: Arc<AtomicWake>,
  local_waker: LocalWaker,
}

impl Process {
  pub fn is_awake(&self) -> bool {
    self.wake.is_awake()
  }

  pub fn poll(&mut self) -> Poll<()> {
    self.wake.reset();
    self.future.as_mut().poll(&self.local_waker)
  }
}

// A `Process` is temporarily stored in the `Processes` resource until it is
// taken by an `Executor` on the next frame. Because the `Process` isn't
// accessed or modified in the meantime, it can be safely treated as
// `Send + Sync` even though it is neither.
unsafe impl Send for Process {}
unsafe impl Sync for Process {}
