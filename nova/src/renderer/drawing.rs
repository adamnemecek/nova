// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::commands::{CommandBufferExt, Commands};
use super::pipeline::Pipeline;
use crate::engine;

pub trait Drawable: Sized {
  fn draw(&mut self, cmd: DrawCommands, res: &engine::Resources);
}

pub struct DrawCommands<'a> {
  cmd: &'a mut Commands,
}

impl<'a> From<&'a mut Commands> for DrawCommands<'a> {
  fn from(cmd: &'a mut Commands) -> Self {
    DrawCommands { cmd }
  }
}

impl<'a> DrawCommands<'a> {
  pub fn bind_pipeline(&mut self, pipeline: &Pipeline) {
    unsafe {
      self.cmd.buffer.bind_graphics_pipeline(&pipeline.raw);
    }
  }

  pub fn push_constant<T>(&mut self, pipeline: &Pipeline, index: usize, value: &T) {
    let range = pipeline.push_constant_range(index);

    // Convert the constant to a slice of `u32` as vulkan/gfx-hal expects.
    let constants =
      unsafe { std::slice::from_raw_parts(value as *const T as *const u32, range.len()) };

    unsafe {
      self.cmd.buffer.push_graphics_constants(
        &pipeline.raw_layout,
        gfx_hal::pso::ShaderStageFlags::VERTEX | gfx_hal::pso::ShaderStageFlags::FRAGMENT,
        range.start,
        constants,
      );
    }
  }

  pub fn draw(&mut self, vertices: std::ops::Range<u32>) {
    unsafe {
      self.cmd.buffer.draw(vertices, 0..1);
    }
  }
}
