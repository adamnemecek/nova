// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::queue::RawCommandQueue as GpuQueueExt;
pub use gfx_hal::Submission as QueueSubmission;
use std::borrow::Borrow;

use crate::gpu::commands::CommandBuffer;
use crate::gpu::sync::{Fence, Semaphore};
use crate::renderer::PipelineStage;
use crate::Backend;
use gfx_hal::queue::{QueueFamily as QueueFamilyExt, QueueFamilyId};
use nova_core::resources::{self, ReadResource, Resources, WriteResource};
use std::sync::Arc;

pub type ReadCommandQueues<'a> = ReadResource<'a, CommandQueues>;
pub type WriteCommandQueues<'a> = WriteResource<'a, CommandQueues>;

type HalQueue = <Backend as gfx_hal::Backend>::CommandQueue;
type HalQueues = gfx_hal::queue::Queues<Backend>;
type HalQueueFamily = <Backend as gfx_hal::Backend>::QueueFamily;

#[derive(Debug, Clone)]
pub struct QueueFamily {
  family: Arc<HalQueueFamily>,
  index: usize,
}

impl QueueFamily {
  pub(crate) fn id(&self) -> QueueFamilyId {
    self.family.id()
  }

  pub(crate) fn as_hal(&self) -> &HalQueueFamily {
    &self.family
  }

  pub fn supports_graphics(&self) -> bool {
    self.family.supports_graphics()
  }
}

pub struct CommandQueues {
  queues: Vec<HalQueue>,
  handles: Vec<QueueFamily>,
}

impl CommandQueues {
  pub(crate) fn new(families: Vec<HalQueueFamily>, mut hal_queues: HalQueues) -> Self {
    let mut queues = Vec::new();
    let mut handles = Vec::new();

    for (index, family) in families.into_iter().enumerate() {
      let queue = hal_queues
        .take_raw(family.id())
        .expect("Adapter did not open all requested queue groups.")
        .into_iter()
        .next()
        .expect("Adapter did not open a queue for one or more requested queue groups.");

      queues.push(queue);

      handles.push(QueueFamily {
        index,
        family: family.into(),
      });
    }

    Self { queues, handles }
  }

  pub fn find(&self, predicate: impl Fn(&QueueFamily) -> bool) -> Option<QueueFamily> {
    self.handles.iter().find(|f| predicate(f)).cloned()
  }

  pub fn submit<'a, C, Ci, W, Wi, S, Si>(
    &mut self,
    family: &QueueFamily,
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
      self.queues[family.index].submit(
        QueueSubmission {
          command_buffers,
          wait_semaphores,
          signal_semaphores,
        },
        signal_fence.map(Fence::as_hal),
      );
    }
  }

  pub(crate) fn get_mut(&mut self, family: &QueueFamily) -> &mut HalQueue {
    &mut self.queues[family.index]
  }

  pub(crate) fn clear(&mut self) {
    self.queues.clear();
  }
}

pub struct SubmitOptions<C, W, S> {
  pub command_buffers: C,
  pub wait_semaphores: W,
  pub signal_semaphores: S,
}

pub fn borrow(res: &Resources) -> ReadCommandQueues {
  resources::borrow(res)
}

pub fn borrow_mut(res: &Resources) -> WriteCommandQueues {
  resources::borrow_mut(res)
}
