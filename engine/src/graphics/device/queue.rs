// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{DeviceHandle, Submission};
use crate::graphics::prelude::*;
use crate::graphics::present::Surface;
use crate::graphics::sync::Fence;
use smallvec::SmallVec;
use std::sync::MutexGuard;

/// Represents a device queue to which commands can be submitted.
pub struct Queue {
  raw: backend::CommandQueue,
}

impl Queue {
  /// Takes a raw queue from a set of HAL queues.
  pub fn take_raw(queues: &mut hal::queue::Queues, family_id: hal::queue::QueueFamilyId) -> Self {
    let raw = queues
      .take_raw(family_id)
      .expect("Expected device queue family was missing.")
      .into_iter()
      .next()
      .expect("Expected device queue was missing.");

    Queue { raw }
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

#[derive(Clone)]
pub struct QueueHandle {
  device: DeviceHandle,
  index: usize,
}

impl QueueHandle {
  pub fn family_id(&self) -> usize {
    self.device.adapter.queue_families[self.index].id().0
  }

  pub fn lock(&self) -> MutexGuard<Queue> {
    self.device.queues[self.index]
      .lock()
      .expect("Could not lock queue")
  }
}

/// Gets a handle to a queue suitable for graphics commands.
pub fn get_graphics_queue(device: &DeviceHandle) -> QueueHandle {
  // Use the first queue that supports graphics.
  let index = device
    .adapter
    .queue_families
    .iter()
    .position(|family| family.supports_graphics())
    .expect("Could not find a graphics queue.");

  QueueHandle {
    device: device.clone(),
    index,
  }
}

/// Gets a handle to a queue suitable for presenting the given surface.
pub fn get_present_queue(surface: &Surface) -> QueueHandle {
  let device = surface.device().clone();

  // Use the first queue that supports graphics.
  let index = device
    .adapter
    .queue_families
    .iter()
    .position(|family| surface.as_ref().supports_queue_family(family))
    .expect("Could not find a present queue.");

  QueueHandle { device, index }
}

/// Gets a handle to a queue suitable for transfer commands.
pub fn get_transfer_queue(device: &DeviceHandle) -> QueueHandle {
  // Use a queue that supports neither graphics nor compute, since it may be
  // specialized for transfer.
  let index = device
    .adapter
    .queue_families
    .iter()
    .position(|family| !family.supports_graphics() && !family.supports_compute())
    // If that doesn't exist, use a queue that doesn't support graphics, to
    // avoid contention with rendering.
    .or_else(|| {
      device
        .adapter
        .queue_families
        .iter()
        .position(|family| !family.supports_graphics())
    })
    // Otherwise use the first available queue.
    .unwrap_or(0);

  QueueHandle {
    device: device.clone(),
    index,
  }
}
