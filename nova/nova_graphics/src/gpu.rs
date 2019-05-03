// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod queues;

mod commands;

pub(crate) use self::commands::CommandBuffer;
pub(crate) use gfx_hal::Device as GpuDeviceExt;

use self::queues::GpuQueues;
use crate::backend::{self, Backend};
use gfx_hal::adapter::DeviceType;
use gfx_hal::error::DeviceCreationError;
use gfx_hal::{Instance as _, PhysicalDevice as _, QueueFamily as _};
use nova_core::log::Logger;
use nova_core::quick_error;
use nova_core::resources::{self, ReadResource, Resources, WriteResource};
use std::fmt;

pub type ReadGpu<'a> = ReadResource<'a, Gpu>;
pub type WriteGpu<'a> = WriteResource<'a, Gpu>;

type Adapter = gfx_hal::Adapter<Backend>;
type Device = <Backend as gfx_hal::Backend>::Device;

pub struct Gpu {
  // Field order matters.
  pub(crate) device: Device,
  pub(crate) adapter: Adapter,
  pub(crate) backend: backend::Instance,
}

impl Gpu {
  pub fn wait_idle(&self) {
    self
      .device
      .wait_idle()
      .expect("Could not wait for GPU device to be idle");
  }
}

pub fn set_up(res: &mut Resources) -> Result<(), GpuSetupError> {
  if res.has_value::<Gpu>() {
    return Ok(());
  }

  let log = Logger::new(module_path!());

  // Instantiate the backend.
  let backend = backend::Instance::create("nova", 1);

  log
    .info("Instantiated backend.")
    .with("backend", backend::NAME);

  // Get and log all available adapters.
  let mut adapters = backend.enumerate_adapters();

  for adapter in &adapters {
    log
      .debug("Detected adapter:")
      .with("name", &adapter.info.name)
      .with("vendor_id", adapter.info.vendor)
      .with("type_id", adapter.info.device)
      .with("type", &adapter.info.device_type);
  }

  // Sort adapters from most powerful type to least.
  adapters.sort_by_key(|adapter| match adapter.info.device_type {
    DeviceType::DiscreteGpu => 3,
    DeviceType::IntegratedGpu => 2,
    DeviceType::Cpu => 1,
    _ => 0,
  });

  // Select the first adapter in the sorted list.
  let adapter = adapters.into_iter().next().ok_or(GpuSetupError::NoDevice)?;

  log
    .info("Selected adapter:")
    .with("name", &adapter.info.name)
    .with("vendor_id", adapter.info.vendor)
    .with("type_id", adapter.info.device)
    .with("type", &adapter.info.device_type);

  // Get all queue families.
  let queue_families = adapter.queue_families.clone();

  // Open the physical device and one queue in every family.
  let queue_requests = queue_families
    .iter()
    .map(|family| {
      log
        .debug("Detected queue family:")
        .with("id", family.id().0)
        .with("type", family.queue_type())
        .with("max_queues", family.max_queues());

      (family, &[1.0][..])
    })
    .collect::<Vec<_>>();

  let gfx_hal::Gpu { device, queues } =
    unsafe { adapter.physical_device.open(&queue_requests[..])? };

  log.info("Opened device.");

  // Create and insert resources.
  let gpu = Gpu {
    backend,
    adapter,
    device,
  };

  let queues = GpuQueues::new(queue_families, queues);

  res.insert(gpu);
  res.insert(queues);

  Ok(())
}

impl fmt::Debug for Gpu {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("Gpu").finish()
  }
}

pub fn borrow(res: &Resources) -> ReadGpu {
  resources::borrow(res)
}

pub fn borrow_mut(res: &Resources) -> WriteGpu {
  resources::borrow_mut(res)
}

quick_error! {
  #[derive(Debug)]
  pub enum GpuSetupError {
    NoDevice {
      display("there is no graphics device available")
    }

    CouldNotCreateDevice(err: DeviceCreationError) {
      from()
      display("could not create device - {}", err)
    }
  }
}
