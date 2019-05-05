// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::gpu::queues::QueueFamily;
use crate::gpu::Gpu;
use crate::images::{Image, ImageLayout};
use crate::renderer::{Framebuffer, MemoryBarrier, Pipeline, PipelineStage, RenderPass};
use crate::{Backend, Color};
use gfx_hal::command::RawCommandBuffer as _;
use gfx_hal::command::RawLevel as CommandLevel;
use gfx_hal::pool::{CommandPoolCreateFlags, RawCommandPool as _};
use gfx_hal::Device as _;
use std::iter;
use std::mem;
use std::ops::Range;
use std::slice;

type HalCommandPool = <Backend as gfx_hal::Backend>::CommandPool;
type HalCommandBuffer = <Backend as gfx_hal::Backend>::CommandBuffer;

macro_rules! debug_assert_recording {
  ($e:expr) => {
    debug_assert!(
      $e.state == State::Recording,
      "Command buffer is not recording."
    );
  };
}

pub struct CommandBuffer {
  buffer: HalCommandBuffer,
  pool: HalCommandPool,
  family: QueueFamily,
  state: State,
}

impl CommandBuffer {
  pub fn new(gpu: &Gpu, family: &QueueFamily) -> Self {
    let mut pool = unsafe {
      gpu
        .device
        .create_command_pool(family.id(), CommandPoolCreateFlags::RESET_INDIVIDUAL)
        .unwrap()
    };

    let buffer = pool.allocate_one(CommandLevel::Primary);

    Self {
      pool,
      buffer,
      state: State::Initial,
      family: family.clone(),
    }
  }

  pub fn queue_family(&self) -> &QueueFamily {
    &self.family
  }

  pub fn begin(&mut self) {
    debug_assert!(
      self.state != State::Recording,
      "Command buffer is already recording."
    );

    unsafe {
      self.buffer.begin(Default::default(), Default::default());
    }

    self.state = State::Recording;
  }

  pub fn pipeline_barrier<'a>(
    &'a mut self,
    stages: Range<PipelineStage>,
    memory_barriers: impl IntoIterator<Item = MemoryBarrier<'a, Backend>>,
  ) {
    debug_assert_recording!(self);

    unsafe {
      self.buffer.pipeline_barrier(
        stages,
        gfx_hal::memory::Dependencies::empty(),
        memory_barriers,
      );
    }
  }

  pub(crate) fn clear_image(&mut self, image: &Image, color: Color) {
    debug_assert_recording!(self);

    unsafe {
      self.buffer.clear_image(
        image.as_hal(),
        ImageLayout::TransferDstOptimal,
        gfx_hal::command::ClearColorRaw {
          float32: [color.r, color.g, color.b, color.a],
        },
        gfx_hal::command::ClearDepthStencilRaw {
          depth: 0.0,
          stencil: 0,
        },
        iter::once(gfx_hal::image::SubresourceRange {
          aspects: gfx_hal::format::Aspects::COLOR,
          levels: 0..1,
          layers: 0..1,
        }),
      );
    }
  }

  pub fn begin_render_pass(&mut self, render_pass: &RenderPass, framebuffer: &Framebuffer) {
    debug_assert_recording!(self);

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
        render_pass.as_hal(),
        framebuffer.as_hal(),
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

  pub fn bind_pipeline(&mut self, pipeline: &Pipeline) {
    debug_assert_recording!(self);

    unsafe { self.buffer.bind_graphics_pipeline(pipeline.as_hal()) };
  }

  pub fn push_constants<T>(&mut self, pipeline: &Pipeline, constants: T) {
    debug_assert_recording!(self);

    let count = pipeline.push_constant_count();

    debug_assert_eq!(count * 4, mem::size_of::<T>(), "The push constants type must be the same size as the pipeline's size_of_push_constants option.");

    unsafe {
      self.buffer.push_graphics_constants(
        pipeline.hal_layout(),
        gfx_hal::pso::ShaderStageFlags::ALL,
        0,
        slice::from_raw_parts(&constants as *const T as *const u32, count),
      );
    }
  }

  pub fn draw(&mut self, indices: Range<u32>) {
    debug_assert_recording!(self);

    unsafe { self.buffer.draw(indices, 0..1) };
  }

  pub fn finish_render_pass(&mut self) {
    debug_assert_recording!(self);

    unsafe { self.buffer.end_render_pass() };
  }

  pub fn finish(&mut self) {
    debug_assert_recording!(self);

    unsafe { self.buffer.finish() };

    self.state = State::Recorded;
  }

  pub fn destroy(mut self, gpu: &Gpu) {
    unsafe {
      self.pool.free(iter::once(self.buffer));

      gpu.device.destroy_command_pool(self.pool);
    }
  }

  pub fn as_hal(&self) -> &HalCommandBuffer {
    &self.buffer
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
  Initial,
  Recording,
  Recorded,
}
