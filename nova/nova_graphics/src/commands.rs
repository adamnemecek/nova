// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::gpu::queues::QueueId;
use crate::gpu::{Gpu, GpuDeviceExt};
use crate::Backend;
use gfx_hal::command::RawCommandBuffer as _;
use gfx_hal::command::RawLevel as CommandLevel;
use gfx_hal::pool::{CommandPoolCreateFlags, RawCommandPool as _};

type BackendCommandPool = <Backend as gfx_hal::Backend>::CommandPool;
type BackendCommandBuffer = <Backend as gfx_hal::Backend>::CommandBuffer;

macro_rules! debug_assert_recording {
  ($e:expr) => {
    debug_assert!(
      $e.state == State::Recording,
      "Command buffer is not recording."
    );
  };
}

pub struct CommandBuffer {
  buffer: BackendCommandBuffer,
  pool: BackendCommandPool,
  state: State,
}

impl CommandBuffer {
  pub fn new(gpu: &Gpu, queue_id: QueueId) -> Self {
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

  pub fn finish(&mut self) {
    debug_assert_recording!(self);

    unsafe { self.buffer.finish() };

    self.state = State::Recorded;
  }

  pub fn destroy(mut self, gpu: &Gpu) {
    unsafe {
      self.pool.free(Some(self.buffer));

      gpu.device.destroy_command_pool(self.pool);
    }
  }

  pub(crate) fn as_backend(&self) -> &BackendCommandBuffer {
    &self.buffer
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
  Initial,
  Recording,
  Recorded,
}
