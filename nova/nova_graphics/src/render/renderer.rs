// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::commands::CommandBuffer;
use crate::gpu;
use crate::gpu::queues::{GpuQueueExt as _, QueueId, QueueSubmission};
use crate::pipelines::PipelineStage;
use crate::render::RenderOptions;
use crate::sync::{Fence, Semaphore};
use nova_core::resources::Resources;

pub struct Renderer {
  queue_id: QueueId,
  frame_fence: Fence,
  commands: CommandBuffer,
  transfer_commands: CommandBuffer,
}

impl Renderer {
  pub fn new(res: &Resources) -> Self {
    let gpu = gpu::borrow(res);

    let queue_id = gpu::queues::borrow(res)
      .find_graphics_queue()
      .expect("Device does not support graphics commands.");

    let frame_fence = Fence::new(&gpu);

    let commands = CommandBuffer::new(&gpu, queue_id);
    let transfer_commands = CommandBuffer::new(&gpu, queue_id);

    Renderer {
      queue_id,
      frame_fence,
      commands,
      transfer_commands,
    }
  }

  pub fn render<'a, W, S>(&'a mut self, res: &Resources, options: RenderOptions<W, S>)
  where
    W: IntoIterator<Item = (&'a Semaphore, PipelineStage)>,
    S: IntoIterator<Item = &'a Semaphore>,
  {
    let gpu = gpu::borrow(res);

    self.frame_fence.wait_and_reset(&gpu);

    self.commands.begin();
    self.commands.finish();

    self.transfer_commands.begin();
    self.transfer_commands.finish();

    self.submit(res, options);
  }

  pub fn submit<'a, W, S>(&'a mut self, res: &Resources, options: RenderOptions<W, S>)
  where
    W: IntoIterator<Item = (&'a Semaphore, PipelineStage)>,
    S: IntoIterator<Item = &'a Semaphore>,
  {
    let mut queues = gpu::queues::borrow_mut(res);
    let queue = &mut queues[self.queue_id];

    let wait_for = options.wait_for.into_iter().map(|(s, p)| (&s.0, p));
    let signal = options.signal.into_iter().map(|s| &s.0);

    unsafe {
      queue.submit(
        QueueSubmission {
          command_buffers: [&self.transfer_commands, &self.commands]
            .iter()
            .map(|c| c.as_backend()),
          wait_semaphores: wait_for,
          signal_semaphores: signal,
        },
        Some(&self.frame_fence.0),
      );
    }
  }
}
