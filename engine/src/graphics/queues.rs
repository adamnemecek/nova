// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::backend::Backend;
use super::device::DeviceHandle;
use gfx_hal::queue::QueueFamily as RawQueueFamilyExt;

type RawQueue = <Backend as gfx_hal::Backend>::CommandQueue;
type RawQueueFamily = <Backend as gfx_hal::Backend>::QueueFamily;
type RawQueues = gfx_hal::queue::Queues<Backend>;

pub struct Queues {
  queues: Vec<RawQueue>,
  _families: Vec<RawQueueFamily>,
  _device: DeviceHandle,
}

impl Queues {
  pub(super) fn from_raw(
    device: &DeviceHandle,
    mut queues: RawQueues,
    families: Vec<RawQueueFamily>,
  ) -> Self {
    let queues = families
      .iter()
      .map(|f| {
        queues
          .take_raw(f.id())
          .expect("Adapter did not open all requested queue groups.")
          .into_iter()
          .next()
          .expect("Adapter did not open a queue for one or more requested queue groups.")
      })
      .collect();

    Queues {
      _device: device.clone(),
      _families: families,
      queues,
    }
  }

  pub fn count(&self) -> usize {
    self.queues.len()
  }
}
