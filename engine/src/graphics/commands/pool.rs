// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::CommandLevel;
use crate::graphics::device::DeviceHandle;
use crate::graphics::prelude::*;
use crate::utils::Droppable;
use gfx_hal::pool::CommandPoolCreateFlags as CreateFlags;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

/// A pool of buffer space to store data for [`Commands`].
pub struct CommandPool {
  /// The raw backend command pool in a mutex for synchronized access.
  raw: Droppable<Mutex<backend::CommandPool>>,
  /// The device the pool was created with.
  device: DeviceHandle,
  /// An atomic flag indicating whether any [`Commands`] are recording.
  pub(super) recording: AtomicBool,
  /// ID of the device queue family this command pool was created for.
  queue_family_id: usize,
}

impl CommandPool {
  /// Creates a new command pool for the given queue family.
  pub fn new(device: &DeviceHandle, queue_family_id: usize) -> CommandPool {
    let pool = device
      .raw()
      .create_command_pool(
        hal::queue::QueueFamilyId(queue_family_id),
        CreateFlags::TRANSIENT,
      )
      .expect("Could not create command pool.");

    CommandPool {
      device: device.clone(),
      raw: Mutex::new(pool).into(),
      recording: AtomicBool::new(false),
      queue_family_id,
    }
  }

  /// Gets the ID of the device queue family this command pool was created for.
  pub fn queue_family_id(&self) -> usize {
    self.queue_family_id
  }

  /// Allocates a raw command buffer with the given level from the pool.
  pub(super) fn allocate_raw(&self, level: CommandLevel) -> backend::CommandBuffer {
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
        .device
        .raw()
        .destroy_command_pool(pool.into_inner().unwrap());
    }
  }
}
