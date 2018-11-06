pub use gfx_hal::command::RawLevel as CommandBufferKind;

use super::*;
use smallvec::SmallVec;
use std::iter;
use std::sync::atomic;
use std::sync::Arc;

pub struct CommandBuffer {
  pool: Arc<CommandPool>,
  raw: Option<Vec<backend::CommandBuffer>>,
  recording: bool,
  passes: SmallVec<[Arc<RenderPass>; 1]>,
  secondaries: Vec<CommandBuffer>,
  pipelines: SmallVec<[Arc<Pipeline>; 1]>,
}

impl CommandBuffer {
  pub fn new(pool: &Arc<CommandPool>, kind: CommandBufferKind) -> CommandBuffer {
    let buffers = pool.raw_mut().allocate(1, kind);

    CommandBuffer {
      pool: pool.clone(),
      raw: Some(buffers),
      recording: true,
      passes: SmallVec::new(),
      secondaries: Vec::new(),
      pipelines: SmallVec::new(),
    }
  }

  pub fn begin(&mut self) {
    self.start_recording();
    self.raw_mut().begin(Default::default(), Default::default());
  }

  pub fn begin_pass(&mut self, pass: &Arc<RenderPass>, framebuffer: &Arc<Framebuffer>) {
    let cmd = self.raw_mut();

    let viewport = gfx_hal::pso::Viewport {
      rect: gfx_hal::pso::Rect {
        x: 0,
        y: 0,
        w: framebuffer.width(),
        h: framebuffer.height(),
      },
      depth: 0.0..1.0,
    };

    cmd.set_viewports(0, &[viewport.clone()]);

    cmd.begin_render_pass(
      pass.raw(),
      framebuffer.raw(),
      viewport.rect,
      &[
        gfx_hal::command::ClearValue::Color(gfx_hal::command::ClearColor::Float([
          0.086, 0.086, 0.114, 1.0,
        ]))
        .into(),
      ],
      gfx_hal::command::SubpassContents::Inline,
    );

    self.passes.push(pass.clone());
  }

  pub fn begin_in_pass(&mut self, pass: &Arc<RenderPass>, framebuffer: &Framebuffer) {
    self.start_recording();

    self.raw_mut().begin(
      Default::default(),
      backend::CommandBufferInheritanceInfo {
        subpass: Some(gfx_hal::pass::Subpass {
          index: 0,
          main_pass: pass.raw(),
        }),
        framebuffer: Some(framebuffer.raw()),
        ..Default::default()
      },
    );

    self.passes.push(pass.clone());
  }

  pub fn execute_commands(&mut self, buffer: CommandBuffer) {
    self.raw_mut().execute_commands(iter::once(buffer.raw()));
    self.secondaries.push(buffer);
  }

  pub fn bind_pipeline(&mut self, pipeline: &Arc<Pipeline>) {
    self.raw_mut().bind_graphics_pipeline(pipeline.raw());
    self.pipelines.push(pipeline.clone());
  }

  pub fn push_constant<T>(&mut self, index: usize, value: &T) {
    let pipeline = self.pipelines.last().expect("no pipeline is bound");
    let (stages, range) = pipeline.push_constant(index);

    let constants =
      unsafe { std::slice::from_raw_parts(value as *const T as *const u32, range.len()) };

    self.raw.as_mut().unwrap()[0].push_graphics_constants(
      pipeline.layout(),
      stages,
      range.start,
      constants,
    );
  }

  pub fn bind_descriptor_set(&mut self, index: usize, set: &DescriptorSet) {
    let pipeline = self.pipelines.last().expect("no pipeline is bound");

    self.raw.as_mut().unwrap()[0].bind_graphics_descriptor_sets(
      pipeline.layout(),
      index,
      iter::once(set.raw()),
      &[0],
    );
  }

  pub fn bind_vertex_buffer<T: Copy>(&mut self, binding: u32, buffer: &Buffer<T>) {
    self
      .raw_mut()
      .bind_vertex_buffers(binding, iter::once((buffer.raw(), 0)));
  }

  pub fn bind_index_buffer(&mut self, buffer: &Buffer<u16>) {
    self
      .raw_mut()
      .bind_index_buffer(gfx_hal::buffer::IndexBufferView {
        buffer: buffer.raw(),
        offset: 0,
        index_type: gfx_hal::IndexType::U16,
      })
  }

  pub fn draw_indexed(&mut self, indices: u32) {
    self.raw_mut().draw_indexed(0..indices, 0, 0..1);
  }

  pub fn finish(&mut self) {
    assert!(self.recording, "command buffer is not recording");

    self.raw_mut().finish();
    self.stop_recording();
  }

  pub fn raw(&self) -> &backend::CommandBuffer {
    &self.raw.as_ref().unwrap()[0]
  }

  pub fn raw_mut(&mut self) -> &mut backend::CommandBuffer {
    &mut self.raw.as_mut().unwrap()[0]
  }

  pub fn record_raw<R>(&mut self, f: impl FnOnce(&mut backend::CommandBuffer) -> R) -> R {
    f(self.raw_mut())
  }

  fn start_recording(&mut self) {
    if !self.recording {
      let was_recording =
        self
          .pool
          .recording
          .compare_and_swap(false, true, atomic::Ordering::Relaxed);

      if was_recording {
        panic!("The command pool is already recording a command buffer.");
      }

      self.recording = true;
    }

    self.passes.clear();
    self.pipelines.clear();
  }

  fn stop_recording(&mut self) {
    self.pool.recording.store(false, atomic::Ordering::Relaxed);
    self.recording = false;
  }
}

impl Drop for CommandBuffer {
  fn drop(&mut self) {
    if self.recording {
      self.stop_recording();
    }

    let buffers = self.raw.take().unwrap();

    unsafe {
      self.pool.raw_mut().free(buffers);
    }
  }
}
