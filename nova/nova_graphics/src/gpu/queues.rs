// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::backend::Backend;
use gfx_hal::QueueFamily as _;
use nova_core::resources::{self, ReadResource, Resources, WriteResource};

pub type GpuQueue = <Backend as gfx_hal::Backend>::CommandQueue;
pub type ReadGpuQueues<'a> = ReadResource<'a, GpuQueues>;
pub type WriteGpuQueues<'a> = WriteResource<'a, GpuQueues>;

type GpuQueueFamily = <Backend as gfx_hal::Backend>::QueueFamily;
type RawQueues = gfx_hal::queue::Queues<Backend>;

pub struct GpuQueues {
  families: Vec<GpuQueueFamily>,
  queues: Vec<GpuQueue>,
}

impl GpuQueues {
  pub(crate) fn new(families: Vec<GpuQueueFamily>, mut queues: RawQueues) -> Self {
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
      .collect::<Vec<_>>();

    GpuQueues { families, queues }
  }
}

pub fn borrow(res: &Resources) -> ReadGpuQueues {
  resources::borrow(res)
}

pub fn borrow_mut(res: &Resources) -> WriteGpuQueues {
  resources::borrow_mut(res)
}
