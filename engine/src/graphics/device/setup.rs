// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Device, DeviceType, Queues, RawPhysicalDeviceExt};
use crate::ecs;
use crate::graphics::backend::{self, InstanceExt};
use crate::log;

pub fn setup(res: &mut ecs::Resources) {
  if res.has_value::<backend::Handle>() {
    return;
  }

  let log = log::Logger::new(module_path!());

  // Instantiate the backend.
  let backend = backend::Handle::from(backend::Instance::create("nova", 1));

  log
    .info("Instantiated backend.")
    .with("backend", backend::NAME);

  // Select the best adapter according to `score_adapter()`.
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
  let gpu = unsafe {
    adapter
      .physical_device
      .open(&queue_requests[..])
      // TODO: Error handling.
      .expect("Could not create graphics device")
  };

  let device = Device::from_raw(&backend, adapter, gpu.device);
  let queues = Queues::from_raw(&device, gpu.queues, queue_families);

  log
    .info("Opened device.")
    .with("adapter_info", device.adapter_info())
    .with("queue_count", queues.count());

  res.insert(backend);
  res.insert(device);
  res.insert(queues);
}
