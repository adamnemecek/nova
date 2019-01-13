// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{AtomicWake, Task};
use std::future::Future;
use std::sync::Arc;
use std::task;

pub struct TaskList {
  spawning: Vec<Task>,
  running: Option<Vec<Task>>,
}

impl TaskList {
  pub fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
    let wake = Arc::new(AtomicWake::new());
    let local_waker = task::local_waker_from_nonlocal(wake.clone());

    self.spawning.push(Task {
      future: Box::pin(future),
      wake,
      local_waker,
    });
  }

  pub(super) fn acquire(&mut self) -> Vec<Task> {
    let mut running = self
      .running
      .take()
      .expect("The task list has already been acquired.");

    running.extend(self.spawning.drain(..));
    running
  }

  pub(super) fn release(&mut self, running: Vec<Task>) {
    self.running = Some(running);
  }
}

impl Default for TaskList {
  fn default() -> Self {
    TaskList {
      spawning: Vec::new(),
      running: Some(Vec::new()),
    }
  }
}
