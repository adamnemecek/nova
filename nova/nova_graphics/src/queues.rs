// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::backend;
use gfx_hal::QueueFamily as _;

/// Identifies a single device queue.
#[derive(Debug)]
pub struct QueueId {
  index: usize,

  /// The backend queue family ID.
  pub(crate) family_id: gfx_hal::queue::QueueFamilyId,
}

/// Structure for accessing the graphics, compute, and transfer queues of a
/// device.
pub struct Queues {
  queues: Vec<Queue>,
}

struct Queue {
  family: backend::QueueFamily,
  #[allow(dead_code)]
  queue: backend::Queue,
}

impl Queues {
  /// Creates a new set of queues from backend queues and queue families.
  pub(crate) fn new(
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
        .expect("adapter did not open a queue for one or more requested queue groups");

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
}
