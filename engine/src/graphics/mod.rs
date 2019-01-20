// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod backend;
mod device;

use self::backend::*;
use crate::ecs;
use crate::log;
use std::sync::Arc;

pub use self::device::{Adapter, Device, DeviceHandle};

pub fn setup(res: &mut ecs::Resources) {
  res.entry().or_insert_with(|| {
    let log = log::Logger::new("nova::graphics");
    let instance = Arc::new(Instance::create("nova", 1));

    log
      .info("Instantiated backend.")
      .with("backend", BACKEND_NAME);

    let device = device::open(&instance);

    log
      .info("Opened device.")
      .with("adapter", device.adapter_info());

    Graphics { instance, device }
  });
}

pub struct Graphics {
  device: DeviceHandle,
  instance: Arc<Instance>,
}

impl Graphics {
  pub fn device(&self) -> &DeviceHandle {
    &self.device
  }
}
