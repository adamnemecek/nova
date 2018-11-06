use super::*;
use smallvec::SmallVec;
use std::iter;
use std::sync::Arc;

pub const FRAME_COUNT: usize = 3;

pub struct Renderer {
  device: Arc<Device>,
  frames: SmallVec<[Frame; FRAME_COUNT]>,
  frame: usize,
}

struct Frame {
  fence: backend::Fence,
  acquire_semaphore: backend::Semaphore,
  render_semaphore: backend::Semaphore,
  command_buffers: Vec<CommandBuffer>,
}

impl Renderer {
  pub fn new(device: &Arc<Device>) -> Self {
    let mut frames = SmallVec::new();

    for _ in 0..FRAME_COUNT {
      frames.push(Frame {
        fence: device.raw.create_fence(true),
        acquire_semaphore: device.raw.create_semaphore(),
        render_semaphore: device.raw.create_semaphore(),
        command_buffers: Vec::new(),
      });
    }

    Renderer {
      device: device.clone(),
      frames,
      frame: 0,
    }
  }

  pub fn render<R>(
    &mut self,
    swapchain: &mut Swapchain,
    func: impl FnOnce(Arc<Framebuffer>) -> R,
  ) -> Result<(), RenderError>
  where
    R: IntoIterator<Item = CommandBuffer>,
  {
    let device = &self.device;
    let frame = &mut self.frames[self.frame];

    device.raw.wait_for_fence(&frame.fence, !0);

    frame.command_buffers.clear();

    let framebuffer = swapchain
      .acquire_framebuffer(&frame.acquire_semaphore)
      .map_err(|err| match err {
        gfx_hal::AcquireError::OutOfDate => RenderError::SwapchainOutOfDate,

        _ => {
          panic!("could not acquire framebuffer");
        }
      })?;

    let fb_index = framebuffer.index();

    frame.command_buffers.extend(func(framebuffer));

    let mut queue = self.device.queues.graphics().raw_mut();

    unsafe {
      queue.submit_raw(
        gfx_hal::queue::RawSubmission {
          cmd_buffers: frame.command_buffers.iter().map(CommandBuffer::raw),
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
        iter::once((swapchain.raw_mut(), fb_index)),
        iter::once(&frame.render_semaphore),
      )
      .or(Err(RenderError::SwapchainOutOfDate)) // Usually what happened.
  }
}

impl Drop for Renderer {
  fn drop(&mut self) {
    for frame in self.frames.drain() {
      let device = &self.device.raw;

      device.destroy_fence(frame.fence);
      device.destroy_semaphore(frame.acquire_semaphore);
      device.destroy_semaphore(frame.render_semaphore);
    }
  }
}

quick_error! {
  #[derive(Debug)]
  pub enum RenderError {
    SwapchainOutOfDate {
      display("The given swapchain is out of date and must be recreated.")
    }
  }
}

/*
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

  cmd.finish();
  */
