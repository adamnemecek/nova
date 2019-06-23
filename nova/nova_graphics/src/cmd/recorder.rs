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

  /// Records a command to begin a render pass.
  pub fn begin_render_pass(&mut self, framebuffer: &mut Framebuffer) {
    debug_assert!(!self.in_render_pass, "already began a render pass");

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

  /// Records a command to bind a pipeline to use for draw commands.
  pub fn bind_pipeline(&mut self, pipeline: &Pipeline) {
    unsafe { self.buffer.bind_graphics_pipeline(pipeline.as_backend()) };

    self.bound_pipeline = Some(pipeline.clone());
  }

  /// Records a command to bind one or more descriptor sets to the current
  /// pipeline.
  pub fn bind_descriptor_sets<'b>(
    &mut self,
    first_index: usize,
    sets: impl IntoIterator<Item = &'b DescriptorSet>,
  ) {
    let pipeline = self
      .bound_pipeline
      .as_ref()
      .expect("no graphics pipeline bound");

    unsafe {
      self.buffer.bind_graphics_descriptor_sets(
        pipeline.as_backend_layout(),
        first_index,
        sets.into_iter().map(DescriptorSet::as_backend),
        &[],
      );
    }
  }

  /// Records a command to bind a buffer to the given vertex buffer index in
  /// the current pipeline.
  pub fn bind_vertex_buffer(&mut self, index: u32, buffer: &Buffer) {
    unsafe {
      self
        .buffer
        .bind_vertex_buffers(index, iter::once((buffer.as_backend(), 0)));
    }
  }

  /// Records a command to set the push constants for the current pipeline.
  ///
  /// `T` must be the same type as specified during pipeline creation or another
  /// type of equal size.
  pub fn push_constants<T: Sized>(&mut self, constants: &T) {
    let pipeline = self
      .bound_pipeline
      .as_ref()
      .expect("no graphics pipeline bound");

    let count = pipeline.push_constant_count();

    debug_assert_eq!(
      count * 4,
      mem::size_of::<T>(),
      "push constants size must be the same as specified by the pipeline"
    );

    unsafe {
      self.buffer.push_graphics_constants(
        pipeline.as_backend_layout(),
        gfx_hal::pso::ShaderStageFlags::ALL,
        0,
        slice::from_raw_parts(constants as *const T as *const u32, count),
      );
    }
  }

  /// Records a command to draw the given vertex range with the current
  /// pipeline.
  pub fn draw(&mut self, vertices: ops::Range<u32>) {
    unsafe { self.buffer.draw(vertices, 0..1) };
  }

  /// Records a command to end the current render pass.
  pub fn end_render_pass(&mut self) {
    debug_assert!(self.in_render_pass, "must have begun render pass");

    unsafe {
      self.buffer.end_render_pass();
    }

    self.in_render_pass = false;
  }

  /// Inserts a pipeline barrier to synchronize resource usage between commands
  /// recorded before and after.
  pub fn pipeline_barrier(&mut self, stages: ops::Range<PipelineStage>, barriers: &[Barrier<'_>]) {
    unsafe {
      self.buffer.pipeline_barrier(
        stages,
        gfx_hal::memory::Dependencies::empty(),
        barriers.iter().map(Barrier::as_backend),
      );
    }
  }

  /// Records a command to copy data from a source to a destination buffer.
  pub fn copy_buffer(
    &mut self,
    src: &Buffer,
    src_range: ops::Range<u64>,
    dest: &Buffer,
    dest_index: u64,
  ) {
    self.copy_buffer_regions(
      src,
      dest,
      iter::once(BufferCopy {
        src_range,
        dest_index,
      }),
    );
  }

  /// Records a command to copy multiple regions of a source buffer to a
  /// destination buffer.
  pub fn copy_buffer_regions(
    &mut self,
    src: &Buffer,
    dest: &Buffer,
    regions: impl IntoIterator<Item = BufferCopy>,
  ) {
    unsafe {
      self.buffer.copy_buffer(
        src.as_backend(),
        dest.as_backend(),
        regions
          .into_iter()
          .map(|copy| gfx_hal::command::BufferCopy {
            src: copy.src_range.start,
            dst: copy.dest_index,
            size: copy.src_range.end - copy.src_range.start,
          }),
      );
    }
  }

  /// Records a command to copy the data in a [`Buffer`] into an ['Image`].
  pub fn copy_buffer_to_image(
    &mut self,
    src: &Buffer,
    src_offset: u64,
    dest: &Image,
    dest_layout: ImageLayout,
    dest_rect: Rect<u32>,
  ) {
    unsafe {
      self.buffer.copy_buffer_to_image(
        src.as_backend(),
        dest.as_backend(),
        dest_layout,
        &[gfx_hal::command::BufferImageCopy {
          buffer_offset: src_offset,
          buffer_width: 0,
          buffer_height: 0,
          image_layers: gfx_hal::image::SubresourceLayers {
            aspects: gfx_hal::format::Aspects::COLOR,
            level: 0,
            layers: 0..1,
          },
          image_offset: gfx_hal::image::Offset {
            x: dest_rect.start.x as i32,
            y: dest_rect.start.y as i32,
            z: 0,
          },
          image_extent: gfx_hal::image::Extent {
            width: dest_rect.width(),
            height: dest_rect.height(),
            depth: 1,
          },
        }],
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
