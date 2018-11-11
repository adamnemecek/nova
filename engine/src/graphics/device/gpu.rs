use super::{Device, Queue};
use crate::graphics::backend;
use crate::graphics::hal::prelude::*;
use crate::utils::quick_error;
use std::sync::Arc;

pub struct Gpu {
  pub device: Arc<Device>,
  pub queues: GpuQueues,
}

pub struct GpuQueues {
  pub graphics: Arc<Queue>,
  pub transfer: Arc<Queue>,
}

impl Gpu {
  pub fn new(backend: &Arc<backend::Instance>) -> Result<Gpu, CreationError> {
    // Select the best available adapter.
    let adapter = select_best_adapter(&backend).ok_or(CreationError::NoSupportedAdapter)?;

    // Determine queue families to open.
    let (graphics_family, transfer_family) = select_queue_families(&adapter);

    // Open the physical device with the selected queues.
    let mut raw = adapter
      .physical_device
      .open(&[(&graphics_family, &[1.0]), (&transfer_family, &[1.0])])?;

    // Create a device from the raw device.
    let device = Arc::new(unsafe { Device::from_raw(raw.device, adapter, backend) });

    // Create a set of queues from the raw queues.
    let queues = GpuQueues {
      graphics: Arc::new(Queue::new(&device, &mut raw.queues, graphics_family)),
      transfer: Arc::new(Queue::new(&device, &mut raw.queues, transfer_family)),
    };

    Ok(Gpu {
      device: device,
      queues,
    })
  }
}

/// Selects the best available device adapter.
pub fn select_best_adapter(instance: &backend::Instance) -> Option<backend::Adapter> {
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

fn select_queue_families(
  adapter: &backend::Adapter,
) -> (backend::QueueFamily, backend::QueueFamily) {
  let graphics = adapter
    .queue_families
    .iter()
    .filter(|family| family.supports_graphics())
    .next()
    .expect("no graphics queue family")
    .clone();

  let transfer = adapter
    .queue_families
    .iter()
    .filter(|family| !family.supports_graphics())
    .next()
    .expect("no transfer queue family")
    .clone();

  (graphics, transfer)
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
