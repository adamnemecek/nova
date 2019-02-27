// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::buffer::Buffer;
use super::device::{self, Device, DeviceExt, QueueFamilyExt};
use super::pipeline::{MemoryBarrier, PipelineStage};
use super::texture::{Texture, TextureLayout};
use super::{Backend, Framebuffer, RenderPass};
use std::ops::{Deref, DerefMut};

pub use gfx_hal::command::RawCommandBuffer as CommandBufferExt;
pub use gfx_hal::command::RawLevel as CommandLevel;
pub use gfx_hal::pool::RawCommandPool as CommandPoolExt;

pub type CommandBuffer = <Backend as gfx_hal::Backend>::CommandBuffer;
pub type CommandPool = <Backend as gfx_hal::Backend>::CommandPool;

pub struct Commands {
  pub(crate) buffer: CommandBuffer,
  pool: CommandPool,
}

impl Commands {
  pub fn new(device: &Device, queue_family: &device::QueueFamily) -> Commands {
    let mut pool = unsafe {
      device
        .create_command_pool(
          queue_family.id(),
          gfx_hal::pool::CommandPoolCreateFlags::TRANSIENT
            | gfx_hal::pool::CommandPoolCreateFlags::RESET_INDIVIDUAL,
        )
        .expect("Could not create command pool")
    };

    let buffer = pool.allocate_one(CommandLevel::Primary);

    Commands { buffer, pool }
  }

  pub fn begin(&mut self) {
    unsafe {
      self.buffer.begin(Default::default(), Default::default());
    }
  }

  pub fn pipeline_barrier<'a>(
    &'a mut self,
    stages: (PipelineStage, PipelineStage),
    memory_barriers: impl IntoIterator<Item = MemoryBarrier<'a, Backend>>,
  ) {
    unsafe {
      self.buffer.pipeline_barrier(
        stages.0..stages.1,
        gfx_hal::memory::Dependencies::empty(),
        memory_barriers,
      );
    }
  }

  pub fn copy_buffer_to_texture(&mut self, buffer: &Buffer, offset: usize, texture: &Texture) {
    let texture_size = texture.size();

    unsafe {
      self.buffer.copy_buffer_to_image(
        &buffer.raw,
        &texture.raw,
        TextureLayout::TransferDstOptimal,
        &[gfx_hal::command::BufferImageCopy {
          buffer_offset: offset as u64,
          buffer_width: 0,
          buffer_height: 0,
          image_layers: gfx_hal::image::SubresourceLayers {
            aspects: gfx_hal::format::Aspects::COLOR,
            level: 0,
            layers: 0..1,
          },
          image_offset: gfx_hal::image::Offset { x: 0, y: 0, z: 0 },
          image_extent: gfx_hal::image::Extent {
            width: texture_size.width,
            height: texture_size.height,
            depth: 1,
          },
        }],
      );
    }
  }

  pub fn begin_render_pass(&mut self, render_pass: &RenderPass, framebuffer: &Framebuffer) {
    // Create a viewport struct covering the entire framebuffer.
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

    // Begin the render pass.
    unsafe {
      self.buffer.set_viewports(0, &[viewport.clone()]);
      self.buffer.set_scissors(0, &[viewport.rect]);

      self.buffer.begin_render_pass(
        render_pass,
        &framebuffer.raw,
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
      self.buffer.end_render_pass();
    }
  }

  pub fn finish(&mut self) {
    unsafe {
      self.buffer.finish();
    }
  }

  pub fn destroy(mut self, device: &Device) {
    unsafe {
      self.pool.free(Some(self.buffer));

      device.destroy_command_pool(self.pool);
    }
  }
}

impl Deref for Commands {
  type Target = CommandBuffer;

  fn deref(&self) -> &Self::Target {
    &self.buffer
  }
}

impl DerefMut for Commands {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.buffer
  }
}
