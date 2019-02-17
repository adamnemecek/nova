// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Device, Fence, Semaphore};
use crate::graphics::renderer::PipelineStage;
use crate::graphics::{Backend, Commands};
use gfx_hal::queue::{QueueFamily, QueueFamilyId};

pub(crate) use gfx_hal::queue::RawCommandQueue as RawQueueExt;

type RawQueue = <Backend as gfx_hal::Backend>::CommandQueue;
type RawQueueFamily = <Backend as gfx_hal::Backend>::QueueFamily;
type RawQueues = gfx_hal::queue::Queues<Backend>;

pub struct Queues {
  queues: Vec<RawQueue>,
  families: Vec<RawQueueFamily>,
  _device: Device,
}

impl Queues {
  pub(crate) fn from_raw(
    device: &Device,
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

  pub fn get_graphics_queue(&self) -> Option<QueueId> {
    self.find_queue_raw(|family| family.supports_graphics())
  }

  pub fn submit<'a, W, S>(&mut self, submission: QueueSubmission<'a, W, S>)
  where
    W: IntoIterator<Item = (&'a Semaphore, PipelineStage)>,
    S: IntoIterator<Item = &'a Semaphore>,
  {
    let queue = &mut self.queues[submission.commands.queue_id().index];

    unsafe {
      queue.submit(
        gfx_hal::queue::Submission {
          command_buffers: Some(&submission.commands.raw),

          wait_semaphores: submission
            .wait_semaphores
            .into_iter()
            .map(|(sem, stage)| (sem.raw(), stage)),

          signal_semaphores: submission
            .signal_semaphores
            .into_iter()
            .map(|sem| sem.raw()),
        },
        submission.fence.map(Fence::raw),
      );
    }
  }

  pub(crate) fn raw_mut(&mut self, id: QueueId) -> &mut RawQueue {
    &mut self.queues[id.index]
  }

  pub(crate) fn find_queue_raw(
    &self,
    mut filter: impl FnMut(&RawQueueFamily) -> bool,
  ) -> Option<QueueId> {
    for index in 0..self.queues.len() {
      let family = &self.families[index];

      if filter(family) {
        return Some(QueueId {
          index,
          family_id: family.id(),
        });
      }
    }

    None
  }
}

#[derive(Debug, Clone, Copy)]
pub struct QueueId {
  index: usize,
  family_id: QueueFamilyId,
}

impl QueueId {
  pub(crate) fn family_id(&self) -> QueueFamilyId {
    self.family_id
  }
}

pub struct QueueSubmission<'a, W, S> {
  pub commands: &'a Commands,
  pub wait_semaphores: W,
  pub signal_semaphores: S,
  pub fence: Option<&'a Fence>,
}
