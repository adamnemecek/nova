use super::backend;
use super::prelude::*;
use super::*;
use smallvec::{Array, SmallVec};
use std::iter;

pub struct Semaphore {
  device: Arc<Device>,
  raw: Option<backend::Semaphore>,
}

impl Semaphore {
  pub fn new(device: &Arc<Device>) -> Self {
    let semaphore = device
      .raw
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
    self.device.raw.destroy_semaphore(self.raw.take().unwrap());
  }
}

pub struct Fence {
  device: Arc<Device>,
  raw: Option<backend::Fence>,
}

impl Fence {
  pub fn new(device: &Arc<Device>) -> Self {
    let fence = device
      .raw
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
      .raw
      .wait_for_fence(self.raw(), !0)
      .expect("could not wait for fence");
  }
}

impl Drop for Fence {
  fn drop(&mut self) {
    self.device.raw.destroy_fence(self.raw.take().unwrap());
  }
}

pub struct Chain<A: Array> {
  items: SmallVec<A>,
  index: usize,
}

impl<A: Array> Chain<A> {
  pub fn allocate(create: impl FnMut() -> A::Item) -> Self {
    Chain {
      items: iter::repeat_with(create).take(A::size()).collect(),
      index: 0,
    }
  }

  pub fn next(&mut self) -> &mut A::Item {
    let index = self.index;

    self.index += 1;
    self.index %= self.items.len();

    &mut self.items[index]
  }
}
