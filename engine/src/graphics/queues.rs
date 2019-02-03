// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::backend::Backend;
use super::device::DeviceHandle;
use gfx_hal::queue::QueueFamily;

pub(crate) use gfx_hal::queue::RawCommandQueue as RawQueueExt;

type RawQueue = <Backend as gfx_hal::Backend>::CommandQueue;
type RawQueueFamily = <Backend as gfx_hal::Backend>::QueueFamily;
type RawQueues = gfx_hal::queue::Queues<Backend>;

pub struct Queues {
  queues: Vec<RawQueue>,
  families: Vec<RawQueueFamily>,
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
      families,
      queues,
    }
  }

  pub fn count(&self) -> usize {
    self.queues.len()
  }

  pub(crate) fn raw_mut(&mut self, index: QueueIndex) -> &mut RawQueue {
    &mut self.queues[index.0]
  }

  pub(crate) fn find_queue_raw(
    &self,
    mut filter: impl FnMut(&RawQueueFamily) -> bool,
  ) -> Option<QueueIndex> {
    for i in 0..self.queues.len() {
      if filter(&self.families[i]) {
        return Some(QueueIndex(i));
      }
    }

    None
  }
}

#[derive(Debug, Clone, Copy)]
pub struct QueueIndex(usize);
