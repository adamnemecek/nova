pub use gfx_hal::queue::QueueFamilyId;
pub use gfx_hal::queue::RawSubmission;

use super::Device;
use crate::graphics::backend;
use crate::graphics::hal::prelude::*;
use crate::graphics::window::Swapchain;
use crate::graphics::Semaphore;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct Queue {
  family: backend::QueueFamily,
  raw: Mutex<backend::CommandQueue>,
  device: Arc<Device>,
}

impl Queue {
  pub fn new(
    device: &Arc<Device>,
    queues: &mut backend::Queues,
    family: backend::QueueFamily,
  ) -> Self {
    let queue = queues
      .take_raw(family.id())
      .expect("queue family not found")
      .into_iter()
      .next()
      .expect("queue not found");

    Queue {
      family,
      raw: Mutex::new(queue),
      device: device.clone(),
    }
  }

  pub fn device(&self) -> &Arc<Device> {
    &self.device
  }

  pub fn family_id(&self) -> QueueFamilyId {
    self.family.id()
  }

  pub fn present<'a>(
    &self,
    images: impl IntoIterator<Item = (&'a Swapchain, u32)>,
    wait_for: impl IntoIterator<Item = &'a Semaphore>,
  ) -> Result<(), ()> {
    self.raw_mut().present(
      images.into_iter().map(|(sc, i)| (sc.raw(), i)),
      wait_for.into_iter().map(Semaphore::raw),
    )
  }

  pub fn raw_mut(&self) -> MutexGuard<backend::CommandQueue> {
    self.raw.lock().unwrap()
  }
}

pub trait QueueSet {
  fn select_families(adapter: &backend::Adapter) -> Vec<backend::QueueFamily>;

  fn from_raw(
    device: &Arc<Device>,
    queues: &mut backend::Queues,
    families: Vec<backend::QueueFamily>,
  ) -> Self;
}

pub struct DefaultQueueSet {
  pub graphics: Arc<Queue>,
  pub transfer: Arc<Queue>,
}

impl QueueSet for DefaultQueueSet {
  fn select_families(adapter: &backend::Adapter) -> Vec<backend::QueueFamily> {
    let graphics = adapter
      .queue_families
      .iter()
      .filter(|family| family.supports_graphics())
      .next()
      .expect("no graphics queue family")
      .clone();

    let transfer = adapter
      .queue_families
      .iter()
      .filter(|family| !family.supports_graphics())
      .next()
      .expect("no transfer queue family")
      .clone();

    vec![graphics, transfer]
  }

  fn from_raw(
    device: &Arc<Device>,
    queues: &mut backend::Queues,
    mut families: Vec<backend::QueueFamily>,
  ) -> Self {
    let transfer = families.pop().unwrap();
    let graphics = families.pop().unwrap();

    DefaultQueueSet {
      graphics: Arc::new(Queue::new(device, queues, graphics)),
      transfer: Arc::new(Queue::new(device, queues, transfer)),
    }
  }
}
