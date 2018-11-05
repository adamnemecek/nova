use super::backend;
use super::prelude::*;
use super::{
  Buffer, DescriptorSet, DescriptorSetLayout, Device, Pipeline, RenderPass, Swapchain, Texture,
};
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
  pools: Option<backend::CommandPool>,
  pass: Arc<RenderPass>,
  device: Arc<Device>,
}

struct Frame {
  pipeline: Option<Arc<Pipeline>>,
  image: u32,
  command_buffer: backend::CommandBuffer,
  fence: backend::Fence,
  acquire_semaphore: backend::Semaphore,
  render_semaphore: backend::Semaphore,
}

impl Renderer {
  pub fn new(pass: &Arc<RenderPass>, descriptor_set_layout: &Arc<DescriptorSetLayout>) -> Self {
    let device = pass.device().clone();

    let mut command_pool = device.raw.create_command_pool(
      device.command_queue.family_id(),
      CommandPoolCreateFlags::RESET_INDIVIDUAL,
    );

    let mut command_buffers =
      command_pool.allocate(FRAME_COUNT, gfx_hal::command::RawLevel::Primary);

    let mut frames = SmallVec::new();

    while let Some(command_buffer) = command_buffers.pop() {
      frames.push(Frame {
        command_buffer,
        fence: device.raw.create_fence(true),
        acquire_semaphore: device.raw.create_semaphore(),
        render_semaphore: device.raw.create_semaphore(),
        image: 0,
        pipeline: None,
      });
    }

    Renderer {
      device,
      pass: pass.clone(),
      pools: Some(command_pool),
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

  pub fn bind_pipeline(&mut self, pipeline: &Arc<Pipeline>) {
    let frame = &mut self.frames[self.frame];
    let cmd = &mut frame.command_buffer;

    cmd.bind_graphics_pipeline(pipeline.raw());
    frame.pipeline = Some(pipeline.clone());
  }

  pub fn push_constant<T>(&mut self, index: usize, value: &T) {
    let frame = &mut self.frames[self.frame];
    let cmd = &mut frame.command_buffer;

    let pipeline = frame.pipeline.as_ref().expect("no pipeline is bound");
    let (stages, range) = pipeline.push_constant(index);

    let constants =
      unsafe { std::slice::from_raw_parts(value as *const T as *const u32, range.len()) };

    cmd.push_graphics_constants(pipeline.layout(), stages, range.start, constants);
  }

  pub fn bind_descriptor_set(&mut self, index: usize, set: &DescriptorSet) {
    let frame = &mut self.frames[self.frame];
    let cmd = &mut frame.command_buffer;

    let pipeline = frame.pipeline.as_ref().expect("no pipeline is bound");

    cmd.bind_graphics_descriptor_sets(pipeline.layout(), index, iter::once(set.raw()), &[]);
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

  pub fn draw_indexed(&mut self, indices: u32) {
    let frame = &mut self.frames[self.frame];
    let cmd = &mut frame.command_buffer;

    cmd.draw_indexed(0..indices, 0, 0..1);
  }

  pub fn present(&mut self, swapchain: &mut Swapchain) -> Result<(), PresentError> {
    let frame = &mut self.frames[self.frame];

    frame.pipeline = None;

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

    if let Some(command_pool) = self.pools.take() {
      self.device.raw.destroy_command_pool(command_pool);
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
