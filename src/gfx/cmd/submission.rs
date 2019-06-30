// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Describes a submission of one or more command buffers to a command queue on
/// the graphics device.
pub struct Submission<'a, F> {
  /// ID of the command queue to submit to.
  pub queue_id: QueueId,

  /// Command lists to submit to the queue.
  pub lists: &'a [&'a List],

  /// Semaphores to wait on before executing the commands in the submission.
  pub wait_semaphores: &'a [(&'a Semaphore, pipeline::Stage)],

  /// Semaphores to signal when the commands in the submission have finished
  /// executing.
  pub signal_semaphores: &'a [&'a Semaphore],

  /// Fence to signal when the commands in the submission have finished
  /// executing.
  pub fence: F,
}
