// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::queue::RawCommandQueue as GpuQueueExt;
pub use gfx_hal::Submission as QueueSubmission;

use crate::surfaces::{HalSurface, HalSurfaceExt};
use crate::Backend;
use gfx_hal::queue::{QueueFamily as QueueFamilyExt, QueueFamilyId};
use nova_core::resources::{self, ReadResource, Resources, WriteResource};
use std::ops::{Index, IndexMut};

pub type GpuQueue = <Backend as gfx_hal::Backend>::CommandQueue;
pub type ReadGpuQueues<'a> = ReadResource<'a, GpuQueues>;
pub type WriteGpuQueues<'a> = WriteResource<'a, GpuQueues>;

type QueueFamily = <Backend as gfx_hal::Backend>::QueueFamily;
type RawQueues = gfx_hal::queue::Queues<Backend>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QueueId {
  index: usize,
  pub(crate) family_id: QueueFamilyId,
}

pub struct GpuQueues {
  families: Vec<QueueFamily>,
  queues: Vec<GpuQueue>,
}

impl GpuQueues {
  pub(crate) fn new(families: Vec<QueueFamily>, mut queues: RawQueues) -> Self {
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

  pub fn find_graphics_queue(&self) -> Option<QueueId> {
    for (index, family) in self.families.iter().enumerate() {
      if family.supports_graphics() {
        return Some(QueueId {
          index,
          family_id: family.id(),
        });
      }
    }

    None
  }

  pub(crate) fn find_present_queue(&self, surface: &HalSurface) -> Option<QueueId> {
    for (index, family) in self.families.iter().enumerate() {
      if surface.supports_queue_family(family) {
        return Some(QueueId {
          index,
          family_id: family.id(),
        });
      }
    }

    None
  }
}

impl Index<QueueId> for GpuQueues {
  type Output = GpuQueue;

  fn index(&self, id: QueueId) -> &GpuQueue {
    &self.queues[id.index]
  }
}

impl IndexMut<QueueId> for GpuQueues {
  fn index_mut(&mut self, id: QueueId) -> &mut GpuQueue {
    &mut self.queues[id.index]
  }
}

pub fn borrow(res: &Resources) -> ReadGpuQueues {
  resources::borrow(res)
}

pub fn borrow_mut(res: &Resources) -> WriteGpuQueues {
  resources::borrow_mut(res)
}
