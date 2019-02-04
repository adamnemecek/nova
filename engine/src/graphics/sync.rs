// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Backend, Device, RawDeviceExt};
use crate::utils::Droppable;

type RawSemaphore = <Backend as gfx_hal::Backend>::Semaphore;

pub struct Semaphore {
  raw: Droppable<RawSemaphore>,
  device: Device,
}

impl Semaphore {
  pub fn new(device: &Device) -> Semaphore {
    let raw = device
      .raw()
      .create_semaphore()
      .expect("Could not create semaphore");

    Semaphore {
      device: device.clone(),
      raw: raw.into(),
    }
  }

  pub(crate) fn raw(&self) -> &RawSemaphore {
    &self.raw
  }
}

impl Drop for Semaphore {
  fn drop(&mut self) {
    if let Some(raw) = self.raw.take() {
      unsafe {
        self.device.raw().destroy_semaphore(raw);
      }
    }
  }
}
