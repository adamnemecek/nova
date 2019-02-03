// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) mod backend;
pub mod device;

mod setup;

pub use self::device::{Device, DeviceExt, DeviceHandle};
pub use self::setup::setup;

pub struct Gpu {
  _queues: Vec<device::Queue>,
  device: DeviceHandle,
}

impl Gpu {
  pub(crate) fn device(&self) -> &DeviceHandle {
    &self.device
  }
}
