// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use gfx_hal::queue::RawCommandQueue as _;
use gfx_hal::QueueFamily as _;

/// Uniquely identifies a single command queue of a graphics device.
#[derive(Debug, Clone, Copy)]
pub struct QueueId {
  index: usize,
  family_id: gfx_hal::queue::QueueFamilyId,
}

impl QueueId {
  /// Returns the underlying backend queue family ID.
  pub(crate) fn as_backend(&self) -> gfx_hal::queue::QueueFamilyId {
    self.family_id
  }
}

/// Structure for accessing the graphics, compute, and transfer queues of a
/// device.
pub struct Queues {
  queues: Vec<Queue>,
}

struct Queue {
  family: backend::QueueFamily,
  queue: Mutex<backend::Queue>,
}

impl Queues {
  /// Creates a new set of queues from backend queues and queue families.
  pub fn new(
    families: impl IntoIterator<Item = backend::QueueFamily>,
    mut input: backend::Queues,
  ) -> Self {
    let mut queues = Vec::new();

    for family in families.into_iter() {
      let queue = input
        .take_raw(family.id())
        .expect("adapter did not open all requested queue groups")
        .into_iter()
        .next()
        .expect("adapter did not open a queue for one or more requested queue groups")
        .into();

      queues.push(Queue { queue, family });
    }

    Self { queues }
  }

  /// Finds a queue suitable for graphics commands.
  pub fn find_graphics_queue(&self) -> QueueId {
    // Return the first queue that supports graphics commands.
    for (index, queue) in self.queues.iter().enumerate() {
      if queue.family.supports_graphics() {
        return QueueId {
          index,
          family_id: queue.family.id(),
        };
      }
    }

    panic!("device has no graphics queues");
  }

  /// Finds a queue suitable for transfer commands.
  pub fn find_transfer_queue(&self) -> QueueId {
    // First look for a queue that is specifically made for transfers.
    for (index, queue) in self.queues.iter().enumerate() {
      if !queue.family.supports_graphics() && !queue.family.supports_compute() {
        return QueueId {
          index,
          family_id: queue.family.id(),
        };
      }
    }

    // Otherwise just use the same queue as graphics commands.
    self.find_graphics_queue()
  }

  pub fn submit<'a>(&'a self, submission: &'a Submission, fence: impl Into<Option<&'a Fence>>) {
    let mut queue = self.queues[submission.queue_id.index].queue.lock();

    unsafe {
      queue.submit(
        gfx_hal::Submission {
          command_buffers: submission.command_buffers.iter().map(cmd::List::as_backend),
          wait_semaphores: submission
            .wait_semaphores
            .iter()
            .map(|(sem, stage)| (sem.as_backend(), *stage)),
          signal_semaphores: submission
            .signal_semaphores
            .iter()
            .map(Semaphore::as_backend),
        },
        fence.into().map(Fence::as_backend),
      );
    }
  }

  pub fn find_present_queue(&self, surface: &backend::Surface) -> QueueId {
    // Return the first queue that supports presentation for this surface.
    for (index, queue) in self.queues.iter().enumerate() {
      use gfx_hal::Surface as _;

      if surface.supports_queue_family(&queue.family) {
        return QueueId {
          index,
          family_id: queue.family.id(),
        };
      }
    }

    panic!("device cannot present to window");
  }

  pub fn present(
    &self,
    queue_id: QueueId,
    swapchain: &backend::Swapchain,
    image_index: u32,
    wait_for: &[&Semaphore],
  ) -> Result<(), gfx_hal::window::PresentError> {
    use gfx_hal::queue::RawCommandQueue as _;

    let result = unsafe {
      self.queues[queue_id.index].queue.lock().present(
        iter::once((swapchain, image_index)),
        wait_for.iter().map(|s| s.as_backend()),
      )
    };

    match result {
      Ok(None) => Ok(()),
      Ok(Some(_)) => Err(gfx_hal::window::PresentError::OutOfDate),
      Err(err) => Err(err),
    }
  }
}
