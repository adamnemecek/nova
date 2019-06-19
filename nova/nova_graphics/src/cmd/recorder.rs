// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use std::sync::atomic;

/// Records commands into a command list.
///
/// Recording finishes when this structure is dropped or when the `finish()`
/// method is called.
pub struct Recorder<'a> {
  pool: &'a Pool,
  buffer: &'a mut backend::CommandBuffer,
  in_render_pass: bool,
  bound_pipeline: Option<Pipeline>,
}

impl<'a> Recorder<'a> {
  /// Creates a new recorder for the given buffer and begins recording.
  pub(crate) fn new(pool: &'a Pool, buffer: &'a mut backend::CommandBuffer) -> Self {
    if pool.is_recording().swap(true, atomic::Ordering::Acquire) {
      panic!("can only record commands using one command buffer at a time per pool");
    }

    unsafe {
      buffer.begin(
        gfx_hal::command::CommandBufferFlags::EMPTY,
        Default::default(),
      );
    }

    Self {
      pool,
      buffer,
      in_render_pass: false,
      bound_pipeline: None,
    }
  }

  /// Begins a render pass using the given framebuffer.
  pub fn begin_render_pass(&mut self, framebuffer: &mut Framebuffer) {
    let render_pass = framebuffer.render_pass().unwrap();
    let size = framebuffer.size();

    let viewport = gfx_hal::pso::Viewport {
      rect: gfx_hal::pso::Rect {
        x: 0,
        y: 0,
        w: size.width as i16,
        h: size.height as i16,
      },
      depth: 0.0..1.0,
    };

    unsafe {
      self.buffer.set_viewports(0, &[viewport.clone()]);
      self.buffer.set_scissors(0, &[viewport.rect]);

      self.buffer.begin_render_pass(
        render_pass.as_backend(),
        framebuffer.as_backend(),
        viewport.rect,
        &[
          // Clear the framebuffer to eigengrau.
          gfx_hal::command::ClearValue::Color(gfx_hal::command::ClearColor::Float([
            0.086, 0.086, 0.114, 1.0,
          ]))
          .into(),
        ],
        gfx_hal::command::SubpassContents::Inline,
      );
    }

    self.in_render_pass = true;
  }

  /// Binds the given graphics pipeline for future commands in the render pass.
  pub fn bind_pipeline(&mut self, pipeline: &Pipeline) {
    unsafe { self.buffer.bind_graphics_pipeline(pipeline.as_backend()) };

    self.bound_pipeline = Some(pipeline.clone());
  }

  pub fn bind_vertex_buffer<T>(&mut self, index: u32, buffer: &Buffer<T>) {
    unsafe {
      self
        .buffer
        .bind_vertex_buffers(index, iter::once((buffer.as_backend(), 0)));
    }
  }

  /// Set the graphics pipeline push constants to the given value.
  ///
  /// The size of type `T` must match the `size_of_push_constants` option of
  /// the graphics pipeline.
  pub fn push_constants<T: Sized>(&mut self, constants: &T) {
    let pipeline = self
      .bound_pipeline
      .as_ref()
      .expect("no graphics pipeline bound");

    let count = pipeline.push_constant_count();

    debug_assert_eq!(count * 4, mem::size_of::<T>(), "The push constants type must be the same size as the pipeline's size_of_push_constants option.");

    unsafe {
      self.buffer.push_graphics_constants(
        pipeline.backend_layout(),
        gfx_hal::pso::ShaderStageFlags::ALL,
        0,
        slice::from_raw_parts(constants as *const T as *const u32, count),
      );
    }
  }

  /// Draws the given vertex range.
  pub fn draw(&mut self, vertices: ops::Range<u32>) {
    unsafe { self.buffer.draw(vertices, 0..1) };
  }

  /// Ends the current render pass.
  pub fn end_render_pass(&mut self) {
    unsafe {
      self.buffer.end_render_pass();
    }

    self.in_render_pass = false;
  }

  /// Inserts a pipeline barrier between the given stages with one or more
  /// memory barriers based on the given descriptions.
  pub fn pipeline_barrier(&mut self, stages: ops::Range<PipelineStage>, barriers: &[Barrier<'_>]) {
    unsafe {
      self.buffer.pipeline_barrier(
        stages,
        gfx_hal::memory::Dependencies::empty(),
        barriers.iter().map(Barrier::as_backend),
      );
    }
  }

  /// Copies data from a source buffer to a destination buffer.
  pub fn copy_buffer<T>(
    &mut self,
    source: &Buffer<T>,
    src_range: ops::Range<u64>,
    destination: &Buffer<T>,
    dst_offset: u64,
  ) {
    self.copy_buffer_regions(
      source,
      destination,
      iter::once(BufferCopy {
        src_range,
        dst_offset,
      }),
    );
  }

  /// Copies multiple regions of data from a source buffer into a destination
  /// buffer.
  pub fn copy_buffer_regions<T>(
    &mut self,
    source: &Buffer<T>,
    destination: &Buffer<T>,
    regions: impl IntoIterator<Item = BufferCopy>,
  ) {
    let size_of = mem::size_of::<T>() as u64;

    unsafe {
      self.buffer.copy_buffer(
        source.as_backend(),
        destination.as_backend(),
        regions.into_iter().map(|copy| {
          let src = copy.src_range.start * size_of;
          let dst = copy.dst_offset * size_of;
          let size = copy.src_range.end * size_of - src;

          gfx_hal::command::BufferCopy { src, dst, size }
        }),
      );
    }
  }

  /// Finishes recording commands, dropping the `Recorder`.
  pub fn finish(self) {}
}

impl<'a> Drop for Recorder<'a> {
  fn drop(&mut self) {
    if self.in_render_pass {
      self.end_render_pass();
    }

    unsafe {
      self.buffer.finish();
    }

    self
      .pool
      .is_recording()
      .store(false, atomic::Ordering::Release);
  }
}
