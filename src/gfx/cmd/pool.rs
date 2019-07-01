// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use gfx_hal::pool::RawCommandPool as _;

/// Pool of reusable command buffers.
pub struct Pool {
  context: Arc<Context>,
  pool: Expect<backend::CommandPool>,
  queue_id: QueueId,
  level: gfx_hal::command::RawLevel,
  is_recording: bool,
  recycle_bin: Vec<backend::CommandBuffer>,
}

impl Pool {
  /// Creates a new command pool for the given queue ID.
  pub fn new(context: &Arc<Context>, queue_id: QueueId) -> Result<Self, OutOfMemoryError> {
    let pool = unsafe {
      context.device().create_command_pool(
        queue_id.as_backend(),
        gfx_hal::pool::CommandPoolCreateFlags::RESET_INDIVIDUAL,
      )?
    };

    Ok(Self {
      context: context.clone(),
      pool: pool.into(),
      queue_id,
      level: gfx_hal::command::RawLevel::Primary,
      is_recording: false,
      recycle_bin: Vec::new(),
    })
  }

  /// Returns the queue ID the command pool was created for.
  pub fn queue_id(&self) -> QueueId {
    self.queue_id
  }

  /// Allocates a new backend command buffer.
  pub fn allocate(&mut self) -> backend::CommandBuffer {
    self.recycle_bin.pop().unwrap_or_else(|| self.pool.allocate_one(self.level))
  }

  /// Registers a backend command buffer for reuse.
  pub fn recycle(&mut self, mut buffer: backend::CommandBuffer) {
    unsafe {
      buffer.reset(true);
    }

    self.recycle_bin.push(buffer);
  }

  /// Sets whether a command buffer in the pool is recording or not.
  pub fn set_recording(&mut self, value: bool) {
    assert!(
      !value || !self.is_recording,
      "a command list in the same command pool is already recording"
    );

    self.is_recording = value;
  }

  /// Returns the pool wrapped in an `Rc<RefCell<T>>`.
  pub fn into_ref_cell(self) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(self))
  }
}

impl Drop for Pool {
  fn drop(&mut self) {
    let mut pool = self.pool.take();

    // Free all allocated command buffers before destroying the pool.
    while let Some(buffer) = self.recycle_bin.pop() {
      unsafe {
        pool.free(iter::once(buffer));
      }
    }

    unsafe {
      self.context.device().destroy_command_pool(pool);
    }
  }
}
