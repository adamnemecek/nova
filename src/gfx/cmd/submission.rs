// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Describes a submission of one or more command buffers to a command queue on
/// the graphics device.
pub struct Submission {
  /// ID of the command queue to submit to.
  pub queue_id: QueueId,

  /// Command lists to submit to the queue.
  pub command_buffers: Vec<List>,

  /// Semaphores to wait on before executing the commands in the submission.
  pub wait_semaphores: Vec<(Semaphore, render::PipelineStage)>,

  /// Semaphores to signal when the commands in the submission have finished
  /// executing.
  pub signal_semaphores: Vec<Semaphore>,
}

impl Submission {
  /// Creates a new, empty submission for the given queue ID.
  pub fn new(queue_id: QueueId) -> Self {
    Self {
      queue_id,
      command_buffers: Vec::new(),
      wait_semaphores: Vec::new(),
      signal_semaphores: Vec::new(),
    }
  }

  /// Clears the submission for reuse.
  pub fn clear(&mut self) {
    self.command_buffers.clear();
    self.wait_semaphores.clear();
    self.signal_semaphores.clear();
  }

  /// Adds a semaphore to the list of semaphores to wait on before executing
  /// commands in the submission.
  ///
  /// The given `stage` indicates which pipeline stage will wait for the
  /// semaphore.
  pub fn wait_for(&mut self, semaphore: &Semaphore, stage: render::PipelineStage) {
    self.wait_semaphores.push((semaphore.clone(), stage));
  }

  /// Adds a semaphore to the list of semaphores to signal after executing the
  /// commands in the submission.
  pub fn signal(&mut self, semaphore: &Semaphore) {
    self.signal_semaphores.push(semaphore.clone());
  }
}
