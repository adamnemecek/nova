// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{AtomicWake, Process};
use std::future::Future;
use std::sync::Arc;
use std::task;

pub struct ProcessList {
  spawning: Vec<Process>,
  running: Option<Vec<Process>>,
}

impl ProcessList {
  pub fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
    let wake = Arc::new(AtomicWake::new());
    let local_waker = task::local_waker_from_nonlocal(wake.clone());

    self.spawning.push(Process {
      future: Box::pin(future),
      wake,
      local_waker,
    });
  }

  pub(super) fn acquire(&mut self) -> Vec<Process> {
    let mut running = self
      .running
      .take()
      .expect("The process list has already been acquired.");

    running.extend(self.spawning.drain(..));
    running
  }

  pub(super) fn release(&mut self, running: Vec<Process>) {
    self.running = Some(running);
  }
}

impl Default for ProcessList {
  fn default() -> Self {
    ProcessList {
      spawning: Vec::new(),
      running: Some(Vec::new()),
    }
  }
}
