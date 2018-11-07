use super::*;
use smallvec::SmallVec;
use std::sync::Arc;

pub const FRAME_COUNT: usize = 3;

pub struct Renderer {
  device: Arc<Device>,
  frames: Chain<[Frame; FRAME_COUNT]>,
}

struct Frame {
  fence: Fence,
  semaphore: Semaphore,
  commands: SmallVec<[CommandBuffer; 1]>,
}

impl Renderer {
  pub fn new(device: &Arc<Device>) -> Self {
    Renderer {
      device: device.clone(),
      frames: Chain::allocate(|| Frame {
        fence: Fence::new(device),
        semaphore: Semaphore::new(device),
        commands: SmallVec::new(),
      }),
    }
  }

  pub fn render(
    &mut self,
    commands: impl IntoIterator<Item = CommandBuffer>,
    wait_for: &Semaphore,
  ) -> &Semaphore {
    let frame = self.frames.next();

    frame.commands.clear();
    frame.commands.extend(commands);

    let mut queue = self.device.queues.graphics().raw_mut();

    unsafe {
      queue.submit_raw(
        gfx_hal::queue::RawSubmission {
          cmd_buffers: frame.commands.iter().map(CommandBuffer::raw),
          wait_semaphores: &[(
            wait_for.raw(),
            gfx_hal::pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
          )],
          signal_semaphores: &[frame.semaphore.raw()],
        },
        Some(frame.fence.raw()),
      );
    }

    &frame.semaphore
  }
}
