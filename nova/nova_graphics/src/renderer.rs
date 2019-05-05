// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod canvas;
mod framebuffer;
mod pipeline;
mod render_pass;
mod shader;

pub use self::canvas::Canvas;
pub use self::pipeline::PipelineStage;
pub use gfx_hal::memory::Barrier as MemoryBarrier;

pub(crate) use self::framebuffer::Framebuffer;
pub(crate) use self::pipeline::Pipeline;
pub(crate) use self::render_pass::RenderPass;

use self::shader::Shader;
use crate::gpu::commands::CommandBuffer;
use crate::gpu::queues::{QueueFamily, Submission};
use crate::gpu::sync::{Fence, Semaphore};
use crate::gpu::{self, Gpu};
use crate::images::{self, ImageId};
use crate::Color;
use nova_core::resources::Resources;
use std::borrow::Borrow;
use std::iter;

pub struct Renderer {
  render_pass: RenderPass,
  frame_fence: Fence,
  framebuffer: Option<Framebuffer>,
  transfer_commands: CommandBuffer,
  canvas: Canvas,
}

impl Renderer {
  pub fn new(res: &Resources) -> Self {
    let gpu = gpu::borrow(res);

    let queue_family = gpu::queues::borrow(res)
      .find(QueueFamily::supports_graphics)
      .expect("Device does not support graphics commands.");

    let render_pass = RenderPass::new(&gpu);
    let frame_fence = Fence::new(&gpu);
    let transfer_commands = CommandBuffer::new(&gpu, &queue_family);
    let canvas = Canvas::new(&gpu, CommandBuffer::new(&gpu, &queue_family));

    Renderer {
      render_pass,
      frame_fence,
      framebuffer: None,
      transfer_commands,
      canvas,
    }
  }

  pub fn render<'a, W, Wi, S, Si>(&'a mut self, res: &Resources, options: RenderOptions<W, S>)
  where
    W: IntoIterator<Item = (&'a Wi, PipelineStage)>,
    Wi: 'a + Borrow<Semaphore>,
    S: IntoIterator<Item = &'a Si>,
    Si: 'a + Borrow<Semaphore>,
  {
    let gpu = gpu::borrow(res);

    self.frame_fence.wait_and_reset(&gpu);

    if let Some(framebuffer) = self.framebuffer.take() {
      framebuffer.destroy(&gpu);
    }

    let framebuffer = {
      let images = images::borrow(res);

      let image = images
        .get(options.target)
        .expect("Target image does not exist.");

      Framebuffer::new(&gpu, &self.render_pass, iter::once(image))
    };

    self.canvas.begin(&framebuffer);
    self.framebuffer = Some(framebuffer);

    self.canvas.draw_quad(Color::new(1.0, 1.0, 0.0, 1.0));

    self.canvas.finish();

    {
      let mut images = images::borrow_mut(res);

      self.transfer_commands.begin();

      images.flush_changes(&mut self.transfer_commands);

      self.transfer_commands.finish();
    }

    let mut queues = gpu::queues::borrow_mut(res);

    queues.submit(
      self.transfer_commands.queue_family(),
      Submission {
        command_buffers: vec![&self.transfer_commands, self.canvas.commands()],
        wait_semaphores: options.wait_semaphores,
        signal_semaphores: options.signal_semaphores,
        fence: Some(&self.frame_fence),
      },
    );
  }

  pub fn destroy(self, gpu: &Gpu) {
    gpu.wait_idle();

    if let Some(framebuffer) = self.framebuffer {
      framebuffer.destroy(&gpu);
    }

    self.transfer_commands.destroy(&gpu);
    self.canvas.destroy(&gpu);
    self.frame_fence.destroy(&gpu);
    self.render_pass.destroy(&gpu);
  }
}

pub struct RenderOptions<W, S> {
  pub target: ImageId,
  pub wait_semaphores: W,
  pub signal_semaphores: S,
}
