use crate::prelude::*;
use gfx_hal as hal;
use gfx_hal::{Device, Swapchain};

mod backend;
mod canvas;
mod mesh;
mod renderer;
mod shaders;
mod target;

pub use self::backend::*;
pub use self::canvas::*;
pub use self::renderer::*;
pub use self::target::*;

pub fn render(renderer: &mut Renderer, target: &mut RenderTarget, draw: impl FnOnce(&mut Canvas)) {
  renderer.device.reset_fence(&target.frame_fence);
  renderer.command_pool.reset();

  // A swapchain contains multiple images - which one should we draw on? This
  // returns the index of the image we'll use. The image may not be ready for
  // rendering yet, but will signal frame_semaphore when it is.
  let frame_index = target
    .swapchain
    .acquire_image(
      !0, /* no timeout */
      hal::FrameSync::Semaphore(&target.frame_semaphore),
    ).expect("could not acquire frame");

  let mut command_buffer = renderer.command_pool.acquire_command_buffer(false);

  // Define a rectangle on screen to draw into.
  // In this case, the whole screen.
  let viewport = hal::pso::Viewport {
    rect: hal::pso::Rect {
      x: 0,
      y: 0,
      w: target.extent.width as i16,
      h: target.extent.height as i16,
    },
    depth: 0.0..1.0,
  };

  command_buffer.set_viewports(0, &[viewport.clone()]);
  command_buffer.set_scissors(0, &[viewport.rect]);

  // Choose a pipeline to use.
  command_buffer.bind_graphics_pipeline(&target.pipeline);

  // Write to the buffer for submission.
  let submit = {
    {
      // Clear the screen and begin the render pass.
      let encoder = command_buffer.begin_render_pass_inline(
        &target.render_pass,
        &target.framebuffers[frame_index as usize],
        viewport.rect,
        &[hal::command::ClearValue::Color(
          hal::command::ClearColor::Float([1.0, 0.0, 0.0, 1.0]),
        )],
      );

      let mut canvas = Canvas {
        size: Vector2::new(target.extent.width, target.extent.height),
        encoder,
      };

      draw(&mut canvas);
    }

    // Finish building the command buffer - it's now ready to send to the
    // GPU.
    command_buffer.finish()
  };

  // This is what we submit to the command queue. We wait until frame_semaphore
  // is signalled, at which point we know our chosen image is available to draw
  // on.
  let submission = hal::queue::Submission::new()
    .wait_on(&[(
      &target.frame_semaphore,
      hal::pso::PipelineStage::BOTTOM_OF_PIPE,
    )]).submit(vec![submit]);

  // We submit the submission to one of our command queues, which will signal
  // frame_fence once rendering is completed.
  renderer.queue_group.queues[0].submit(submission, Some(&target.frame_fence));

  // We first wait for the rendering to complete...
  renderer.device.wait_for_fence(&target.frame_fence, !0);

  // ...and then present the image on screen!
  target
    .swapchain
    .present(&mut renderer.queue_group.queues[0], frame_index, &[])
    .expect("could not present image");
}
