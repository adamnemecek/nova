pub use gfx_hal::queue::RawSubmission;

use super::Device;
use crate::graphics::backend;
use crate::graphics::hal::prelude::*;
use crate::graphics::window::Swapchain;
use crate::graphics::Semaphore;
use std::sync::Arc;

pub struct Queue {
  family: backend::QueueFamily,
  raw: backend::CommandQueue,
  device: Arc<Device>,
}

impl Queue {
  pub unsafe fn from_raw(
    device: &Arc<Device>,
    queues: &mut backend::Queues,
    family: backend::QueueFamily,
  ) -> Self {
    let raw = queues
      .take_raw(family.id())
      .expect("Expected device queue family was missing.")
      .into_iter()
      .next()
      .expect("Expected device queue was missing.");

    Queue {
      family,
      raw,
      device: device.clone(),
    }
  }

  pub fn device(&self) -> &Arc<Device> {
    &self.device
  }

  pub fn family_id(&self) -> usize {
    self.family.id().0
  }

  pub fn present<'a>(
    &mut self,
    images: impl IntoIterator<Item = (&'a Swapchain, u32)>,
    wait_for: impl IntoIterator<Item = &'a Semaphore>,
  ) -> Result<(), ()> {
    self.raw.present(
      images.into_iter().map(|(sc, i)| (sc.raw(), i)),
      wait_for.into_iter().map(Semaphore::raw),
    )
  }

  pub fn raw_mut(&mut self) -> &mut backend::CommandQueue {
    &mut self.raw
  }
}
