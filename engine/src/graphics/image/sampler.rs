// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::device::DeviceHandle;
use crate::graphics::prelude::*;

pub struct Sampler {
  raw: Option<backend::Sampler>,
  device: DeviceHandle,
}

impl Sampler {
  pub fn new(device: &DeviceHandle) -> Self {
    let sampler = device
      .raw()
      .create_sampler(hal::image::SamplerInfo::new(
        hal::image::Filter::Linear,
        hal::image::WrapMode::Tile,
      ))
      .expect("could not create sampler");

    Sampler {
      device: device.clone(),
      raw: Some(sampler),
    }
  }

  pub fn raw(&self) -> &backend::Sampler {
    self.raw.as_ref().unwrap()
  }
}

impl Drop for Sampler {
  fn drop(&mut self) {
    if let Some(sampler) = self.raw.take() {
      self.device.raw().destroy_sampler(sampler);
    }
  }
}
