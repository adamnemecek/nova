use super::device;
use super::hal::prelude::*;
use super::pipeline;
use super::window::swapchain::{self, Swapchain};
use super::window::Window;
use super::{Commands, Fence, Framebuffer, RenderPass, Semaphore};
use nova::math::Size;
use nova::utils::Droppable;
use std::iter;
use std::sync::Arc;

pub struct Renderer {
  queue_family_id: usize,
  pass: Arc<RenderPass>,
  fence: Fence,
  semaphores: (Semaphore, Semaphore),
  swapchain: Droppable<Swapchain>,
  commands: Vec<Commands>,
  framebuffers: Vec<Arc<Framebuffer>>,
  frame: usize,
  size: Size<u32>,
  log: bflog::Logger,
}

impl Renderer {
  pub fn new(queue: &device::Queue, window: &Window, log: &bflog::Logger) -> Self {
    let device = queue.device();

    // Create the render pass, which currently defaults to a single image
    // attachment for a swapchain backbuffer.
    let pass = RenderPass::new(device);

    Renderer {
      queue_family_id: queue.family_id(),
      pass,
      fence: Fence::new(&device),
      semaphores: (Semaphore::new(&device), Semaphore::new(&device)),
      commands: Vec::new(),
      swapchain: Droppable::dropped(),
      framebuffers: Vec::new(),
      frame: 0,
      size: window.size(),
      log: log.with_src("game::graphics::Renderer"),
    }
  }

  pub fn pass(&self) -> &Arc<RenderPass> {
    &self.pass
  }

  pub fn begin_frame(&mut self, window: &mut Window) -> Arc<Framebuffer> {
    for _ in 0..5 {
      self.ensure_swapchain(window);

      let (ref semaphore, _) = self.semaphores;

      match self.swapchain.acquire_image(semaphore) {
        Ok(index) => {
          self.frame = index;

          return self.framebuffers[index].clone();
        }

        Err(swapchain::AcquireImageError::OutOfDate) => self.destroy_swapchain(),
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

    let (ref acquire_semaphore, ref render_semaphore) = self.semaphores;

    unsafe {
      queue.raw_mut().submit_raw(
        device::queue::RawSubmission {
          cmd_buffers: self.commands.iter().map(AsRef::as_ref),
          wait_semaphores: &[(
            acquire_semaphore.raw(),
            pipeline::Stage::COLOR_ATTACHMENT_OUTPUT,
          )],
          signal_semaphores: &[render_semaphore.raw()],
        },
        Some(self.fence.as_ref()),
      );
    }

    let result = queue.present(
      iter::once((self.swapchain.as_ref(), self.frame as u32)),
      iter::once(render_semaphore),
    );

    if result.is_err() {
      self.destroy_swapchain();
    }
  }

  fn ensure_swapchain(&mut self, window: &mut Window) {
    let size = window.size();

    if size != self.size {
      self.size = size;
      self.swapchain.drop();
    }

    if !self.swapchain.is_dropped() {
      return;
    }

    self.swapchain = Swapchain::new(&self.pass, window.surface_mut().as_mut(), size).into();

    self.log.trace("Created swapchain.").with(
      "present_mode",
      &format!("{:#?}", self.swapchain.present_mode()),
    );

    for image in self.swapchain.images() {
      self.framebuffers.push(Arc::new(Framebuffer::new(
        self.pass(),
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
    self.framebuffers.clear();
    self.frame = 0;

    self.swapchain.drop();

    self.log.trace("Destroyed swapchain.");
  }
}
