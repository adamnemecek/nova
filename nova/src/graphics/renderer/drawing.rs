// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Pipeline;
use crate::engine;
use crate::graphics::commands::{Commands, RawCommandsExt};

pub trait Drawable {
  fn draw(&mut self, cmd: DrawCommands, res: &engine::Resources);
}

pub struct DrawCommands<'a> {
  cmd: &'a mut Commands,
  pipeline: Option<Pipeline>,
}

impl<'a> From<&'a mut Commands> for DrawCommands<'a> {
  fn from(cmd: &'a mut Commands) -> Self {
    DrawCommands {
      cmd,
      pipeline: None,
    }
  }
}

impl<'a> DrawCommands<'a> {
  pub fn bind_pipeline(&mut self, pipeline: &Pipeline) {
    unsafe {
      self.cmd.raw.bind_graphics_pipeline(pipeline.raw());
    }

    self.pipeline = Some(pipeline.clone());
  }

  pub fn push_constant<T>(&mut self, index: usize, value: &T) {
    let pipeline = self
      .pipeline
      .as_ref()
      .expect("A pipeline must be bound to push constants.");

    let range = pipeline.push_constant_range(index);

    // Convert the constant to a slice of `u32` as vulkan/gfx-hal expects.
    let constants =
      unsafe { std::slice::from_raw_parts(value as *const T as *const u32, range.len()) };

    unsafe {
      self.cmd.raw.push_graphics_constants(
        pipeline.raw_layout(),
        gfx_hal::pso::ShaderStageFlags::VERTEX | gfx_hal::pso::ShaderStageFlags::FRAGMENT,
        range.start,
        constants,
      );
    }
  }

  pub fn draw(&mut self, vertices: std::ops::Range<u32>) {
    unsafe {
      self.cmd.raw.draw(vertices, 0..1);
    }
  }
}
