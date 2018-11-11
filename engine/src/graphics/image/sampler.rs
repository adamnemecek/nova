use crate::graphics::backend;
use crate::graphics::hal::prelude::*;
use crate::graphics::Device;
use std::sync::Arc;

pub struct Sampler {
  raw: Option<backend::Sampler>,
  device: Arc<Device>,
}

impl Sampler {
  pub fn new(device: &Arc<Device>) -> Self {
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
