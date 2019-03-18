// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::commands::{CommandBufferExt, Commands};
use crate::pipeline::Pipeline;
use crate::textures::{TextureId, Textures};
use std::iter;

pub struct Render<'a> {
  pub(crate) cmd: &'a mut Commands,
  pub(crate) textures: &'a mut Textures,
}

impl<'a> Render<'a> {
  pub fn bind_pipeline(&mut self, pipeline: &Pipeline) {
    unsafe {
      self.cmd.buffer.bind_graphics_pipeline(&pipeline.raw);
    }
  }

  pub fn textures(&self) -> &Textures {
    self.textures
  }

  pub fn textures_mut(&mut self) -> &mut Textures {
    self.textures
  }

  pub fn bind_texture(&mut self, pipeline: &Pipeline, binding: usize, id: TextureId) {
    let texture = match self.textures.get_texture(id) {
      Some(t) => t,
      None => self.textures.transparent(),
    };

    unsafe {
      self.cmd.buffer.bind_graphics_descriptor_sets(
        &pipeline.raw_layout,
        binding,
        iter::once(&texture.descriptor_set),
        &[],
      );
    }
  }

  pub fn push_constants<T>(&mut self, pipeline: &Pipeline, constants: &T) {
    let size = std::mem::size_of::<T>();

    debug_assert!(
      size == pipeline.push_constants * 4,
      "Push constants must be the same size as the type defined by the pipeline."
    );

    // Convert the constant to a slice of `u32` as vulkan/gfx-hal expects.
    let constants = unsafe {
      std::slice::from_raw_parts(constants as *const T as *const u32, pipeline.push_constants)
    };

    unsafe {
      self.cmd.buffer.push_graphics_constants(
        &pipeline.raw_layout,
        gfx_hal::pso::ShaderStageFlags::VERTEX | gfx_hal::pso::ShaderStageFlags::FRAGMENT,
        0,
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
