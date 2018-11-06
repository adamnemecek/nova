use super::*;
use gfx_hal::queue::QueueFamilyId;
use std::sync::{Mutex, MutexGuard};

pub struct CommandQueue {
  family_id: QueueFamilyId,
  raw: Mutex<backend::CommandQueue>,
}

impl CommandQueue {
  pub fn new(queues: &mut backend::Queues, family_id: QueueFamilyId) -> Self {
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

  pub fn family_id(&self) -> QueueFamilyId {
    self.family_id
  }

  pub fn raw_mut(&self) -> MutexGuard<backend::CommandQueue> {
    self.raw.lock().unwrap()
  }
}

pub struct CommandQueueSet {
  graphics: CommandQueue,
  transfer: CommandQueue,
}

impl CommandQueueSet {
  pub fn new(mut queues: backend::Queues, families: &CommandQueueFamilies) -> Self {
    CommandQueueSet {
      graphics: CommandQueue::new(&mut queues, families.graphics.id()),
      transfer: CommandQueue::new(&mut queues, families.transfer.id()),
    }
  }

  pub fn graphics(&self) -> &CommandQueue {
    &self.graphics
  }

  pub fn transfer(&self) -> &CommandQueue {
    &self.transfer
  }
}

pub struct CommandQueueFamilies {
  graphics: backend::QueueFamily,
  transfer: backend::QueueFamily,
}

impl CommandQueueFamilies {
  pub fn create_info(&self) -> [(&backend::QueueFamily, &[f32]); 2] {
    [(&self.graphics, &[1.0]), (&self.transfer, &[1.0])]
  }
}

pub fn select_queue_families<'a>(
  adapter: &'a backend::Adapter,
  surface: &'a backend::Surface,
) -> CommandQueueFamilies {
  let graphics = adapter
    .queue_families
    .iter()
    .filter(|family| family.supports_graphics())
    .filter(|family| surface.supports_queue_family(family))
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

  CommandQueueFamilies { graphics, transfer }
}
