// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::backend;
use crate::cmd::Pool;
use crate::renderer;
use gfx_hal::command::RawCommandBuffer as _;
use std::sync::atomic;

pub struct Buffer {
  pool: Pool,
  buffer: Option<backend::CommandBuffer>,
}

impl Buffer {
  pub fn new(pool: &Pool) -> Self {
    Self {
      buffer: Some(pool.allocate()),
      pool: pool.clone(),
    }
  }

  pub fn record(&mut self) -> Recorder {
    Recorder::new(&self.pool, self.buffer.as_mut().unwrap())
  }

  pub(crate) fn as_backend(&self) -> &backend::CommandBuffer {
    self.buffer.as_ref().unwrap()
  }
}

impl Drop for Buffer {
  fn drop(&mut self) {
    self.pool.recycle(self.buffer.take().unwrap());
  }
}

pub struct Recorder<'a> {
  pool: &'a Pool,
  buffer: &'a mut backend::CommandBuffer,
  in_render_pass: bool,
}

impl<'a> Recorder<'a> {
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
    }
  }

  pub fn begin_render_pass(&mut self, framebuffer: &mut renderer::Framebuffer) {
    framebuffer.ensure_created();

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

  pub fn end_render_pass(&mut self) {
    unsafe {
      self.buffer.end_render_pass();
    }

    self.in_render_pass = false;
  }

  pub fn finish(self) {}

  fn _finish(&mut self) {}
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
