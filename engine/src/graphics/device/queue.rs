// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::queue::RawSubmission;

use super::Device;
use crate::graphics::backend;
use crate::graphics::commands::Submission;
use crate::graphics::hal::prelude::*;
use crate::graphics::Fence;
use smallvec::SmallVec;
use std::sync::Arc;

/// A device queue for submitting [`Commands`] or presenting [`Swapchain`]
/// images.
pub struct Queue {
  /// Raw backend queue family information.
  family: backend::QueueFamily,
  /// Raw backend queue structure.
  raw: backend::CommandQueue,
  /// Device the queue was created with.
  device: Arc<Device>,
}

impl Queue {
  /// Creates a new queue from the given raw backend structures.
  ///
  /// Unsafe because this function does not verify that the given queues belong
  /// to the device.
  pub unsafe fn from_raw(
    device: &Arc<Device>,
    queues: &mut hal::queue::Queues,
    family: backend::QueueFamily,
  ) -> Self {
    let raw = queues
      .take_raw(family.id())
      .expect("Expected device queue family was missing.")
      .into_iter()
      .next()
      .expect("Expected device queue was missing.");

    Queue {
      family,
      raw,
      device: device.clone(),
    }
  }

  /// Gets a reference to the device the queue was created with.
  pub fn device(&self) -> &Arc<Device> {
    &self.device
  }

  /// Gets the ID of the queue family the queue belongs to.
  pub fn family_id(&self) -> usize {
    self.family.id().0
  }

  /// Gets a mutable reference to the raw backend queue.
  pub fn raw_mut(&mut self) -> &mut backend::CommandQueue {
    &mut self.raw
  }

  /// Submits a prepared set of commands with synchronization using any number
  /// of semaphores and an optional fence.
  ///
  /// Pipeline stages will not execute until all of the associated semaphores in
  /// [`Submission::wait_semaphores`] have been signaled.
  ///
  /// After all of the submitted commands have executed, all of the semaphores
  /// in [`Submission::signal_semaphores`] will be signaled. If a fence is
  /// given, that will also be signaled.
  pub fn submit(&mut self, submission: &Submission, fence: Option<&Fence>) {
    // Create a temporary storage of references to the wait semaphores.
    let mut wait_semaphores = SmallVec::<[_; 1]>::new();

    wait_semaphores.extend(
      submission
        .wait_semaphores
        .iter()
        .map(|(semph, stage)| (semph.as_ref().as_ref(), *stage)),
    );

    // Create a temporary storage of references to the signal semaphores.
    let mut signal_semaphores = SmallVec::<[_; 16]>::new();

    signal_semaphores.extend(
      submission
        .signal_semaphores
        .iter()
        .map(AsRef::as_ref)
        .map(AsRef::as_ref),
    );

    unsafe {
      self.raw.submit_raw(
        hal::queue::RawSubmission {
          cmd_buffers: submission.commands.iter().map(AsRef::as_ref),
          wait_semaphores: &wait_semaphores,
          signal_semaphores: &signal_semaphores,
        },
        fence.map(AsRef::as_ref),
      );
    }
  }
}
