use super::Level;
use crate::graphics::backend;
use crate::graphics::device;
use crate::graphics::hal::prelude::*;
use crate::utils::Droppable;
use gfx_hal::pool::CommandPoolCreateFlags as CreateFlags;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

/// A pool of buffer space to store data for [`Commands`].
pub struct CommandPool {
  /// The device queue that commands will be submitted to.
  queue: Arc<device::Queue>,
  /// The raw backend command pool in a mutex for synchronized access.
  raw: Droppable<Mutex<backend::CommandPool>>,
  /// An atomic flag indicating whether any [`Commands`] are recording.
  pub(super) recording: AtomicBool,
}

impl CommandPool {
  /// Creates a new command pool for the given device queue.
  pub fn new(queue: &Arc<device::Queue>) -> Arc<CommandPool> {
    let pool = queue
      .device()
      .raw()
      .create_command_pool(queue.family_id(), CreateFlags::TRANSIENT)
      .expect("Could not create command pool.");

    Arc::new(CommandPool {
      queue: queue.clone(),
      raw: Mutex::new(pool).into(),
      recording: AtomicBool::new(false),
    })
  }

  /// Gets a reference to the device queue the pool was created for.
  pub fn queue(&self) -> &Arc<device::Queue> {
    &self.queue
  }

  /// Allocates a raw command buffer with the given level from the pool.
  pub(super) fn allocate_raw(&self, level: Level) -> backend::CommandBuffer {
    self
      .raw
      .lock()
      .unwrap()
      .allocate(1, level)
      .into_iter()
      .next()
      .unwrap()
  }

  /// Frees the given raw command buffer.Droppable
  ///
  /// Unsafe because there is no way to verify the command buffer came from this
  /// pool.
  pub(super) unsafe fn free_raw(&self, commands: backend::CommandBuffer) {
    self.raw.lock().unwrap().free(vec![commands]);
  }
}

// Implement drop to destroy the raw backend command pool.
impl Drop for CommandPool {
  fn drop(&mut self) {
    if let Some(pool) = self.raw.take() {
      self
        .queue
        .device()
        .raw()
        .destroy_command_pool(pool.into_inner().unwrap());
    }
  }
}
