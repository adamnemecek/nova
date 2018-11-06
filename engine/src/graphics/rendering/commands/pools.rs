use super::*;
use gfx_hal::pool::CommandPoolCreateFlags;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct CommandPool {
  device: Arc<Device>,
  raw: Option<Mutex<backend::CommandPool>>,
  pub(super) recording: AtomicBool,
}

impl CommandPool {
  pub fn new(device: &Arc<Device>, queue: &CommandQueue) -> Arc<CommandPool> {
    let pool = device
      .raw
      .create_command_pool(queue.family_id(), CommandPoolCreateFlags::TRANSIENT);

    Arc::new(CommandPool {
      device: device.clone(),
      raw: Some(Mutex::new(pool)),
      recording: AtomicBool::new(false),
    })
  }

  pub fn raw_mut(&self) -> MutexGuard<backend::CommandPool> {
    self.raw.as_ref().unwrap().lock().unwrap()
  }
}

impl Drop for CommandPool {
  fn drop(&mut self) {
    if let Some(pool) = self.raw.take() {
      self
        .device
        .raw
        .destroy_command_pool(pool.into_inner().unwrap());
    }
  }
}
