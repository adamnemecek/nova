// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Device;
use crate::graphics::backend;
use crate::graphics::hal::prelude::*;
use std::sync::Arc;

pub struct Semaphore {
  device: Arc<Device>,
  raw: Option<backend::Semaphore>,
}

impl Semaphore {
  pub fn new(device: &Arc<Device>) -> Self {
    let semaphore = device
      .raw()
      .create_semaphore()
      .expect("could not create semaphore");

    Semaphore {
      raw: Some(semaphore),
      device: device.clone(),
    }
  }

  pub fn raw(&self) -> &backend::Semaphore {
    self.raw.as_ref().unwrap()
  }
}

impl Drop for Semaphore {
  fn drop(&mut self) {
    self
      .device
      .raw()
      .destroy_semaphore(self.raw.take().unwrap());
  }
}
