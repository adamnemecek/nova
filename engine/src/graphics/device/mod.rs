// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod gpu;
pub mod queue;

pub use self::gpu::Gpu;
pub use self::queue::Queue;

use super::backend::{self, Backend};
use super::hal::prelude::*;
use crate::utils::Droppable;
use gfx_memory::{MemoryAllocator, SmartAllocator};
use std::sync::{Arc, Mutex, MutexGuard};

/// Type of memory allocator used by the device.
pub type Allocator = SmartAllocator<Backend>;

/// A graphics device. Used to create most other graphics resources.
pub struct Device {
  /// Raw backend device.
  raw: backend::Device,
  /// Raw backend adapter information for this device.
  adapter: hal::Adapter,
  /// Memory allocator for allocating device memory.
  allocator: Droppable<Mutex<Allocator>>,
  /// Raw backend instance this device was created from.
  backend: Arc<backend::Instance>,
}

impl Device {
  /// Creates a new device from raw backend structures.
  ///
  /// Unsafe because this function does not verify that the given device,
  /// adapter, and backend instance are related.
  pub unsafe fn from_raw(
    raw: backend::Device,
    adapter: hal::Adapter,
    backend: &Arc<backend::Instance>,
  ) -> Device {
    // Create a new gfx_memory smart allocator.
    let allocator = Mutex::new(Allocator::new(
      adapter.physical_device.memory_properties(),
      // short-lived arena storage.
      64 * 1024 * 1024, // 64 MB per allocation.
      // long-lived chunked storage.
      32,                // 32 blocks allocated per chunk.
      128,               // 128 bytes minimum block size.
      256 * 1024 * 1024, // 256 MB maximum chunk size.
    ));

    Device {
      adapter,
      raw,
      allocator: allocator.into(),
      backend: backend.clone(),
    }
  }

  /// Gets a reference to the backend instance this device was created with.
  pub fn backend(&self) -> &Arc<backend::Instance> {
    &self.backend
  }

  /// Gets a reference to the HAL adapter information.
  pub fn adapter(&self) -> &hal::Adapter {
    &self.adapter
  }

  /// Locks and gets a reference to the device's memory allocator.
  pub fn allocator(&self) -> MutexGuard<Allocator> {
    self.allocator.lock().unwrap()
  }

  /// Gets a reference to the raw backend device.
  pub fn raw(&self) -> &backend::Device {
    &self.raw
  }
}

// Implement `Drop` to dispose the memory allocator and free allocated memory.
impl Drop for Device {
  fn drop(&mut self) {
    if let Some(allocator) = self.allocator.take() {
      allocator
        .into_inner()
        .unwrap()
        .dispose(&self.raw)
        .expect("could not dispose allocator");
    }
  }
}
