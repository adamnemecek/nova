pub mod queues;

mod buffers;
mod sync;

pub use self::buffers::*;
pub use self::queues::{DefaultQueueSet, Queue, QueueSet};
pub use self::sync::*;

use super::backend::{self, Backend};
use super::hal::prelude::*;
use crate::utils::{quick_error, Droppable};
use gfx_memory::{MemoryAllocator, SmartAllocator};
use std::sync::{Arc, Mutex, MutexGuard};

pub type Allocator = SmartAllocator<Backend>;

pub struct Device {
  raw: backend::Device,
  adapter: backend::Adapter,
  allocator: Droppable<Mutex<Allocator>>,
  backend: Arc<backend::Instance>,
}

impl Device {
  pub fn open<Q: QueueSet>(
    backend: &Arc<backend::Instance>,
  ) -> Result<(Arc<Device>, Q), CreationError> {
    // Select the best available adapter.
    let adapter = select_best_adapter(&backend).ok_or(CreationError::NoSupportedAdapter)?;

    // Determine queue families to open.
    let queue_families = Q::select_families(&adapter);

    // Open the adapter's physical device.
    let priorities = [1.0f32];

    let creation_info = queue_families
      .iter()
      .map(|f| (f, &priorities[..]))
      .collect::<Vec<_>>();

    let mut gpu = adapter.physical_device.open(&creation_info[..])?;

    let allocator = Allocator::new(
      adapter.physical_device.memory_properties(),
      64 * 1024 * 1024,
      32,
      128,
      256 * 1024 * 1024,
    );

    // Create a wrapper.
    let device = Arc::new(Device {
      adapter,
      raw: gpu.device,
      allocator: Mutex::new(allocator).into(),
      backend: backend.clone(),
    });

    // Create a queue set from the queues.
    let queues = Q::from_raw(&device, &mut gpu.queues, queue_families);

    Ok((device, queues))
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

/// Selects the best available device adapter.
fn select_best_adapter(instance: &backend::Instance) -> Option<backend::Adapter> {
  instance
    .enumerate_adapters()
    .into_iter()
    // Select only adapters with a graphics queue family.
    .filter(|adapter| adapter.queue_families.iter().any(|f| f.supports_graphics()))
    // Select the adapter with the higest score:
    .max_by_key(|adapter| {
      let mut score = 0;

      // Prefer discrete graphics devices over integrated ones.
      if adapter.info.device_type == gfx_hal::adapter::DeviceType::DiscreteGpu {
        score += 1000;
      }

      score
    })
}

quick_error! {
  #[derive(Debug)]
  pub enum CreationError {
    NoSupportedAdapter {
      display("No supported graphics adapters available.")
    }
    OpenAdapterFailed(err: gfx_hal::error::DeviceCreationError) {
      display("Could not open adapter: {}", err)
      from()
    }
  }
}
