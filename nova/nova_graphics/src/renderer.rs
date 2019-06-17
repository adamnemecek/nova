// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod framebuffer;
mod render_pass;

pub(crate) use self::framebuffer::Framebuffer;
pub(crate) use self::render_pass::RenderPass;

use crate::{
  cmd, pipeline, shader, Color, Context, Fence, OutOfMemoryError, Semaphore, Submission, Surface,
};

use nova_log as log;
use nova_math::Size;
use nova_sync::channel;
use nova_time as time;
use nova_window as window;
use std::{mem, thread};

/// Renders graphics onto a window on a background thread.
pub struct Renderer {
  messages: channel::Sender<RendererMsg>,
  thread: thread::JoinHandle<()>,
}

/// Control message for a renderer background thread.
enum RendererMsg {
  ShutDown,
  ResizeSurface(Size<f64>),
}

impl Renderer {
  /// Resizes the render surface.
  ///
  /// Call this function each time the window resizes.
  pub fn resize_surface(&self, size: Size<f64>) {
    let _ = self.messages.send(RendererMsg::ResizeSurface(size));
  }

  /// Shuts down the renderer, blocking until the background thread completes.
  pub fn shut_down(self) {
    let _ = self.messages.send(RendererMsg::ShutDown);
    let _ = self.thread.join();
  }
}

/// Starts a renderer on a background thread, returning a `Renderer` to
/// represent it.
pub fn start(
  context: &Context,
  window: &window::Handle,
  logger: &log::Logger,
) -> Result<Renderer, OutOfMemoryError> {
  let context = context.clone();
  let logger = logger.clone();

  // Create resources needed for rendering.
  let graphics_queue_id = context.queues().find_graphics_queue();
  let command_pool = cmd::Pool::new(&context, graphics_queue_id)?;
  let render_pass = RenderPass::new(&context);
  let mut surface = Surface::new(&context, &window);
  let mut framebuffer = Framebuffer::new(&context);

  framebuffer.set_render_pass(&render_pass);

  let frame_fence = Fence::new(&context)?;
  let acquire_semaphore = Semaphore::new(&context)?;
  let render_semaphore = Semaphore::new(&context)?;

  let vertex_shader = shader::compile_hlsl(
    &context,
    shader::Stage::Vertex,
    include_str!("./renderer/shaders/quad.vert"),
  );

  let fragment_shader = shader::compile_hlsl(
    &context,
    shader::Stage::Fragment,
    include_str!("./renderer/shaders/color.frag"),
  );

  let pipeline = pipeline::Graphics::new(
    &context,
    &render_pass,
    pipeline::Options {
      size_of_push_constants: mem::size_of::<Color>(),
      shaders: pipeline::ShaderSet {
        vertex: vertex_shader.expect("failed to create vertex shader"),
        fragment: fragment_shader.expect("failed to create fragment shader"),
      },
    },
  )
  .expect("failed to create graphics pipeline");

  // Set up a submission that waits for the acquire semaphore and signals the
  // render semaphore when complete.
  let mut submission = Submission::new(graphics_queue_id);

  submission.wait_for(&acquire_semaphore, pipeline::Stage::COLOR_ATTACHMENT_OUTPUT);
  submission.signal(&render_semaphore);

  // Create a channel to send and receive control messages.
  let (send_messages, recv_messages) = channel::unbounded();

  // Spawn the renderer on a background thread.
  let thread = thread::spawn(move || {
    // Try to render at 60 fps maximum.
    time::loop_at_frequency(60.0, |render_loop| {
      // Process incoming messages.
      loop {
        let message = match recv_messages.try_recv() {
          Ok(message) => message,
          Err(channel::TryRecvError::Disconnected) => return render_loop.stop(),
          Err(channel::TryRecvError::Empty) => break,
        };

        match message {
          RendererMsg::ShutDown => return render_loop.stop(),
          RendererMsg::ResizeSurface(size) => surface.resize(size),
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

      if let Err(err) = framebuffer.ensure_created() {
        log::error!(logger,
          "failed to create framebuffer";
          "cause" => log::Display(err),
        );

        return render_loop.stop();
      }

      // Record rendering commands.
      let mut command_buffer = cmd::Buffer::new(&command_pool);
      let mut cmd = command_buffer.record();

      cmd.begin_render_pass(&mut framebuffer);

      cmd.bind_graphics_pipeline(&pipeline);
      cmd.push_graphics_constants(&Color::new(0.0, 1.0, 0.0, 1.0));
      cmd.draw(0..4);

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
