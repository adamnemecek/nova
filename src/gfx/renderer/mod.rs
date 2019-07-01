// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Starts a new renderer with the given options.
pub fn start(
  thread_scope: &thread::Scope,
  context: Arc<Context>,
  window: &window::Handle,
  logger: log::Logger,
) -> Result<(), StartError> {
  let context = context.clone();
  let logger = logger.clone();

  // Create resources needed for rendering.
  let mut surface = Surface::new(&context, window, &logger);
  let queue_id = context.queues().find_graphics_queue();
  let cmd_pool = cmd::Pool::new(&context, queue_id)?;
  let acquire_semaphore = cmd::Semaphore::new(&context)?;
  let render_semaphore = cmd::Semaphore::new(&context)?;
  let frame_fence = cmd::Fence::new(&context, false)?;
  let render_pass = RenderPass::new(&context).into();
  let mut framebuffer = Framebuffer::new(&context);

  framebuffer.set_render_pass(&render_pass);

  // Start a thread to run the render loop.
  thread_scope.spawn(move |_| {
    let cmd_pool = cmd_pool.into_ref_cell();

    // Run the renderer indefinitely (until the window is closed).
    log::info!(&logger, "renderer started");

    loop {
      // Render a single frame or exit the loop on failure.
      let mut render = || -> Result<(), RenderError> {
        let backbuffer = surface.acquire(&acquire_semaphore)?;

        framebuffer.set_attachment(backbuffer.image());
        framebuffer.ensure_created()?;

        let mut cmd_list = cmd::List::new(&cmd_pool);
        let mut cmd = cmd_list.begin();

        cmd.begin_render_pass(&framebuffer);
        cmd.end_render_pass();

        cmd.end();

        context.queues().submit(cmd::Submission {
          queue_id: cmd_list.queue_id(),
          lists: &[&cmd_list],
          wait_semaphores: &[(&acquire_semaphore, pipeline::Stage::COLOR_ATTACHMENT_OUTPUT)],
          signal_semaphores: &[&render_semaphore],
          fence: &frame_fence,
        });

        backbuffer.present(&[&render_semaphore])?;

        frame_fence.wait_and_reset();

        Ok(())
      };

      if let Err(err) = render() {
        if err != RenderError::BackbufferAcquireFailed(SurfaceAcquireError::WindowClosed) {
          log::crit!(&logger, "could not render frame: {}", err);
        }

        break;
      }
    }

    // Wait for the device to be idle before shutting down.
    context.wait_idle();

    log::info!(&logger, "renderer stopped");
  });

  Ok(())
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
impl From<oneshot::Canceled> for StartError {
  fn from(_: oneshot::Canceled) -> Self {
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
