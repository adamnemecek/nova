use super::backend;
use super::prelude::*;
use super::*;

pub struct Semaphore {
  device: Arc<Device>,
  raw: Option<backend::Semaphore>,
}

impl Semaphore {
  pub fn new(device: &Arc<Device>) -> Self {
    Semaphore {
      raw: Some(device.raw.create_semaphore()),
      device: device.clone(),
    }
  }

  pub fn raw(&self) -> &backend::Semaphore {
    self.raw.as_ref().unwrap()
  }
}

impl Drop for Semaphore {
  fn drop(&mut self) {
    self.device.raw.destroy_semaphore(self.raw.take().unwrap());
  }
}
