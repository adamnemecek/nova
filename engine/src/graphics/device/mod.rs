// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod queue;
mod submission;

pub use self::queue::*;
pub use self::submission::*;

use crate::graphics::prelude::*;
use crate::utils::{quick_error, Droppable};
use derive_more::*;
use gfx_memory::{MemoryAllocator, SmartAllocator};
use std::sync::{Arc, Mutex, MutexGuard};

/// Type of memory allocator used by the device.
pub type Allocator = SmartAllocator<Backend>;

/// A graphics device. Used to create most other graphics resources.
pub struct Device {
  raw: backend::Device,
  adapter: hal::Adapter,
  instance: backend::Instance,
  queues: Vec<Mutex<Queue>>,
  allocator: Droppable<Mutex<Allocator>>,
}

impl Device {
  pub fn create() -> Result<DeviceHandle, CreationError> {
    let instance = backend::Instance::create("nova", 1);

    // Select the best available adapter.
    let adapter = instance
      .enumerate_adapters()
      .into_iter()
      .max_by_key(score_adapter)
      .ok_or(CreationError::NoSupportedAdapter)?;

    // Open the physical device with a queue from each family.
    let queue_reqs = adapter
      .queue_families
      .iter()
      .map(|f| (f, &[1.0f32][..]))
      .collect::<Vec<_>>();

    let mut raw = adapter.physical_device.open(&queue_reqs)?;

    // Wrap each opened queue.
    let queues = adapter
      .queue_families
      .iter()
      .map(|family| Queue::take_raw(&mut raw.queues, family.id()))
      .map(Mutex::new)
      .collect();

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

    Ok(DeviceHandle(Arc::new(Device {
      raw: raw.device,
      adapter,
      instance,
      queues,
      allocator: allocator.into(),
    })))
  }

  /// Gets the name of the device.
  pub fn name(&self) -> &str {
    &self.adapter.info.name
  }

  /// Locks and gets a reference to the device's memory allocator.
  pub fn allocator(&self) -> MutexGuard<Allocator> {
    self.allocator.lock().unwrap()
  }

  /// Gets a reference to the raw backend device.
  pub fn raw(&self) -> &backend::Device {
    &self.raw
  }

  /// Gets a reference to the HAL adapter info for this device.
  pub fn adapter(&self) -> &hal::Adapter {
    &self.adapter
  }

  /// Gets a reference to the backend instance this device was created with.
  pub fn backend(&self) -> &backend::Instance {
    &self.instance
  }

  /// Waits for all command submissions to complete on all queues and for the
  /// device to become completely idle.
  ///
  /// This function should be avoided in favor of other synchronizaton
  /// primitives or methods, but is useful prior to exiting the application or
  /// dropping the device.
  pub fn wait_idle(&self) {
    self
      .raw
      .wait_idle()
      .expect("Could not wait for the device to be idle");
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
        .unwrap_or_else(|_| panic!("Device dropped before all resources have been freed."));
    }

    self.queues.clear();
  }
}

#[derive(Clone, Deref)]
pub struct DeviceHandle(Arc<Device>);

/// Scores an adapter. The highest scoring adapter is used.
fn score_adapter(adapter: &hal::Adapter) -> usize {
  let mut score = 0;

  // Prefer discrete graphics devices over integrated ones.
  if adapter.info.device_type == hal::adapter::DeviceType::DiscreteGpu {
    score += 1000;
  }

  score
}

quick_error! {
  #[derive(Debug)]
  pub enum CreationError {
    NoSupportedAdapter {
      display("No supported graphics adapters available.")
    }
    OpenAdapterFailed(err: hal::error::DeviceCreationError) {
      display("Could not open adapter: {}", err)
      from()
    }
  }
}
