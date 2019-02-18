// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::backend::{self, Backend, InstanceExt};
use crate::log;
use gfx_hal::adapter::DeviceType;

pub use gfx_hal::queue::QueueFamily as QueueFamilyExt;
pub use gfx_hal::queue::RawCommandQueue as QueueExt;
pub use gfx_hal::Device as DeviceExt;
pub use gfx_hal::PhysicalDevice as PhysicalDeviceExt;

pub type Device = <Backend as gfx_hal::Backend>::Device;
pub type PhysicalDevice = <Backend as gfx_hal::Backend>::PhysicalDevice;
pub type Queue = <Backend as gfx_hal::Backend>::CommandQueue;
pub type QueueFamily = <Backend as gfx_hal::Backend>::QueueFamily;

type Adapter = gfx_hal::Adapter<Backend>;

pub struct Gpu {
  queue_families: Vec<QueueFamily>,
  queues: Vec<Queue>,
  device: Device,
  adapter: Adapter,
  backend: backend::Instance,
}

impl Default for Gpu {
  fn default() -> Self {
    Gpu::new()
  }
}

impl Gpu {
  pub fn new() -> Self {
    let log = log::Logger::new(format!("{}::Gpu", module_path!()));

    // Instantiate the backend.
    let backend = backend::Instance::create("nova", 1);

    log
      .info("Instantiated backend.")
      .with("backend", backend::NAME);

    // Select the best adapter according to a score, which for now is just a
    // ranking of GPU types.
    let mut adapters = backend.enumerate_adapters();

    adapters.sort_by_key(|adapter| match adapter.info.device_type {
      DeviceType::DiscreteGpu => 3,
      DeviceType::IntegratedGpu => 2,
      DeviceType::Cpu => 1,
      _ => 0,
    });

    let adapter = adapters
      .into_iter()
      .next()
      // TODO: Error handling (doesn't gfx_hal panic when no devices are found?)
      .expect("Could not create graphics device: no adapters found.");

    let queue_families = adapter.queue_families.clone();

    // Open one queue in every family.
    let queue_requests = queue_families
      .iter()
      .map(|family| (family, &[1.0][..]))
      .collect::<Vec<_>>();

    // Open the physical device.
    let mut gpu = unsafe {
      adapter
        .physical_device
        .open(&queue_requests[..])
        // TODO: Error handling.
        .expect("Could not create graphics device")
    };

    // Collect all opened queues.
    let queues = queue_families
      .iter()
      .map(|f| {
        gpu
          .queues
          .take_raw(f.id())
          .expect("Adapter did not open all requested queue groups.")
          .into_iter()
          .next()
          .expect("Adapter did not open a queue for one or more requested queue groups.")
      })
      .collect::<Vec<_>>();

    log
      .info("Opened device.")
      .with("adapter_info", &adapter.info)
      .with("queue_count", queues.len());

    Gpu {
      backend,
      adapter,
      device: gpu.device,
      queues,
      queue_families,
    }
  }

  pub fn backend(&self) -> &backend::Instance {
    &self.backend
  }

  pub fn device(&self) -> &Device {
    &self.device
  }

  pub fn physical_device(&self) -> &PhysicalDevice {
    &self.adapter.physical_device
  }

  pub fn queues(&self) -> &[Queue] {
    &self.queues
  }

  pub fn queue_mut(&mut self, index: usize) -> &mut Queue {
    &mut self.queues[index]
  }

  pub fn queue_families(&self) -> &[QueueFamily] {
    &self.queue_families
  }
}
