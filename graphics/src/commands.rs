use super::backend;
use super::Context;
use gfx_hal::pool::CommandPoolCreateFlags as CreateFlags;
use gfx_hal::Device;
use std::sync::Arc;

pub struct CommandPool {
  context: Arc<Context>,
  pool: Option<backend::CommandPool>,
}

impl CommandPool {
  pub fn new(context: &Arc<Context>, family: gfx_hal::queue::QueueFamilyId) -> Self {
    let pool = context.device().create_command_pool(
      family,
      CreateFlags::TRANSIENT | CreateFlags::RESET_INDIVIDUAL,
    );

    CommandPool {
      context: context.clone(),
      pool: Some(pool),
    }
  }
}

impl Drop for CommandPool {
  fn drop(&mut self) {
    if let Some(pool) = self.pool.take() {
      self.context.device().destroy_command_pool(pool);
    }
  }
}
