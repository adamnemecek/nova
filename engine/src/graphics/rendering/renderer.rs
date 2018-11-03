use super::backend;
use super::prelude::*;
use super::{Buffer, Device, Pipeline, RenderPass, Swapchain};
use gfx_hal::command::CommandBufferFlags;
use gfx_hal::pool::CommandPoolCreateFlags;
use quick_error::quick_error;
use smallvec::SmallVec;
use std::iter;
use std::sync::Arc;

pub const FRAME_COUNT: usize = 3;

pub struct Renderer {
  frame: usize,
  frames: SmallVec<[Frame; FRAME_COUNT]>,
  command_pool: Option<backend::CommandPool>,
  pass: Arc<RenderPass>,
  device: Arc<Device>,
}

struct Frame {
  command_buffer: backend::CommandBuffer,
  fence: backend::Fence,
  acquire_semaphore: backend::Semaphore,
  render_semaphore: backend::Semaphore,
  image: u32,
}

impl Renderer {
  pub fn new(pass: &Arc<RenderPass>) -> Self {
    let pass = pass.clone();
    let device = pass.device().clone();

    let mut command_pool = device.raw.create_command_pool(
      device.command_queue.family_id(),
      CommandPoolCreateFlags::TRANSIENT | CommandPoolCreateFlags::RESET_INDIVIDUAL,
    );

    let command_buffers = command_pool.allocate(FRAME_COUNT, gfx_hal::command::RawLevel::Primary);

    let frames = command_buffers
      .into_iter()
      .map(|command_buffer| Frame {
        command_buffer,
        fence: device.raw.create_fence(true),
        acquire_semaphore: device.raw.create_semaphore(),
        render_semaphore: device.raw.create_semaphore(),
        image: 0,
      })
      .collect();

    Renderer {
      device,
      pass,
      command_pool: Some(command_pool),
      frames,
      frame: 0,
    }
  }

  pub fn pass(&self) -> &Arc<RenderPass> {
    &self.pass
  }

  pub fn begin(&mut self, swapchain: &mut Swapchain) -> Result<(), BeginRenderError> {
    let frame = &mut self.frames[self.frame];
    let cmd = &mut frame.command_buffer;
    let device = &self.device;

    device.raw.wait_for_fence(&frame.fence, !0);

    frame.image = swapchain
      .raw_mut()
      .acquire_image(!0, gfx_hal::FrameSync::Semaphore(&frame.acquire_semaphore))
      .map_err(|err| match err {
        gfx_hal::AcquireError::OutOfDate => BeginRenderError::SwapchainOutOfDate,
        gfx_hal::AcquireError::NotReady | gfx_hal::AcquireError::SurfaceLost => {
          BeginRenderError::SurfaceLost
        }
      })?;

    cmd.begin(CommandBufferFlags::ONE_TIME_SUBMIT, Default::default());

    let viewport = gfx_hal::pso::Viewport {
      rect: gfx_hal::pso::Rect {
        x: 0,
        y: 0,
        w: swapchain.width() as i16,
        h: swapchain.height() as i16,
      },
      depth: 0.0..1.0,
    };

    cmd.set_viewports(0, &[viewport.clone()]);

    cmd.begin_render_pass(
      self.pass.raw(),
      swapchain.framebuffer(frame.image),
      viewport.rect,
      &[
        gfx_hal::command::ClearValue::Color(gfx_hal::command::ClearColor::Float([
          0.0080232, 0.0080232, 0.0122865, 1.0,
        ]))
        .into(),
      ],
      gfx_hal::command::SubpassContents::Inline,
    );

    Ok(())
  }

  pub fn bind_vertex_buffer<T: Copy>(&mut self, binding: u32, buffer: &Buffer<T>) {
    let frame = &mut self.frames[self.frame];
    let cmd = &mut frame.command_buffer;

    cmd.bind_vertex_buffers(binding, iter::once((buffer.raw(), 0)));
  }

  pub fn bind_index_buffer(&mut self, buffer: &Buffer<u16>) {
    let frame = &mut self.frames[self.frame];
    let cmd = &mut frame.command_buffer;

    cmd.bind_index_buffer(gfx_hal::buffer::IndexBufferView {
      buffer: buffer.raw(),
      offset: 0,
      index_type: gfx_hal::IndexType::U16,
    })
  }

  pub fn bind_pipeline(&mut self, pipeline: &Pipeline) {
    let frame = &mut self.frames[self.frame];
    let cmd = &mut frame.command_buffer;

    cmd.bind_graphics_pipeline(pipeline.raw());
  }

  pub fn draw_indexed(&mut self, indices: u32) {
    let frame = &mut self.frames[self.frame];
    let cmd = &mut frame.command_buffer;

    cmd.draw_indexed(0..indices, 0, 0..1);
  }

  pub fn present(&mut self, swapchain: &mut Swapchain) -> Result<(), PresentError> {
    use std::iter;

    let frame = &mut self.frames[self.frame];
    let cmd = &mut frame.command_buffer;
    let mut queue = self.device.command_queue.raw_mut();

    cmd.finish();

    unsafe {
      queue.submit_raw(
        gfx_hal::queue::RawSubmission {
          cmd_buffers: iter::once(cmd),
          wait_semaphores: &[(
            &frame.acquire_semaphore,
            gfx_hal::pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
          )],
          signal_semaphores: &[&frame.render_semaphore],
        },
        Some(&frame.fence),
      );
    }

    queue
      .present(
        iter::once((swapchain.raw_mut(), frame.image)),
        iter::once(&frame.render_semaphore),
      )
      .or(Err(PresentError::SwapchainOutOfDate)) // Usually what happened.
  }
}

impl Drop for Renderer {
  fn drop(&mut self) {
    for frame in self.frames.drain() {
      self.device.raw.destroy_fence(frame.fence);
      self.device.raw.destroy_semaphore(frame.acquire_semaphore);
      self.device.raw.destroy_semaphore(frame.render_semaphore);
    }

    if let Some(pool) = self.command_pool.take() {
      self.device.raw.destroy_command_pool(pool);
    }
  }
}

quick_error! {
  #[derive(Debug)]
  pub enum BeginRenderError {
    SwapchainOutOfDate {
      display("The given swapchain is out of date and must be recreated.")
    }
    SurfaceLost {
      display("The render surface was destroyed.")
    }
  }
}

quick_error! {
  #[derive(Debug)]
  pub enum PresentError {
    SwapchainOutOfDate {
      display("The given swapchain is out of date and must be recreated.")
    }
  }
}
