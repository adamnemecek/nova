// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Device, Queue};
use crate::graphics::backend;
use crate::graphics::hal::prelude::*;
use crate::graphics::window::Surface;
use crate::utils::quick_error;
use std::sync::Arc;

/// Initialization helper structure for creating a device and a set of queues
/// for submitting commands.
pub struct Gpu {
  pub device: Arc<Device>,
  pub queues: GpuQueues,
}

/// Set of queues created by [`Gpu::new`].
pub struct GpuQueues {
  /// Graphics queue for submitting rendering commands and presenting to a
  /// window surface.
  pub graphics: Queue,
  /// Transfer queue for copying to and from buffers and images.
  pub transfer: Option<Queue>,
}

impl Gpu {
  /// Initializes a new graphics device with a set of queues for submitting
  /// commands and presenting to a window surface.
  pub fn new(
    backend: &Arc<backend::Instance>,
    surface: Option<&Surface>,
  ) -> Result<Gpu, CreationError> {
    let surface = surface.map(AsRef::as_ref);

    // Select the best available adapter.
    let adapter =
      select_best_adapter(&backend, surface).ok_or(CreationError::NoSupportedAdapter)?;

    // Determine queue families to open.
    let (graphics_family, transfer_family) = select_queue_families(&adapter, surface);

    let mut queue_req = vec![(&graphics_family, &[1.0f32][..])];

    if let Some(ref transfer_family) = transfer_family {
      queue_req.push((transfer_family, &[1.0]))
    }

    // Open the physical device with the selected queues.
    let mut raw = adapter.physical_device.open(&queue_req)?;

    // Create a device from the raw device.
    let device = Arc::new(unsafe { Device::from_raw(raw.device, adapter, backend) });

    // Create a set of queues from the raw queues.
    let queues = GpuQueues {
      graphics: unsafe { Queue::from_raw(&device, &mut raw.queues, graphics_family) },
      transfer: match transfer_family {
        Some(f) => Some(unsafe { Queue::from_raw(&device, &mut raw.queues, f) }),
        None => None,
      },
    };

    Ok(Gpu {
      device: device,
      queues,
    })
  }
}

/// Selects the best available device adapter.
pub fn select_best_adapter(
  instance: &backend::Instance,
  surface: Option<&backend::Surface>,
) -> Option<hal::Adapter> {
  instance
    .enumerate_adapters()
    .into_iter()
    // Select only adapters with a graphics queue family that supports the given
    // surface if provided.
    .filter(|adapter| {
      adapter.queue_families.iter().any(|f| {
        f.supports_graphics() || surface.map(|s| s.supports_queue_family(f)).unwrap_or(true)
      })
    })
    // Select the adapter with the higest score:
    .max_by_key(|adapter| {
      let mut score = 0;

      // Prefer discrete graphics devices over integrated ones.
      if adapter.info.device_type == hal::adapter::DeviceType::DiscreteGpu {
        score -= 1000;
      }

      score
    })
}

/// Selects a queue family for graphics queues and a queue family for transfer
/// queues from the given backend adapter info.
fn select_queue_families(
  adapter: &hal::Adapter,
  surface: Option<&backend::Surface>,
) -> (backend::QueueFamily, Option<backend::QueueFamily>) {
  let graphics = adapter
    .queue_families
    .iter()
    .filter(|family| family.supports_graphics())
    .filter(|family| {
      surface
        .map(|s| s.supports_queue_family(family))
        .unwrap_or(true)
    })
    .next()
    .expect("no graphics queue family")
    .clone();

  let transfer = adapter
    .queue_families
    .iter()
    .filter(|family| !family.supports_graphics())
    .next()
    .map(Clone::clone);

  (graphics, transfer)
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
