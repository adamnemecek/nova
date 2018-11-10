use crate::graphics::backend;
use crate::graphics::device;
use crate::graphics::hal::prelude::*;
use gfx_hal::pool::CommandPoolCreateFlags;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct CommandPool {
  queue: Arc<device::Queue>,
  raw: Option<Mutex<backend::CommandPool>>,
  pub(super) recording: AtomicBool,
}

impl CommandPool {
  pub fn new(queue: &Arc<device::Queue>) -> Arc<CommandPool> {
    let pool = queue
      .device()
      .raw()
      .create_command_pool(queue.family_id(), CommandPoolCreateFlags::TRANSIENT)
      .expect("could not create command pool");

    Arc::new(CommandPool {
      queue: queue.clone(),
      raw: Some(Mutex::new(pool)),
      recording: AtomicBool::new(false),
    })
  }

  pub fn queue(&self) -> &Arc<device::Queue> {
    &self.queue
  }

  pub fn raw_mut(&self) -> MutexGuard<backend::CommandPool> {
    self.raw.as_ref().unwrap().lock().unwrap()
  }
}

impl Drop for CommandPool {
  fn drop(&mut self) {
    if let Some(pool) = self.raw.take() {
      self
        .queue
        .device()
        .raw()
        .destroy_command_pool(pool.into_inner().unwrap());
    }
  }
}
