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
use std::sync::Arc;
use std::thread;

pub struct Renderer {
  messages: channel::Sender<Message>,
  thread: thread::JoinHandle<()>,
}

enum Message {
  Resize(Size<f64>),
  ShutDown,
}

impl Renderer {
  pub fn resize_surface(&self, size: Size<f64>) {
    let _ = self.messages.send(Message::Resize(size));
  }

  pub fn shut_down(self) {
    let _ = self.messages.send(Message::ShutDown);
    let _ = self.thread.join();
  }
}

pub fn start(
  context: &Context,
  window: &window::Handle,
  logger: &log::Logger,
) -> Result<Renderer, OutOfMemoryError> {
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

  let thread = thread::spawn(move || {
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
          Message::ShutDown => return render_loop.stop(),
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

    // Wait for everything to flush out before shutting down.
    context.wait_idle();
  });

  Ok(Renderer {
    messages: send_messages,
    thread,
  })
}
