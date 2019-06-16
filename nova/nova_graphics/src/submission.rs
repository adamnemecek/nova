// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::cmd;
use crate::pipeline;
use crate::{QueueId, Semaphore};

pub struct Submission {
  pub queue_id: QueueId,
  pub command_buffers: Vec<cmd::Buffer>,
  pub wait_semaphores: Vec<(Semaphore, pipeline::Stage)>,
  pub signal_semaphores: Vec<Semaphore>,
}

impl Submission {
  pub fn new(queue_id: QueueId) -> Self {
    Self {
      queue_id,
      command_buffers: Vec::new(),
      wait_semaphores: Vec::new(),
      signal_semaphores: Vec::new(),
    }
  }

  pub fn clear(&mut self) {
    self.command_buffers.clear();
    self.wait_semaphores.clear();
    self.signal_semaphores.clear();
  }

  pub fn wait_for(&mut self, semaphore: &Semaphore, stage: pipeline::Stage) {
    self.wait_semaphores.push((semaphore.clone(), stage));
  }

  pub fn signal(&mut self, semaphore: &Semaphore) {
    self.signal_semaphores.push(semaphore.clone());
  }
}
