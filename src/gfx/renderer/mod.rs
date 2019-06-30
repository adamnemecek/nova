// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Renders individual frames to a [`Surface`].
struct Renderer {
  context: Arc<Context>,
  surface: Surface,
  cmd_pool: cmd::Pool,
  acquire_semaphore: cmd::Semaphore,
  render_semaphore: cmd::Semaphore,
  frame_fence: cmd::Fence,
  framebuffer: Framebuffer,
}

impl Renderer {
  /// Creates a new renderer targeting the given surface.
  fn new(surface: Surface) -> Result<Self, StartError> {
    let context = surface.context();

    let queue_id = context.queues().find_graphics_queue();
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

  /// Renders a single frame.
  ///
  /// This function blocks until the commands for the frame have been fully
  /// executed.
  fn render(&mut self) -> Result<(), RenderError> {
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

// Implement `Drop` to wait for the device to finish executing all commands
// before destroying resources.
impl Drop for Renderer {
  fn drop(&mut self) {
    self.context.wait_idle();
  }
}

/// Starts a new renderer with the given options.
pub fn start(
  thread_scope: &thread::Scope,
  context: &Arc<Context>,
  window: &window::Handle,
  logger: &log::Logger,
) -> Result<(), StartError> {
  let logger = logger.clone();

  // Create a surface to render to.
  let surface = Surface::new(&context, window, &logger);

  // Create a channel to receive the `Handle` from the render thread.
  let (send_result, recv_result) = channel::bounded(0);

  // Start a thread to run the render loop.
  thread_scope.spawn(move |_| {
    // Create a new renderer or send the error back to the main thread.
    let mut renderer = match Renderer::new(surface) {
      Ok(renderer) => renderer,

      Err(err) => {
        let _ = send_result.send(Err(err));
        return;
      }
    };

    if send_result.send(Ok(())).is_err() {
      return;
    }

    // Run the renderer indefinitely (until the window is closed).
    log::info!(&logger, "renderer started");

    loop {
      // Render a single frame or exit the loop on failure.
      if let Err(err) = renderer.render() {
        if err != RenderError::BackbufferAcquireFailed(SurfaceAcquireError::WindowClosed) {
          log::crit!(&logger, "could not render frame: {}", err);
        }

        break;
      }
    }

    drop(renderer);

    log::info!(&logger, "renderer stopped");
  });

  // Receive the handle or error from the render thread initialization.
  recv_result.recv()?
}

/// An error that occurred while starting a new renderer.
#[derive(Debug)]
pub enum StartError {
  /// Out of either device or host memory.
  OutOfMemory,
  /// An unknown error occurred.
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

// Implement `From` to convert from out of memory errors.
impl From<OutOfMemoryError> for StartError {
  fn from(_: OutOfMemoryError) -> Self {
    StartError::OutOfMemory
  }
}

// Implement `From` to convert from channel receive errors.
impl From<channel::RecvError> for StartError {
  fn from(_: channel::RecvError) -> Self {
    StartError::Unknown
  }
}

/// An error that occurred while rendering a frame.
#[derive(Debug, PartialEq, Eq)]
enum RenderError {
  /// Failed to acquire backbuffer.
  BackbufferAcquireFailed(SurfaceAcquireError),
  /// Failed to present backbuffer.
  BackbufferPresentFailed(SurfacePresentError),
  /// Out of either host or device memory.
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

// Implement `From` to convert from surface acquire errors.
impl From<SurfaceAcquireError> for RenderError {
  fn from(err: SurfaceAcquireError) -> Self {
    RenderError::BackbufferAcquireFailed(err)
  }
}

// Implement `From` to convert from surface present errors.
impl From<SurfacePresentError> for RenderError {
  fn from(err: SurfacePresentError) -> Self {
    RenderError::BackbufferPresentFailed(err)
  }
}

// Implement `From` to convert from out of memory errors.
impl From<OutOfMemoryError> for RenderError {
  fn from(_: OutOfMemoryError) -> Self {
    RenderError::OutOfMemory
  }
}
