// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod pool;

use super::{Backend, Framebuffer, QueueId, RenderPass};
use crate::utils::Droppable;

pub use self::pool::*;

use gfx_hal::command::RawCommandBuffer as RawCommandsExt;

type RawCommandBuffer = <Backend as gfx_hal::Backend>::CommandBuffer;

pub struct Commands {
  raw: Droppable<RawCommandBuffer>,
  pool: CommandPool,
}

impl Commands {
  pub fn queue_id(&self) -> QueueId {
    self.pool.queue_id()
  }

  pub(super) fn raw(&self) -> &RawCommandBuffer {
    &self.raw
  }

  pub fn begin(&mut self) {
    unsafe {
      self.raw.begin(Default::default(), Default::default());
    }
  }

  pub fn begin_render_pass(&mut self, render_pass: &RenderPass, framebuffer: &Framebuffer) {
    // Convert the framebuffer size from `u32` to `i16`.
    let size = framebuffer.size().vector.map(|u| u as i16);

    // Create a viewport struct covering the entire framebuffer.
    let viewport = gfx_hal::pso::Viewport {
      rect: gfx_hal::pso::Rect {
        x: 0,
        y: 0,
        w: size.x,
        h: size.y,
      },
      depth: 0.0..1.0,
    };

    // Begin the render pass.
    unsafe {
      self.raw.set_viewports(0, &[viewport.clone()]);
      self.raw.set_scissors(0, &[viewport.rect]);

      self.raw.begin_render_pass(
        render_pass.raw(),
        framebuffer.raw(),
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
  }

  pub fn finish_render_pass(&mut self) {
    unsafe {
      self.raw.end_render_pass();
    }
  }

  pub fn finish(&mut self) {
    unsafe {
      self.raw.finish();
    }
  }
}

impl Drop for Commands {
  fn drop(&mut self) {
    if let Some(raw) = self.raw.take() {
      self.pool.release_raw(raw);
    }
  }
}
