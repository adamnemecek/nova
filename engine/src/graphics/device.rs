// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::backend::{self, Backend};
use std::sync::Arc;

pub use gfx_hal::AdapterInfo;
pub(crate) use gfx_hal::Device as RawDeviceExt;

pub type DeviceHandle = Arc<Device>;
pub(crate) type RawDevice = <Backend as gfx_hal::Backend>::Device;
pub(crate) type RawPhysicalDevice = <Backend as gfx_hal::Backend>::PhysicalDevice;

type RawAdapter = gfx_hal::Adapter<Backend>;

pub struct Device {
  raw: RawDevice,
  adapter: RawAdapter,
  backend: backend::Handle,
}

impl Device {
  pub(super) fn from_raw(
    backend: &backend::Handle,
    adapter: RawAdapter,
    device: RawDevice,
  ) -> DeviceHandle {
    DeviceHandle::new(Device {
      backend: backend.clone(),
      adapter,
      raw: device,
    })
  }

  pub(crate) fn raw(&self) -> &RawDevice {
    &self.raw
  }

  pub(crate) fn raw_physical(&self) -> &RawPhysicalDevice {
    &self.adapter.physical_device
  }

  pub(crate) fn adapter_info(&self) -> &AdapterInfo {
    &self.adapter.info
  }

  pub(crate) fn backend(&self) -> &backend::Handle {
    &self.backend
  }
}
