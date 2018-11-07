use super::*;
use smallvec::SmallVec;
use std::sync::Arc;

pub const FRAME_COUNT: usize = 3;

pub struct Renderer {
  device: Arc<Device>,
  pass: Arc<RenderPass>,
  command_pool: Arc<CommandPool>,
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
      pass: RenderPass::new(device),
      command_pool: CommandPool::new(device, device.queues.graphics()),
      frames: Chain::allocate(|| Frame {
        fence: Fence::new(device),
        semaphore: Semaphore::new(device),
        commands: SmallVec::new(),
      }),
    }
  }

  pub fn pass(&self) -> &Arc<RenderPass> {
    &self.pass
  }

  pub fn render(
    &mut self,
    framebuffer: &Framebuffer,
    wait_for: &Semaphore,
    commands: impl IntoIterator<Item = CommandBuffer>,
  ) -> &Semaphore {
    let mut primary = CommandBuffer::new(&self.command_pool, CommandBufferKind::Primary);

    primary.begin();
    primary.begin_pass(self.pass(), &framebuffer);
    primary.execute_commands(commands);
    primary.finish();

    let frame = self.frames.next();

    frame.fence.wait();

    frame.commands.clear();
    frame.commands.push(primary);

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
