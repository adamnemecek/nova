// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//pub mod render;

mod backend;
mod buffer;
mod cmd;
mod color;
mod context;
mod descriptors;
mod framebuffer;
mod image;
mod image_data;
mod loader;
mod memory;
mod pipeline;
mod render_pass;
mod sampler;
mod shader;
mod vertex;

pub use self::context::*;

use self::{
  buffer::*, color::*, descriptors::*, framebuffer::*, image::*, memory::*, render_pass::*,
  sampler::*,
};

use super::*;
use gfx_hal::{Device as _, Instance as _, PhysicalDevice as _};

pub fn init(logger: &log::Logger) -> Result<Context, InitError> {
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

  Ok(Context::new(backend, adapter, device, queues))
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

/// An error which indicates that there is not enough of either host or device
/// memory remaining to complete an operation.
#[derive(Debug)]
pub struct OutOfMemoryError;

impl std::error::Error for OutOfMemoryError {}

impl fmt::Display for OutOfMemoryError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "out of memory")
  }
}

impl From<gfx_hal::device::OutOfMemory> for OutOfMemoryError {
  fn from(_: gfx_hal::device::OutOfMemory) -> Self {
    OutOfMemoryError
  }
}
