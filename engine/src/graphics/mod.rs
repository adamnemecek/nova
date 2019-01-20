// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod device;

mod backend;

use self::backend::{Backend, Instance, InstanceExt, BACKEND_NAME};
use crate::ecs;
use crate::log;
use std::sync::Arc;

pub use self::device::DeviceHandle;

pub fn setup(res: &mut ecs::Resources) {
  res.entry().or_insert_with(|| {
    let log = log::Logger::new("nova::graphics::setup");
    let instance = Arc::new(Instance::create("nova", 1));

    log
      .info("Instantiated backend.")
      .with("backend", BACKEND_NAME);

    let (device, queues) = device::open(&instance);

    log
      .info("Opened device.")
      .with("adapter", device.adapter_info())
      .with("queues", queues.len());

    Graphics {
      instance,
      device,
      queues,
    }
  });
}

pub struct Graphics {
  queues: Vec<device::Queue>,
  device: DeviceHandle,
  instance: Arc<Instance>,
}

impl Graphics {
  pub fn device(&self) -> &DeviceHandle {
    &self.device
  }
}
