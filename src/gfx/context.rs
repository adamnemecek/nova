// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::error::DeviceCreationError;

use super::*;

pub struct Context {
  memory: Memory,
  queues: cmd::Queues,
  device: backend::Device,
  adapter: backend::Adapter,
  backend: Arc<backend::Instance>,
}

impl Context {
  pub fn new(logger: log::Logger) -> Result<Self, InitError> {
    // Instantiate the backend.
    let backend = backend::Instance::create("nova", 1);

    log::debug!(logger, "instantiated graphics backend";
      "name" => backend::NAME,
    );

    // Get and log all available adapters.
    let mut adapters = backend.enumerate_adapters();

    for adapter in &adapters {
      log::debug!(logger, "found graphics adapter";
        "type" => log::Debug(&adapter.info.device_type),
        "type_id" => adapter.info.device,
        "vendor_id" => adapter.info.vendor,
        "name" => &adapter.info.name,
      );
    }

    // Sort adapters from most powerful type to least.
    adapters.sort_by_key(|adapter| match adapter.info.device_type {
      gfx_hal::adapter::DeviceType::DiscreteGpu => 3,
      gfx_hal::adapter::DeviceType::IntegratedGpu => 2,
      gfx_hal::adapter::DeviceType::Cpu => 1,
      _ => 0,
    });

    // Select the first adapter in the sorted list.
    let adapter = adapters.into_iter().next().ok_or(InitError::NoAdapter)?;

    log::debug!(logger, "selected graphics adapter";
      "type" => log::Debug(&adapter.info.device_type),
      "type_id" => adapter.info.device,
      "vendor_id" => adapter.info.vendor,
      "name" => &adapter.info.name,
    );

    // Get all queue families.
    let queue_families = adapter.queue_families.clone();

    // Open the physical device and one queue in every family.
    let queue_requests = queue_families
      .iter()
      .map(|family| {
        use gfx_hal::QueueFamily as _;

        log::debug!(logger, "found graphics queue family";
          "max_queues" => family.max_queues(),
          "type" => log::Debug(family.queue_type()),
          "id" => family.id().0,
        );

        (family, &[1.0][..])
      })
      .collect::<Vec<_>>();

    let gfx_hal::Gpu { device, queues } = unsafe {
      use gfx_hal::PhysicalDevice as _;

      adapter.physical_device.open(&queue_requests[..], gfx_hal::Features::empty())?
    };

    log::debug!(logger, "opened graphics device");

    let queues = cmd::Queues::new(queue_families, queues);
    let memory = Memory::new(&adapter);

    Ok(Context { memory, queues, device, adapter, backend: backend.into() })
  }

  pub(super) fn backend(&self) -> &backend::Instance {
    &self.backend
  }

  pub(super) fn physical_device(&self) -> &backend::PhysicalDevice {
    &self.adapter.physical_device
  }

  pub(super) fn device(&self) -> &backend::Device {
    &self.device
  }

  pub(super) fn memory(&self) -> &Memory {
    &self.memory
  }

  pub(super) fn queues(&self) -> &cmd::Queues {
    &self.queues
  }

  /// Waits for the graphics device to be idle, meaning no command buffers are
  /// being executed.
  pub(super) fn wait_idle(&self) {
    let _ = self.device.wait_idle();
  }
}

/// An error that occurred during the initialization of a new graphics context.
#[derive(Debug)]
pub enum InitError {
  /// There is no suitable graphics adapter.
  NoAdapter,
  /// An error occurred while creating the graphics device.
  DeviceCreationFailed(DeviceCreationError),
}

impl std::error::Error for InitError {}

impl fmt::Display for InitError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      InitError::NoAdapter => write!(f, "no suitable graphics adapter"),
      InitError::DeviceCreationFailed(cause) => write!(f, "failed to create device: {}", cause),
    }
  }
}

// Impl `From` to convert from device creation errors.
impl From<DeviceCreationError> for InitError {
  fn from(cause: DeviceCreationError) -> Self {
    InitError::DeviceCreationFailed(cause)
  }
}
