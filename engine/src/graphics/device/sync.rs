use super::Device;
use crate::graphics::hal::*;
use crate::utils::Chain;
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

pub struct Fence {
  device: Arc<Device>,
  raw: Option<backend::Fence>,
}

impl Fence {
  pub fn chain(device: &Arc<Device>, size: usize) -> Chain<Fence> {
    Chain::allocate(size, |_| Fence::new(device))
  }

  pub fn new(device: &Arc<Device>) -> Self {
    let fence = device
      .raw()
      .create_fence(true)
      .expect("could not create fence");

    Fence {
      raw: Some(fence),
      device: device.clone(),
    }
  }

  pub fn raw(&self) -> &backend::Fence {
    self.raw.as_ref().unwrap()
  }

  pub fn wait(&self) {
    self
      .device
      .raw()
      .wait_for_fence(self.raw(), !0)
      .expect("could not wait for fence");
  }
}

impl Drop for Fence {
  fn drop(&mut self) {
    self.device.raw().destroy_fence(self.raw.take().unwrap());
  }
}
