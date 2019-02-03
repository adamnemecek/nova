// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::backend::{self, Backend};
use std::ops::Deref;
use std::sync::Arc;

pub use gfx_hal::adapter::PhysicalDevice as PhysicalDeviceExt;
pub use gfx_hal::queue::QueueFamily as QueueFamilyExt;
pub use gfx_hal::queue::RawCommandQueue as QueueExt;
pub use gfx_hal::AdapterInfo;
pub use gfx_hal::Device as DeviceExt;

pub type Adapter = gfx_hal::Adapter<Backend>;
pub type Device = <Backend as gfx_hal::Backend>::Device;
pub type PhysicalDevice = <Backend as gfx_hal::Backend>::PhysicalDevice;
pub type Queue = <Backend as gfx_hal::Backend>::CommandQueue;
pub type QueueFamily = <Backend as gfx_hal::Backend>::QueueFamily;

#[derive(Clone)]
pub struct DeviceHandle(Arc<Inner>);

struct Inner {
  device: Device,
  adapter: Adapter,
  _instance: Arc<backend::Instance>,
}

impl DeviceHandle {
  pub(super) fn new(device: Device, adapter: Adapter, instance: Arc<backend::Instance>) -> Self {
    DeviceHandle(Arc::new(Inner {
      device,
      adapter,
      _instance: instance,
    }))
  }

  pub(crate) fn adapter_info(&self) -> &AdapterInfo {
    &self.0.adapter.info
  }
}

impl Deref for DeviceHandle {
  type Target = Device;

  fn deref(&self) -> &Device {
    &self.0.device
  }
}
