use super::{Pipeline, RenderTarget};
use gfx_hal::command::{CommandBufferFlags, RawCommandBuffer};
use gfx_hal::queue::RawCommandQueue;
use gfx_hal::{Device, Swapchain};
use smallvec::SmallVec;

pub struct BeginError;

pub fn begin(target: &mut RenderTarget) -> Result<(), BeginError> {
  let device = &target.context.device;
  let state = &mut target.states[target.current_state];

  device.wait_for_fence(&state.fence, !0);

  let swapchain = target.swapchain.as_mut().expect("swapchain was destroyed");

  state.image = swapchain
    .acquire_image(!0, gfx_hal::FrameSync::Semaphore(&state.acquire_semaphore))
    .map_err(|_| BeginError)?;

  state
    .command_buffer
    .begin(CommandBufferFlags::ONE_TIME_SUBMIT, Default::default());

  let viewport = gfx_hal::pso::Viewport {
    rect: gfx_hal::pso::Rect {
      x: 0,
      y: 0,
      w: target.width as i16,
      h: target.height as i16,
    },
    depth: 0.0..1.0,
  };

  state.command_buffer.set_viewports(0, &[viewport.clone()]);

  state.command_buffer.begin_render_pass(
    target.render_pass.raw(),
    &target.images[state.image as usize].framebuffer,
    viewport.rect,
    &[
      gfx_hal::command::ClearValue::Color(gfx_hal::command::ClearColor::Float([
        1.0, 0.0, 0.0, 1.0,
      ])).into(),
    ],
    gfx_hal::command::SubpassContents::Inline,
  );

  Ok(())
}

pub fn bind_pipeline(target: &mut RenderTarget, pipeline: &Pipeline) {
  let state = &mut target.states[target.current_state];

  state.command_buffer.bind_graphics_pipeline(pipeline.raw());
}

pub fn draw(target: &mut RenderTarget) {
  let state = &mut target.states[target.current_state];

  state.command_buffer.draw(0..3, 0..1);
}

pub fn end(target: &mut RenderTarget) -> Result<(), ()> {
  let current_state = target.current_state;

  target.current_state = (current_state + 1) % target.states.len();

  let state = &mut target.states[current_state];

  state.command_buffer.finish();

  let mut cmd_buffers = SmallVec::<[_; 1]>::new();

  cmd_buffers.push(&state.command_buffer);

  unsafe {
    target.command_queue.submit_raw(
      gfx_hal::queue::RawSubmission {
        cmd_buffers,
        wait_semaphores: &[(
          &state.acquire_semaphore,
          gfx_hal::pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
        )],
        signal_semaphores: &[&state.render_semaphore],
      },
      Some(&state.fence),
    );
  }

  let swapchain = target.swapchain.as_ref().expect("swapchain was destroyed");

  let mut swapchains = SmallVec::<[_; 1]>::new();
  let mut wait_semaphores = SmallVec::<[_; 1]>::new();

  swapchains.push((swapchain, state.image));
  wait_semaphores.push(&state.render_semaphore);

  let result = target.command_queue.present(swapchains, wait_semaphores);

  result
}
