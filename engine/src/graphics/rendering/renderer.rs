use super::*;
use smallvec::SmallVec;
use std::iter;
use std::sync::Arc;

pub const FRAME_COUNT: usize = 3;

pub struct Renderer {
  device: Arc<Device>,
  fence: Option<backend::Fence>,
  semaphore: Semaphore,
  commands: Vec<CommandBuffer>,
}

impl Renderer {
  pub fn new(device: &Arc<Device>) -> Self {
    let fence = device.raw.create_fence(true);
    let semaphore = Semaphore::new(device);
    let commands = Vec::new();

    Renderer {
      device: device.clone(),
      fence: Some(fence),
      semaphore,
      commands,
    }
  }

  pub fn semaphore(&self) -> &Semaphore {
    &self.semaphore
  }

  pub fn wait_ready(&self) {
    self
      .device
      .raw
      .wait_for_fence(self.fence.as_ref().unwrap(), !0);
  }

  pub fn render(
    &mut self,
    commands: impl IntoIterator<Item = CommandBuffer>,
    wait_for: &Semaphore,
  ) {
    self.commands.clear();
    self.commands.extend(commands);

    let mut queue = self.device.queues.graphics().raw_mut();

    unsafe {
      queue.submit_raw(
        gfx_hal::queue::RawSubmission {
          cmd_buffers: self.commands.iter().map(CommandBuffer::raw),
          wait_semaphores: &[(
            wait_for.raw(),
            gfx_hal::pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
          )],
          signal_semaphores: &[self.semaphore.raw()],
        },
        self.fence.as_ref(),
      );
    }
  }
}

impl Drop for Renderer {
  fn drop(&mut self) {
    self.device.raw.destroy_fence(self.fence.take().unwrap());
  }
}
