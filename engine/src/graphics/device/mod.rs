pub mod queues;

pub mod gpu;

pub use self::gpu::Gpu;
pub use self::queues::{DefaultQueueSet, Queue, QueueSet};

use super::backend::{self, Backend};
use super::hal::prelude::*;
use crate::utils::{quick_error, Droppable};
use gfx_memory::{MemoryAllocator, SmartAllocator};
use std::sync::{Arc, Mutex, MutexGuard};

/// Type of memory allocator used by the device.
pub type Allocator = SmartAllocator<Backend>;

/// A graphics device. Used to create most other graphics resources.
pub struct Device {
  /// Raw backend device.
  raw: backend::Device,
  /// Raw backend adapter information for this device.
  adapter: backend::Adapter,
  /// Memory allocator for allocating device memory.
  allocator: Droppable<Mutex<Allocator>>,
  /// Raw backend instance this device was created from.
  backend: Arc<backend::Instance>,
}

impl Device {
  pub unsafe fn from_raw(
    raw: backend::Device,
    adapter: backend::Adapter,
    backend: &Arc<backend::Instance>,
  ) -> Device {
    let allocator = Mutex::new(Allocator::new(
      adapter.physical_device.memory_properties(),
      64 * 1024 * 1024,
      32,
      128,
      256 * 1024 * 1024,
    ));

    Device {
      adapter,
      raw,
      allocator: allocator.into(),
      backend: backend.clone(),
    }
  }

  pub fn backend(&self) -> &Arc<backend::Instance> {
    &self.backend
  }

  pub fn adapter(&self) -> &backend::Adapter {
    &self.adapter
  }

  pub fn allocator(&self) -> MutexGuard<SmartAllocator<Backend>> {
    self.allocator.lock().unwrap()
  }

  pub fn raw(&self) -> &backend::Device {
    &self.raw
  }
}

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
