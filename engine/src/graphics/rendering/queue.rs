use super::backend;
use std::sync::{Mutex, MutexGuard};

pub struct CommandQueue {
  family_id: gfx_hal::queue::QueueFamilyId,
  raw: Mutex<backend::CommandQueue>,
}

impl CommandQueue {
  pub fn take(queues: &mut backend::Queues, family_id: gfx_hal::queue::QueueFamilyId) -> Self {
    let queue = queues
      .take_raw(family_id)
      .expect("queue family not found")
      .into_iter()
      .next()
      .expect("queue not found");

    CommandQueue {
      family_id,
      raw: Mutex::new(queue),
    }
  }

  pub fn family_id(&self) -> gfx_hal::queue::QueueFamilyId {
    self.family_id
  }

  pub fn raw_mut(&self) -> MutexGuard<backend::CommandQueue> {
    self.raw.lock().unwrap()
  }
}
