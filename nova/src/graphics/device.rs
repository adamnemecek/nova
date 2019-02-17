// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod queues;
mod setup;
mod sync;

use super::backend::{self, Backend};
use std::sync::Arc;

pub use self::queues::*;
pub use self::setup::*;
pub use self::sync::*;
pub use gfx_hal::adapter::{AdapterInfo, DeviceType};

pub(crate) use gfx_hal::Device as RawDeviceExt;
pub(crate) use gfx_hal::PhysicalDevice as RawPhysicalDeviceExt;

pub(crate) type RawDevice = <Backend as gfx_hal::Backend>::Device;
pub(crate) type RawPhysicalDevice = <Backend as gfx_hal::Backend>::PhysicalDevice;

type RawAdapter = gfx_hal::Adapter<Backend>;

#[derive(Clone)]
pub struct Device {
  inner: Arc<Inner>,
}

struct Inner {
  raw: RawDevice,
  adapter: RawAdapter,
  backend: backend::Handle,
}

impl Device {
  pub(crate) fn from_raw(
    backend: &backend::Handle,
    adapter: RawAdapter,
    device: RawDevice,
  ) -> Device {
    Device {
      inner: Arc::new(Inner {
        backend: backend.clone(),
        adapter,
        raw: device,
      }),
    }
  }

  pub(crate) fn raw(&self) -> &RawDevice {
    &self.inner.raw
  }

  pub(crate) fn raw_physical(&self) -> &RawPhysicalDevice {
    &self.inner.adapter.physical_device
  }

  pub(crate) fn adapter_info(&self) -> &AdapterInfo {
    &self.inner.adapter.info
  }

  pub(crate) fn backend(&self) -> &backend::Handle {
    &self.inner.backend
  }
}
