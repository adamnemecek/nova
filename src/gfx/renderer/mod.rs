// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod handle;

pub use self::handle::*;

use super::*;

pub struct Renderer {
  context: Arc<Context>,
  surface: Surface,
  cmd_pool: cmd::Pool,
  acquire_semaphore: cmd::Semaphore,
  render_semaphore: cmd::Semaphore,
  frame_fence: cmd::Fence,
  framebuffer: Framebuffer,
}

impl Renderer {
  pub fn new(surface: Surface, queue_id: cmd::QueueId) -> Result<Self, StartError> {
    let context = surface.context();

    let cmd_pool = cmd::Pool::new(context, queue_id)?;
    let acquire_semaphore = cmd::Semaphore::new(context)?;
    let render_semaphore = cmd::Semaphore::new(context)?;
    let frame_fence = cmd::Fence::new(context, false)?;
    let mut framebuffer = Framebuffer::new(context);
    let render_pass = RenderPass::new(context).into();

    framebuffer.set_render_pass(&render_pass);

    Ok(Renderer {
      context: context.clone(),
      surface,
      cmd_pool,
      acquire_semaphore,
      render_semaphore,
      frame_fence,
      framebuffer,
    })
  }

  pub fn render(&mut self) -> Result<(), RenderError> {
    let backbuffer = self.surface.acquire(&self.acquire_semaphore)?;

    self.framebuffer.set_attachment(backbuffer.image());
    self.framebuffer.ensure_created()?;

    let mut cmd_list = cmd::List::new(&self.cmd_pool);
    let mut cmd = cmd_list.begin();

    cmd.begin_render_pass(&self.framebuffer);
    cmd.end_render_pass();

    cmd.end();

    self.context.queues().submit(cmd::Submission {
      queue_id: self.cmd_pool.queue_id(),
      lists: &[&cmd_list],
      wait_semaphores: &[(&self.acquire_semaphore, pipeline::Stage::COLOR_ATTACHMENT_OUTPUT)],
      signal_semaphores: &[&self.render_semaphore],
      fence: &self.frame_fence,
    });

    backbuffer.present(&[&self.render_semaphore])?;

    self.frame_fence.wait_and_reset();

    Ok(())
  }
}

impl Drop for Renderer {
  fn drop(&mut self) {
    self.context.wait_idle();
  }
}

pub fn start(
  thread_scope: &thread::Scope,
  context: &Arc<Context>,
  window: &window::Handle,
  logger: &log::Logger,
) -> Result<Handle, StartError> {
  let logger = logger.clone();

  let surface = Surface::new(&context, window, &logger);
  let queue_id = context.queues().find_graphics_queue();

  let (send_result, recv_result) = channel::bounded(0);

  thread_scope.spawn(move |_| {
    let mut renderer = match Renderer::new(surface, queue_id) {
      Ok(renderer) => renderer,

      Err(err) => {
        let _ = send_result.send(Err(err));

        return;
      }
    };

    let (send_messages, recv_messages) = channel::unbounded();
    let handle = Handle::new(send_messages);

    if send_result.send(Ok(handle)).is_err() {
      return;
    }

    log::info!(&logger, "renderer started");

    let mut clock = time::Clock::new().with_frequency(60.0);
    let mut is_stopping = false;

    while !is_stopping {
      clock.tick();

      if let Err(err) = renderer.render() {
        log::crit!(&logger, "could not render frame: {}", err);
        break;
      }

      loop {
        match recv_messages.try_recv() {
          Ok(ControlMessage::SetTargetFPS(value)) => {
            clock.set_frequency(value.max(0.0));
          }

          Ok(ControlMessage::Stop) => {
            is_stopping = true;
          }

          Err(_) => {
            break;
          }
        }
      }
    }

    drop(renderer);

    log::info!(&logger, "renderer stopped");
  });

  recv_result.recv()?
}

#[derive(Debug)]
pub enum StartError {
  OutOfMemory,
  Unknown,
}

impl fmt::Display for StartError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      StartError::OutOfMemory => write!(f, "out of memory"),
      StartError::Unknown => write!(f, "unknown error"),
    }
  }
}

impl From<OutOfMemoryError> for StartError {
  fn from(_: OutOfMemoryError) -> Self {
    StartError::OutOfMemory
  }
}

impl From<channel::RecvError> for StartError {
  fn from(_: channel::RecvError) -> Self {
    StartError::Unknown
  }
}

#[derive(Debug)]
pub enum RenderError {
  BackbufferAcquireFailed(SurfaceAcquireError),
  BackbufferPresentFailed(SurfacePresentError),
  OutOfMemory,
}

impl fmt::Display for RenderError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      RenderError::BackbufferAcquireFailed(err) => {
        write!(f, "failed to acquire backbuffer: {}", err)
      }

      RenderError::BackbufferPresentFailed(err) => {
        write!(f, "failed to present backbuffer: {}", err)
      }

      RenderError::OutOfMemory => write!(f, "out of memory"),
    }
  }
}

impl From<SurfaceAcquireError> for RenderError {
  fn from(err: SurfaceAcquireError) -> Self {
    RenderError::BackbufferAcquireFailed(err)
  }
}

impl From<SurfacePresentError> for RenderError {
  fn from(err: SurfacePresentError) -> Self {
    RenderError::BackbufferPresentFailed(err)
  }
}

impl From<OutOfMemoryError> for RenderError {
  fn from(_: OutOfMemoryError) -> Self {
    RenderError::OutOfMemory
  }
}
