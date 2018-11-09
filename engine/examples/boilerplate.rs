use nova::graphics;
use nova::graphics::rendering;
use nova::graphics::window;
use nova::utils::Droppable;
use std::iter;

pub fn main() -> Result<(), String> {
  // Create a graphics context. This structure is actually just a container for
  // related resources, so all of the fields can be taken and used separately.
  let mut gfx = graphics::Context::new("nova/examples/boilerplate", 1)
    .map_err(|err| format!("Could not create graphics context: {}", err))?;

  // Create a renderer. The renderer is responsible for submitting command
  // buffers to the device's graphics queue.
  let mut renderer = rendering::Renderer::new(&gfx.queues.graphics);

  // Create a swapchain. The swapchain is an abstraction over a set of images
  // that can be presented onto a window surface.
  let mut swapchain = Droppable::<window::Swapchain>::dropped();

  // Create a command pool. Command pools can be used to allocated command
  // buffers. Any number of command buffers in a pool can be used at once, but
  // only one per pool can record new commands at a time.
  let command_pool = rendering::CommandPool::new(&gfx.queues.graphics);

  // Loop over a chain of three fences. Fences support synchronization between
  // the GPU and the CPU. Command buffers can be submitted with a fence to
  // signal when the commands have been executed. This fence chain rotates
  // between three fences to ensure that rendering gets no further than 3 frames
  // ahead of the GPU.
  //
  // TODO: Does this actually do anything in mailbox present mode?
  for fence in &mut graphics::device::Fence::chain(&gfx.device, 3) {
    // Update the window. This handles incoming events and updates the window
    // state to match.
    gfx.window.update();

    // If the window is closed, exit the loop and the program.
    if gfx.window.is_closed() {
      break;
    }

    // If the swapchain exists but is the wrong size, destroy it. The swapchain
    // size must be the same as the window size.
    if !swapchain.is_dropped() && swapchain.size() != gfx.window.size() {
      swapchain.drop();
    }

    // Acquire a framebuffer from the swapchain. Framebuffers are collections of
    // images that can be read from and/or written to by the renderer.
    //
    // When the framebuffer is acquired, the swapchain also returns a semaphore.
    // Semaphores signal the completion of an operation, like fences, but can be
    // waited on by the GPU in command buffers or during other operations to
    // control ordering. This semaphore is signaled when the swapchain image is
    // ready for rendering.
    let (framebuffer, framebuffer_semaphore) = loop {
      // Create the swapchain if it does not exist.
      if swapchain.is_dropped() {
        let size = gfx.window.size();

        swapchain =
          window::Swapchain::new(renderer.pass(), gfx.window.raw_surface_mut(), size).into();
      }

      // Attempt to acquire the swapchain. If the swapchain is out of date,
      // destroy it and then loop back around to recreate it above.
      match swapchain.acquire_framebuffer() {
        Ok(fb) => break fb,
        Err(window::AcquireFramebufferError::SwapchainOutOfDate) => swapchain.drop(),
      };
    };

    // Create a primary command buffer to record rendering commands with. This
    // buffer is created and used each frame, although in this case the buffer
    // could be pre-recorded and reused since the commands are the same each
    // frame.
    let mut cmd =
      rendering::CommandBuffer::new(&command_pool, rendering::CommandBufferKind::Primary);

    // Begin recording. This function must be called before any recording
    // functions. Only one command buffer from a pool can be recorded at once.
    cmd.begin();

    // Begin the render pass with the acquired framebuffer.
    cmd.begin_pass(renderer.pass(), &framebuffer);

    // Finish recording. This function must be called before submitting the
    // command buffer.
    cmd.finish();

    // Wait for the fence to be signaled so the program doesn't get ahead of the
    // GPU.
    fence.wait();

    // Submit the command buffer with the renderer. Rendering will wait for the
    // framebuffer semaphore to be signaled. The renderer returns another
    // semaphore that is signaled when all commands finish executing.
    let render_semaphore = renderer.render(
      &framebuffer,
      iter::once(cmd),
      &framebuffer_semaphore,
      Some(fence),
    );

    // Submit the acquired swapchain image for presentation. Presentation will
    // wait for the render semaphore to be signaled.
    let result = gfx.queues.graphics.present(
      iter::once((swapchain.as_ref(), framebuffer.index())),
      iter::once(render_semaphore),
    );

    // If presentation failed, it probably means the swapchain is out of date.
    // Destroy it and try again next frame.
    if let Err(_) = result {
      swapchain.drop();
    }
  }

  Ok(())
}
