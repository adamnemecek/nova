// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod framebuffer;
mod render_pass;

pub(crate) use self::framebuffer::Framebuffer;
pub(crate) use self::render_pass::RenderPass;

use crate::cmd;
use crate::pipeline;
use crate::{Context, Fence, OutOfMemoryError, Semaphore, Submission, Surface};
use crossbeam_channel as channel;
use nova_log as log;
use nova_math::Size;
use nova_time as time;
use nova_window as window;
use std::iter;
use std::thread;

#[derive(Clone)]
pub struct Handle {
  messages: channel::Sender<Message>,
}

impl Handle {
  pub fn resize_surface(&self, size: Size<f64>) {
    let _ = self.messages.send(Message::Resize(size));
  }
}

enum Message {
  Resize(Size<f64>),
}

pub fn start(
  context: &Context,
  window: &window::Handle,
  logger: &log::Logger,
) -> Result<Handle, OutOfMemoryError> {
  let context = context.clone();
  let logger = logger.clone();

  let graphics_queue_id = context.queues().find_graphics_queue();
  let command_pool = cmd::Pool::new(&context, graphics_queue_id)?;
  let render_pass = RenderPass::new(&context);
  let mut surface = Surface::new(&context, &window);
  let mut framebuffer = Framebuffer::new(&context);

  framebuffer.set_render_pass(&render_pass);

  let frame_fence = Fence::new(&context);
  let acquire_semaphore = Semaphore::new(&context);
  let render_semaphore = Semaphore::new(&context);
  let mut submission = Submission::new(graphics_queue_id);

  submission.signal(&render_semaphore);
  submission.wait_for(&acquire_semaphore, pipeline::Stage::COLOR_ATTACHMENT_OUTPUT);

  let (send_messages, recv_messages) = channel::unbounded();

  thread::spawn(move || {
    time::loop_at_frequency(60.0, |render_loop| {
      // Process incoming messages.
      loop {
        let message = match recv_messages.try_recv() {
          Ok(message) => message,
          Err(channel::TryRecvError::Empty) => break,

          // If the channel is disconnected, all handles have been dropped so
          // shut down the renderer.
          Err(channel::TryRecvError::Disconnected) => return render_loop.stop(),
        };

        match message {
          Message::Resize(size) => surface.set_size(size),
        }
      }

      // Wait for the previous frame to finish.
      frame_fence.wait_and_reset();

      // Clear resources used by the previous frame.
      submission.command_buffers.clear();

      // Acquire a backbuffer from the surface.
      let backbuffer = match surface.acquire(&acquire_semaphore) {
        Ok(backbuffer) => backbuffer,

        Err(err) => {
          log::error!(logger,
            "failed to acquire surface backbuffer";
            "cause" => log::Display(err),
          );

          return render_loop.stop();
        }
      };

      // Attach the backbuffer to the framebuffer.
      framebuffer.set_attachment(backbuffer.image());

      // Record rendering commands.
      let mut command_buffer = cmd::Buffer::new(&command_pool);
      let mut cmd = command_buffer.record();

      cmd.begin_render_pass(&mut framebuffer);

      cmd.finish();

      // Submit rendering commands.
      submission.command_buffers.push(command_buffer);

      context.queues().submit(&submission, &frame_fence);

      // Present the backbuffer.
      if let Err(err) = backbuffer.present(&[&render_semaphore]) {
        log::error!(logger,
          "failed to present surface backbuffer";
          "cause" => log::Display(err),
        );

        return render_loop.stop();
      }
    });
  });

  Ok(Handle {
    messages: send_messages,
  })
}

/*
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
      .expect("device does not support graphics commands");

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
*/
