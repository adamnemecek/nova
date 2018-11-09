mod buffers;
mod queues;
mod sync;

pub use self::buffers::*;
pub use self::queues::*;
pub use self::sync::*;

use super::hal::*;
use crate::utils::{quick_error, Droppable};
use gfx_memory::{MemoryAllocator, SmartAllocator};
use std::sync::{Arc, Mutex, MutexGuard};

pub type Allocator = SmartAllocator<Backend>;

pub struct Device {
  allocator: Droppable<Mutex<Allocator>>,
  raw: backend::Device,
  adapter: backend::Adapter,
  _instance: Arc<backend::Instance>,
}

impl Device {
  pub fn new<Q: QueueSet>(
    instance: &Arc<backend::Instance>,
    surface: &backend::Surface,
  ) -> Result<(Arc<Device>, Q), CreationError> {
    // Select the best available adapter.
    let adapter =
      select_best_adapter(&instance, surface).ok_or(CreationError::NoSupportedAdapter)?;

    // Determine queue families to open.
    let queue_families = Q::select_families(&adapter, &surface);

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
      _instance: instance.clone(),
      adapter,
      raw: gpu.device,
      allocator: Mutex::new(allocator).into(),
    });

    // Create a queue set from the queues.
    let queues = Q::from_raw(&device, &mut gpu.queues, queue_families);

    Ok((device, queues))
  }

  pub fn adapter(&self) -> &backend::Adapter {
    &self.adapter
  }

  pub fn allocator(&self) -> MutexGuard<SmartAllocator<Backend>> {
    self.allocator.lock().unwrap()
  }

  pub fn memory_properties(&self) -> gfx_hal::MemoryProperties {
    self.adapter.physical_device.memory_properties()
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
fn select_best_adapter(
  instance: &backend::Instance,
  surface: &backend::Surface,
) -> Option<backend::Adapter> {
  instance
    .enumerate_adapters()
    .into_iter()
    // Only select adapters with at least one graphics queue that supports
    // presentation to the surface.
    .filter(|adapter| {
      adapter
        .queue_families
        .iter()
        .any(|f| f.supports_graphics() && surface.supports_queue_family(f))
    })
    // Only select adapters with at least one non-graphics queue.
    .filter(|adapter| {
      adapter
        .queue_families
        .iter()
        .any(|f| !f.supports_graphics())
    })
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
