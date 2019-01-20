// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Backend, Instance, InstanceExt};
use gfx_hal::adapter::DeviceType;
use gfx_hal::adapter::PhysicalDevice as PhysicalDeviceExt;
use gfx_hal::queue::QueueFamily as QueueFamilyExt;
use std::ops::Deref;
use std::sync::Arc;

pub use gfx_hal::AdapterInfo;

pub type Adapter = gfx_hal::Adapter<Backend>;
pub type Device = <Backend as gfx_hal::Backend>::Device;
pub type Queue = <Backend as gfx_hal::Backend>::CommandQueue;

pub fn open(instance: &Arc<Instance>) -> (DeviceHandle, Vec<Queue>) {
  // Select the best adapter according to `score_adapter()`.
  let mut adapters = instance.enumerate_adapters();

  adapters.sort_by_key(score_adapter);

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
    // This is not “unsafe” in the traditional sense. gfx_hal enforces some
    // Vulkan requirements with the type system and marks functions that do
    // not use those types as `unsafe`. In this case, it's because the “safe”
    // variant creates queue structures that can only be used to record
    // supported commands. However, by enforcing this with the type system, it
    // imposes too large of an organizational cost on nova's code.
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

  let device = DeviceHandle(Arc::new(Inner {
    instance: instance.clone(),
    adapter,
    device: gpu.device,
  }));

  (device, queues)
}

#[derive(Clone)]
pub struct DeviceHandle(Arc<Inner>);

struct Inner {
  device: Device,
  adapter: Adapter,
  instance: Arc<Instance>,
}

impl DeviceHandle {
  pub fn instance(&self) -> &Instance {
    &self.0.instance
  }

  pub fn adapter_info(&self) -> &AdapterInfo {
    &self.0.adapter.info
  }
}

impl Deref for DeviceHandle {
  type Target = Device;

  fn deref(&self) -> &Device {
    &self.0.device
  }
}

fn score_adapter(adapter: &Adapter) -> usize {
  match adapter.info.device_type {
    DeviceType::DiscreteGpu => 3,
    DeviceType::IntegratedGpu => 2,
    DeviceType::Cpu => 1,
    _ => 0,
  }
}
