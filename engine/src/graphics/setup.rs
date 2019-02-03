// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::backend::{self, InstanceExt};
use super::device::{DeviceHandle, PhysicalDeviceExt, QueueFamilyExt};
use super::Gpu;
use gfx_hal::adapter::DeviceType;
use std::sync::Arc;

pub fn setup() -> Gpu {
  let instance = Arc::new(backend::Instance::create("nova", 1));

  // Select the best adapter according to `score_adapter()`.
  let mut adapters = instance.enumerate_adapters();

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

  // Open one queue in every family.
  let queue_requests = adapter
    .queue_families
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

  let queues = adapter
    .queue_families
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
    .collect();

  let device = DeviceHandle::new(gpu.device, adapter, instance.clone());

  Gpu {
    device,
    _queues: queues,
  }
}
