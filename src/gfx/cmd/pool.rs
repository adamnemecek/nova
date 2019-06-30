// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use gfx_hal::pool::RawCommandPool as _;

/// Pool of reusable command buffers.
///
/// This structure is cloneable and all clones refer to the same semaphore. When
/// all clones are dropped, the underlying backend resources are destroyed.
#[derive(Clone)]
pub struct Pool(Rc<RefCell<PoolInner>>);

struct PoolInner {
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
      )
    };

    match pool {
      Ok(pool) => Ok(Self(Rc::new(RefCell::new(PoolInner {
        context: context.clone(),
        pool: pool.into(),
        queue_id,
        level: gfx_hal::command::RawLevel::Primary,
        is_recording: false,
        recycle_bin: Vec::new(),
      })))),

      Err(_) => Err(OutOfMemoryError),
    }
  }

  /// Returns the queue ID the command pool was created for.
  pub fn queue_id(&self) -> QueueId {
    self.0.borrow().queue_id
  }

  /// Allocates a new backend command buffer.
  pub fn allocate(&self) -> backend::CommandBuffer {
    let mut inner = self.0.borrow_mut();
    let level = inner.level;

    inner.recycle_bin.pop().unwrap_or_else(|| inner.pool.allocate_one(level))
  }

  /// Registers a backend command buffer for reuse.
  pub fn recycle(&self, mut buffer: backend::CommandBuffer) {
    unsafe {
      buffer.reset(true);
    }

    self.0.borrow_mut().recycle_bin.push(buffer);
  }

  /// Sets whether a command buffer in the pool is recording or not.
  pub fn set_recording(&self, value: bool) {
    let mut inner = self.0.borrow_mut();

    assert!(!value || !inner.is_recording, "a command buffer in the pool is already recording");

    inner.is_recording = value;
  }
}

impl Drop for PoolInner {
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
