// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::commands::{CommandBufferExt, Commands};
use super::pipeline::Pipeline;
use super::texture::TextureCache;
use super::{Allocator, Device};
use nova_graphics as graphics;
use std::iter;

pub struct Render<'a> {
  pub(crate) cmd: &'a mut Commands,
  pub(crate) device: &'a Device,
  pub(crate) allocator: &'a mut Allocator,
  pub(crate) texture_cache: &'a mut TextureCache,
}

impl<'a> Render<'a> {
  pub fn bind_pipeline(&mut self, pipeline: &Pipeline) {
    unsafe {
      self.cmd.buffer.bind_graphics_pipeline(&pipeline.raw);
    }
  }

  pub fn bind_texture_or_default(
    &mut self,
    pipeline: &Pipeline,
    binding: usize,
    texture: Option<&graphics::Image>,
  ) {
    match texture {
      Some(texture) => self.bind_texture(pipeline, binding, texture),
      None => self.bind_default_texture(pipeline, binding),
    }
  }

  pub fn bind_default_texture(&mut self, pipeline: &Pipeline, binding: usize) {
    let descriptor_set = self.texture_cache.get_default();

    unsafe {
      self.cmd.buffer.bind_graphics_descriptor_sets(
        &pipeline.raw_layout,
        binding,
        iter::once(descriptor_set),
        &[],
      );
    }
  }

  pub fn bind_texture(&mut self, pipeline: &Pipeline, binding: usize, texture: &graphics::Image) {
    let descriptor_set =
      self.texture_cache
        .get_cached(texture, &self.device, &mut self.allocator);

    unsafe {
      self.cmd.buffer.bind_graphics_descriptor_sets(
        &pipeline.raw_layout,
        binding,
        iter::once(descriptor_set),
        &[],
      );
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
