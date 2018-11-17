use super::device;
use super::hal::prelude::*;
use super::pipeline;
use super::window::swapchain::{self, Swapchain};
use super::window::{Surface, Window};
use super::{Commands, Fence, Framebuffer, RenderPass, Semaphore};
use nova::utils::{Droppable, Ring};
use std::iter;
use std::sync::Arc;

pub struct Renderer {
  queue_family_id: usize,
  surface: Surface,
  render_pass: Arc<RenderPass>,
  fence: Fence,
  semaphores: Ring<(Semaphore, Semaphore)>,
  swapchain: Droppable<Swapchain>,
  commands: Vec<Commands>,
  framebuffers: Vec<Arc<Framebuffer>>,
  frame: usize,
  log: bflog::Logger,
}

impl Renderer {
  pub fn new(queue: &device::Queue, window: &mut Window, log: &bflog::Logger) -> Self {
    let device = queue.device();

    let surface = window.take_surface();

    let render_pass = RenderPass::new(device);

    let semaphores = Ring::new(3, |_| (Semaphore::new(&device), Semaphore::new(&device)));

    Renderer {
      queue_family_id: queue.family_id(),
      surface,
      render_pass,
      fence: Fence::new(&device),
      semaphores,
      commands: Vec::new(),
      swapchain: Droppable::dropped(),
      framebuffers: Vec::new(),
      frame: 0,
      log: log.with_src("game::graphics::Renderer"),
    }
  }

  pub fn render_pass(&self) -> &Arc<RenderPass> {
    &self.render_pass
  }

  pub fn begin_frame(&mut self, window: &mut Window) -> Arc<Framebuffer> {
    self.semaphores.next();

    for _ in 0..5 {
      self.ensure_swapchain(window);

      match self.swapchain.acquire_image(&self.semaphores.current().0) {
        Ok(index) => {
          self.frame = index;

          return self.framebuffers[index].clone();
        }

        Err(swapchain::AcquireImageError::OutOfDate) => {
          self.log.trace("Swapchain out of date.");
          self.destroy_swapchain();
        }
      }
    }

    panic!("Swapchain was repeatedly out of date.");
  }

  pub fn submit_frame(
    &mut self,
    queue: &mut device::Queue,
    commands: impl IntoIterator<Item = Commands>,
  ) {
    assert!(
      queue.family_id() == self.queue_family_id,
      "Frames must be submitted with queue family {}.",
      self.queue_family_id
    );

    if self.fence.is_signaled() {
      self.fence.reset();
    } else {
      self.fence.wait_and_reset();
    }

    self.commands.clear();
    self.commands.extend(commands);

    let (acquire_semaphore, render_semaphore) = self.semaphores.current();

    unsafe {
      queue.raw_mut().submit_raw(
        device::queue::RawSubmission {
          cmd_buffers: self.commands.iter().map(AsRef::as_ref),
          wait_semaphores: &[(
            acquire_semaphore.as_ref(),
            pipeline::Stage::COLOR_ATTACHMENT_OUTPUT,
          )],
          signal_semaphores: &[render_semaphore.as_ref()],
        },
        Some(self.fence.as_ref()),
      );
    }

    let result = queue.present(
      iter::once((self.swapchain.as_ref(), self.frame as u32)),
      iter::once(render_semaphore),
    );

    if result.is_err() {
      self.log.trace("Presentation failed.");
      self.destroy_swapchain();
    }
  }

  fn ensure_swapchain(&mut self, window: &mut Window) {
    if !self.swapchain.is_dropped() {
      return;
    }

    self.swapchain =
      Swapchain::new(self.render_pass.device(), &mut self.surface, window.size()).into();

    self.log.trace("Created swapchain.").with(
      "present_mode",
      &format!("{:#?}", self.swapchain.present_mode()),
    );

    for image in self.swapchain.images() {
      self.framebuffers.push(Arc::new(Framebuffer::new(
        self.render_pass(),
        iter::once(image.clone()),
      )));
    }

    self
      .log
      .trace("Created framebuffers.")
      .with("count", &self.framebuffers.len())
      .with("width", &self.framebuffers[0].size().width())
      .with("height", &self.framebuffers[0].size().height());
  }

  fn destroy_swapchain(&mut self) {
    if self.swapchain.is_dropped() {
      return;
    }

    self.render_pass.device().wait_idle();

    self.framebuffers.clear();
    self.frame = 0;

    self.swapchain.drop();

    self.log.trace("Destroyed swapchain.");
  }
}

// Implement `Drop` to destroy the swapchain because this must happen before the
// surface is dropped and destroyed.
//
// TODO: Figure out how to ensure that a surface is not destroyed until after
//       the swapchain is destroyed.
impl Drop for Renderer {
  fn drop(&mut self) {
    self.destroy_swapchain();
  }
}
