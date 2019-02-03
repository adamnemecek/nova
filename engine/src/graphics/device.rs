// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::backend::{self, Backend};
use std::sync::Arc;

pub use gfx_hal::AdapterInfo;

pub type DeviceHandle = Arc<Device>;

type RawAdapter = gfx_hal::Adapter<Backend>;
type RawDevice = <Backend as gfx_hal::Backend>::Device;

pub struct Device {
  _raw: RawDevice,
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
      _raw: device,
    })
  }

  pub(crate) fn adapter_info(&self) -> &AdapterInfo {
    &self.adapter.info
  }

  pub(crate) fn backend(&self) -> &backend::Handle {
    &self.backend
  }
}
