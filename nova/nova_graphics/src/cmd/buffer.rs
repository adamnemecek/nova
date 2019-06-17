// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::cmd::Pool;
use crate::{backend, pipeline, renderer};
use gfx_hal::command::RawCommandBuffer as _;
use std::sync::atomic;
use std::{mem, slice, ops};

/// Buffer for recording commands to submit to a device queue.
pub struct Buffer {
  pool: Pool,
  buffer: Option<backend::CommandBuffer>,
}

impl Buffer {
  /// Creates a new command buffer using the given pool.
  pub fn new(pool: &Pool) -> Self {
    Self {
      buffer: Some(pool.allocate()),
      pool: pool.clone(),
    }
  }

  /// Begins recording commands.
  ///
  /// This function returns a `Recorder` structure for recording the actual
  /// commands. Recording is finished when the structure is dropped or when
  /// the `Recorder::finish` function is called.
  pub fn record(&mut self) -> Recorder {
    Recorder::new(&self.pool, self.buffer.as_mut().unwrap())
  }

  /// Returns a reference to the underlying backend command buffer.
  pub(crate) fn as_backend(&self) -> &backend::CommandBuffer {
    self.buffer.as_ref().unwrap()
  }
}

impl Drop for Buffer {
  fn drop(&mut self) {
    self.pool.recycle(self.buffer.take().unwrap());
  }
}

/// Structure for recording commands into a `Buffer`.
pub struct Recorder<'a> {
  pool: &'a Pool,
  buffer: &'a mut backend::CommandBuffer,
  in_render_pass: bool,
  graphics_pipeline: Option<pipeline::Graphics>,
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
      graphics_pipeline: None,
    }
  }

  /// Begins a render pass using the given framebuffer.
  pub fn begin_render_pass(&mut self, framebuffer: &mut renderer::Framebuffer) {
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
            //0.086, 0.086, 0.114, 1.0,
            1.0, 0.0, 0.0, 1.0,
          ]))
          .into(),
        ],
        gfx_hal::command::SubpassContents::Inline,
      );
    }

    self.in_render_pass = true;
  }

  /// Binds the given graphics pipeline for future commands in the render pass.
  pub fn bind_graphics_pipeline(&mut self, pipeline: &pipeline::Graphics) {
    unsafe { self.buffer.bind_graphics_pipeline(pipeline.as_backend()) };

    self.graphics_pipeline = Some(pipeline.clone());
  }


  /// Set the push constants to the given value.
  ///
  /// The size of type `T` must match the `size_of_push_constants` option of
  /// the graphics pipeline.
  pub fn push_graphics_constants<T: Sized>(&mut self, constants: &T) {
    let pipeline = self
      .graphics_pipeline
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

  /// Binds the given graphics pipeline for future commands in the render pass.
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
