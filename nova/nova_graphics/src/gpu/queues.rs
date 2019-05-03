// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::queue::{QueueType as GpuQueueKind, RawCommandQueue as GpuQueueExt};
pub use gfx_hal::Submission as QueueSubmission;
use std::borrow::Borrow;

use crate::gpu::CommandBuffer;
use crate::rendering::PipelineStage;
use crate::sync::{Fence, Semaphore};
use crate::Backend;
use gfx_hal::queue::{QueueFamily as QueueFamilyExt, QueueFamilyId};
use nova_core::resources::{self, ReadResource, Resources, WriteResource};
use std::ops::{Deref, Index, IndexMut};

pub type ReadGpuQueues<'a> = ReadResource<'a, GpuQueues>;
pub type WriteGpuQueues<'a> = WriteResource<'a, GpuQueues>;

type HalQueue = <Backend as gfx_hal::Backend>::CommandQueue;
type HalQueues = gfx_hal::queue::Queues<Backend>;
type HalQueueFamily = <Backend as gfx_hal::Backend>::QueueFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GpuQueueId {
  index: usize,
  pub(crate) family_id: QueueFamilyId,
}

pub struct GpuQueues {
  queues: Vec<GpuQueue>,
}

impl GpuQueues {
  pub(crate) fn new(families: Vec<HalQueueFamily>, mut queues: HalQueues) -> Self {
    let queues = families
      .into_iter()
      .enumerate()
      .map(|(index, family)| {
        let family_id = family.id();

        let queue = queues
          .take_raw(family.id())
          .expect("Adapter did not open all requested queue groups.")
          .into_iter()
          .next()
          .expect("Adapter did not open a queue for one or more requested queue groups.");

        GpuQueue {
          queue,
          family,
          id: GpuQueueId { index, family_id },
        }
      })
      .collect::<Vec<_>>();

    GpuQueues { queues }
  }

  pub fn find<P>(&self, predicate: P) -> Option<GpuQueueId>
  where
    P: FnMut(&&GpuQueue) -> bool,
  {
    self.queues.iter().find(predicate).map(GpuQueue::id)
  }

  pub fn find_kind(&self, kind: GpuQueueKind) -> Option<GpuQueueId> {
    self
      .find(|q| q.kind() == kind)
      .or_else(|| self.find(|q| q.kind() == GpuQueueKind::General))
  }

  pub(crate) fn clear(&mut self) {
    self.queues.clear();
  }
}

impl Deref for GpuQueues {
  type Target = [GpuQueue];

  fn deref(&self) -> &Self::Target {
    &self.queues
  }
}

impl Index<GpuQueueId> for GpuQueues {
  type Output = GpuQueue;

  fn index(&self, id: GpuQueueId) -> &GpuQueue {
    &self.queues[id.index]
  }
}

impl IndexMut<GpuQueueId> for GpuQueues {
  fn index_mut(&mut self, id: GpuQueueId) -> &mut GpuQueue {
    &mut self.queues[id.index]
  }
}

pub struct GpuQueue {
  pub(crate) queue: HalQueue,
  pub(crate) family: HalQueueFamily,
  pub(crate) id: GpuQueueId,
}

impl GpuQueue {
  pub fn id(&self) -> GpuQueueId {
    self.id
  }

  pub fn kind(&self) -> GpuQueueKind {
    self.family.queue_type()
  }

  pub fn submit<'a, C, Ci, W, Wi, S, Si>(
    &mut self,
    options: SubmitOptions<C, W, S>,
    signal_fence: Option<&Fence>,
  ) where
    C: IntoIterator<Item = &'a Ci>,
    Ci: 'a + Borrow<CommandBuffer>,
    W: IntoIterator<Item = (&'a Wi, PipelineStage)>,
    Wi: 'a + Borrow<Semaphore>,
    S: IntoIterator<Item = &'a Si>,
    Si: 'a + Borrow<Semaphore>,
  {
    let command_buffers = options
      .command_buffers
      .into_iter()
      .map(Borrow::borrow)
      .map(CommandBuffer::as_hal);

    let wait_semaphores = options
      .wait_semaphores
      .into_iter()
      .map(|(s, p)| (s.borrow().as_hal(), p));

    let signal_semaphores = options
      .signal_semaphores
      .into_iter()
      .map(Borrow::borrow)
      .map(Semaphore::as_hal);

    unsafe {
      self.queue.submit(
        QueueSubmission {
          command_buffers,
          wait_semaphores,
          signal_semaphores,
        },
        signal_fence.map(Fence::as_hal),
      );
    }
  }

  pub fn as_hal(&self) -> &HalQueue {
    &self.queue
  }

  pub fn as_hal_mut(&mut self) -> &mut HalQueue {
    &mut self.queue
  }
}

pub struct SubmitOptions<C, W, S> {
  pub command_buffers: C,
  pub wait_semaphores: W,
  pub signal_semaphores: S,
}

pub fn borrow(res: &Resources) -> ReadGpuQueues {
  resources::borrow(res)
}

pub fn borrow_mut(res: &Resources) -> WriteGpuQueues {
  resources::borrow_mut(res)
}
