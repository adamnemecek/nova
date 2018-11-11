use super::device;
use super::hal::prelude::*;
use super::pipeline;
use super::window::swapchain::{self, Swapchain};
use super::window::{self, Window};
use super::{Commands, Fence, Framebuffer, RenderPass, Semaphore};
use nova::math::algebra::Vector2;
use nova::utils::{Chain, Droppable};
use std::iter;
use std::sync::Arc;

const FRAMES_IN_FLIGHT: usize = 3;

pub struct Renderer {
  queue_family_id: usize,
  surface: Arc<window::Surface>,
  pass: Arc<RenderPass>,
  fences: Chain<Fence>,
  semaphores: Chain<(Semaphore, Semaphore)>,
  swapchain: Droppable<Swapchain>,
  submissions: Chain<Vec<Commands>>,
  framebuffers: Vec<Arc<Framebuffer>>,
  frame: usize,
  size: Vector2<u32>,
  log: bflog::Logger,
}

impl Renderer {
  pub fn new(queue: &device::Queue, window: &Window, log: &bflog::Logger) -> Self {
    let mut log = log.with_src("game::graphics::Renderer");

    let device = queue.device();

    // Create the render pass, which currently defaults to a single image
    // attachment for a swapchain backbuffer.
    let pass = RenderPass::new(device);

    log.trace("Created render pass.");

    let fences = Chain::allocate(FRAMES_IN_FLIGHT, |_| Fence::new(&device));

    let semaphores = Chain::allocate(FRAMES_IN_FLIGHT, |_| {
      (Semaphore::new(&device), Semaphore::new(&device))
    });

    let submissions = Chain::allocate(FRAMES_IN_FLIGHT, |_| Vec::new());

    log.trace("Created resource chains.");

    Renderer {
      queue_family_id: queue.family_id(),
      surface: window.surface().clone(),
      pass,
      fences,
      semaphores,
      submissions,
      swapchain: Droppable::dropped(),
      framebuffers: Vec::new(),
      frame: 0,
      size: window.size(),
      log,
    }
  }

  pub fn pass(&self) -> &Arc<RenderPass> {
    &self.pass
  }

  pub fn resize(&mut self, size: Vector2<u32>) {
    if size != self.size {
      self.size = size;
      self.swapchain.drop();
    }
  }

  pub fn begin_frame(&mut self) -> Arc<Framebuffer> {
    self.fences.next().wait();
    self.semaphores.next();
    self.submissions.next().clear();

    for _ in 0..5 {
      self.ensure_swapchain();

      let (semaphore, _) = self.semaphores.current();

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

    let fence = self.fences.current();
    let (acquire_semaphore, render_semaphore) = self.semaphores.current();
    let submission = self.submissions.current_mut();

    submission.extend(commands);

    unsafe {
      queue.raw_mut().submit_raw(
        device::queue::RawSubmission {
          cmd_buffers: submission.iter().map(AsRef::as_ref),
          wait_semaphores: &[(
            acquire_semaphore.raw(),
            pipeline::Stage::COLOR_ATTACHMENT_OUTPUT,
          )],
          signal_semaphores: &[render_semaphore.raw()],
        },
        Some(fence.raw()),
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

  fn ensure_swapchain(&mut self) {
    if !self.swapchain.is_dropped() {
      return;
    }

    let size = self.size.map(|c| c as f32);

    self.swapchain = Swapchain::new(&self.pass, &mut self.surface.lock(), size).into();

    self.log.trace("Created swapchain.");

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
      .with("width", &self.framebuffers[0].size().x)
      .with("height", &self.framebuffers[0].size().y);
  }

  fn destroy_swapchain(&mut self) {
    self.framebuffers.clear();
    self.frame = 0;

    self.swapchain.drop();

    self.log.trace("Destroyed swapchain.");
  }
}
