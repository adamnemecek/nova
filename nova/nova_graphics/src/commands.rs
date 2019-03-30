// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::gpu::queues::GpuQueueId;
use crate::gpu::{Gpu, GpuDeviceExt};
use crate::images::{Image, ImageLayout};
use crate::pipelines::{MemoryBarrier, PipelineStage};
use crate::{Backend, Color4};
use gfx_hal::command::RawCommandBuffer as _;
use gfx_hal::command::RawLevel as CommandLevel;
use gfx_hal::pool::{CommandPoolCreateFlags, RawCommandPool as _};
use std::iter;
use std::ops::Range;

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
  state: State,
}

impl CommandBuffer {
  pub fn new(gpu: &Gpu, queue_id: GpuQueueId) -> Self {
    let mut pool = unsafe {
      gpu
        .device
        .create_command_pool(queue_id.family_id, CommandPoolCreateFlags::RESET_INDIVIDUAL)
        .unwrap()
    };

    let buffer = pool.allocate_one(CommandLevel::Primary);

    Self {
      pool,
      buffer,
      state: State::Initial,
    }
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

  pub(crate) fn clear_image(&mut self, image: &Image, color: Color4) {
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
